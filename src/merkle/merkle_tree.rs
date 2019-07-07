use std::clone::Clone;
use hex;
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
    store: LocalKeyValueStore
}

impl MerkleTree {
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
            store: LocalKeyValueStore::new()
        };
        tree
    }

    fn get_node(&self, n: IndexT) -> Vec<u8> {
        let log = log2_64(n as u64);
        let idx = (self.height as usize) - log - 1;

        if self.hash_cache[idx].index == n {
            self.hash_cache[idx].hash.bytes_borrow().to_vec()
        } else if let Some(val) = self.store.get(n) {
            val.to_vec()
        } else {
            self.empty_hashes[idx].bytes_borrow().to_vec()
        }
    }

    pub fn root(&mut self) -> String {
        hex::encode(self.get_node(1))
    }

    pub fn prove(&mut self, idx: IndexT, data: &[u8]) -> bool {
        let computed_root = self.root_hash_for_idx_and_data(idx, data);
        let actual_root = self.root();

        let is_proved = *actual_root == *computed_root;
        is_proved
    }

    pub fn root_hash_for_idx_and_data(&mut self, idx: IndexT, data: &[u8]) -> String {
        let mut hash = HashConvenient::hash_bytes(&mut self.hasher, data);

        let mut n = idx + first_node_idx(self.height);
        while n > 1 {
            let sibling_vec = self.get_node(n ^ 1);
            let sibling = sibling_vec.as_slice();

            hash = if n & 1 == 0 {
                HashConvenient::hash_two_inputs(&mut self.hasher, hash.bytes_borrow(), sibling)
            } else {
                HashConvenient::hash_two_inputs(&mut self.hasher, sibling, hash.bytes_borrow())
            };

            n >>= 1;
        }

        let computed_root = hash.to_string();
        computed_root
    }

    /// appends a piece of data you want everybody to remember
    pub fn append(&mut self, data: &[u8]) -> IndexT {
        assert!( ! self.is_empty());

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

            curr_hash = if n & 1 == 0 {
                HashConvenient::hash_two_inputs(
                &mut self.hasher, curr_hash.bytes_borrow(), 
                sibling_hash.bytes_borrow())
            } else {
                HashConvenient::hash_two_inputs(
                &mut self.hasher, sibling_hash.bytes_borrow(),
                curr_hash.bytes_borrow())
            };

            n >>= 1;
            i += 1;
        }

        // update the tree root
        self.hash_cache[i] = TreeNode{
            index: n,
            hash: curr_hash.clone()
        };

        let tree_key = self.curr - first_node_idx(self.height);
        self.curr += 1;

        // our tree has fixed size, so 
        if self.is_empty() {
            self.save_state();
        }

        tree_key
    }

    /// dumps all the internal state to the database.
    pub fn save_state(&mut self) {
        for e in self.hash_cache.iter() {
            e.persistent_save(&mut self.store);
        }
    }

    pub fn capacity(&self) -> SizeT {
        ((1 as IndexT) << self.height) - self.curr
    }

    pub fn is_empty(&self) -> bool {
        self.capacity() == 0
    }

    #[allow(dead_code)]
    pub fn show_all(&self) {
        self.store.show_all()
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
