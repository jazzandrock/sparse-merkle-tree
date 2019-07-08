use std::clone::Clone;
use sha2::Sha256;
use super::storage::{LocalKeyValueStore, KeyValueStore};
use super::hash_convenient::HashConvenient;
use super::utils::log2_64;

pub type IndexT = usize;
pub type SizeT = usize;

pub struct MerkleTree {
    height: i32,
    curr: SizeT,
    hasher: Box<Sha256>,
    empty_hashes: Vec<HashConvenient>,
    hash_cache: Vec<TreeNode>,
    sibling_cache: Vec<TreeNode>,
    store: LocalKeyValueStore,
    values: Vec<Vec<u8>>
}

impl MerkleTree {
    /// initialize empty tree.
    pub fn new(height: i32) -> Self {
        assert!(height > 1);

        let mut hasher = Sha256::default();
        let mut empty_hashes = Vec::<HashConvenient>::new();

        empty_hashes.push(HashConvenient::zero_hash());

        for _ in 1..height as usize {
            let bytes = empty_hashes.last().unwrap().bytes_borrow();
            let new_hash = HashConvenient::hash_two_inputs(&mut hasher, bytes, bytes);
            empty_hashes.push(new_hash);
        }
 
        let mut hash_cache = Vec::<TreeNode>::new();
        for (i, hash) in empty_hashes.iter().rev().enumerate() {
            let node = TreeNode {
                index: (1 as IndexT) << i, 
                hash: hash.clone()
            };
            hash_cache.push(node);
        }

        hash_cache.reverse();

        let mut sibling_cache = Vec::<TreeNode>::new();
        for _ in 0..height {
            sibling_cache.push(TreeNode::dummy());
        }

        let tree = MerkleTree {
            height: height,
            curr: first_node_idx(height),
            hasher: Box::new(hasher),
            empty_hashes: empty_hashes,
            hash_cache: hash_cache,
            sibling_cache: sibling_cache,
            store: LocalKeyValueStore::new(),
            values: Vec::<Vec<u8>>::new()
        };
        tree
    }

    /// get the root of the tree, needed for
    /// a proof.
    pub fn root(&mut self) -> Vec<u8> {
        self.get_node(1)
    }

    /// gives the value at an index
    /// along with everything one needs
    /// to verify it.
    pub fn get_value_and_proof(&mut self, idx: IndexT) -> (Vec<u8>, Proof) {
        if self.values.len() <= idx {
            panic!(format!("there's no value at index {}", idx));
        }
        let val = self.values[idx as usize].to_vec();
        let hashes = self.intermediary_hashes(idx);
        let n = idx + first_node_idx(self.height);

        let proof = Proof {
            n: n, 
            hashes: hashes
        };

        (val, proof)
    }

    /// appends a piece of data you want everybody to remember
    pub fn append(&mut self, data: &[u8]) -> IndexT {
        assert!(self.capacity() > 0);

        let mut curr_hash = HashConvenient::hash_bytes(&mut self.hasher, data);

        let mut n = self.curr;
        let mut i = 0;
        while n > 1 {
            let e = &mut self.hash_cache[i];

            if e.index != n {
                e.persistent_save(&mut self.store);
                self.sibling_cache[i] = e.clone();
            }

            *e = TreeNode{
                index: n,
                hash: curr_hash.clone()
            };

            let sibling_hash = if n & 1 == 0 {
                &self.empty_hashes[i]
            } else {
                &self.sibling_cache[i].hash
            };

            curr_hash = HashConvenient::hash_from_sibling_in_order(
                &mut self.hasher, curr_hash.bytes_borrow(), 
                sibling_hash.bytes_borrow(), n);

            n >>= 1;
            i += 1;
        }

        // update the tree root
        self.hash_cache[i] = TreeNode{
            index: n,
            hash: curr_hash.clone()
        };

        self.values.push(data.to_vec());

        let tree_key = self.curr - first_node_idx(self.height);
        self.curr += 1;

        tree_key
    }
    
    /// We only write a hash of a node to the database
    /// when the hash of that node is final.
    /// 
    /// After you finish writing to 
    /// the tree, you must call this method
    /// to persist everything not yet saved to the database.
    pub fn save_state(&mut self) {
        for e in self.hash_cache.iter() {
            e.persistent_save(&mut self.store);
        }
    }

    pub fn capacity(&self) -> SizeT {
        ((1 as IndexT) << self.height) - self.curr
    }

    fn get_node(&self, n: IndexT) -> Vec<u8> {
        let log = log2_64(n as u64);
        let idx = (self.height as usize) - log - 1;

        let node = if self.hash_cache[idx].index == n {
            self.hash_cache[idx].hash.bytes_borrow().to_vec()
        } else if let Some(val) = self.store.get(n) {
            val.to_vec()
        } else {
            self.empty_hashes[idx].bytes_borrow().to_vec()
        };

        node
    }

    /// gives every needed node, except
    /// the root - the verifier will have 
    /// to get the root from third party anyway.
    fn intermediary_hashes(&mut self, idx: IndexT) -> Vec<Vec<u8>> {
        let mut res = Vec::<Vec<u8>>::new();

        let mut n = idx + first_node_idx(self.height);
        while n > 1 {
            res.push(self.get_node(n ^ 1));
            n >>= 1;
        }
        
        res
    }
}

fn first_node_idx(height: i32) -> SizeT {
    (1 as SizeT) << (height - 1)
}

struct TreeNode {
    index: IndexT,
    hash: HashConvenient
}

impl Clone for TreeNode {
    fn clone(&self) -> TreeNode {
        TreeNode {
            index: self.index,
            hash: self.hash.clone()
        }
    }
}

impl TreeNode {
    fn dummy() -> TreeNode {
        TreeNode {
            index: 0,
            hash: HashConvenient::zero_hash()
        }
    }

    fn persistent_save(&self, store: &mut KeyValueStore) {
        store.put(self.index, self.hash.bytes_borrow());
        println!("persistently saved node {}, {}", self.index, self.hash.to_string());
    }
}

pub struct Proof {
    n: IndexT, 
    hashes: Vec<Vec<u8>>
}

impl Proof {
    /// a function you can use to know if 
    /// MerkleTree::get_value_and_proof 
    /// gave you indeed the data from the tree.
    pub fn check(&self, hasher: &mut Sha256, data: &Vec<u8>,
        root: &Vec<u8>) -> bool {
        let mut hash = HashConvenient::hash_bytes(hasher, &data);

        let mut n = self.n;
        for sibling in self.hashes.iter() {
            hash = HashConvenient::hash_from_sibling_in_order(
                hasher, hash.bytes_borrow(), sibling, n);

            n >>= 1;
        }

        let correct = hash.bytes_borrow() == &root[..];
        correct
    }
}
