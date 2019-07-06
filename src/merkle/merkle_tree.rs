use sha2::{Sha256, Digest};
use std::clone::Clone;
use std::collections::HashMap;
use super::utils::log2_64;
use super::storage::{LocalKeyValueStore, KeyValueStore};
use super::hash_ren::HashConvenient;
use hex::encode;

pub type IndexT = usize;
pub type SizeT = usize;

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

pub struct MerkleTree {
    height: i32,
    curr: SizeT,
    hasher: Sha256,
    empty_hashes: Vec<HashConvenient>,
    hash_cache: Vec<TreeNode>,
    sibling_cache: Vec<TreeNode>,
    store: LocalKeyValueStore
}

impl MerkleTree {
    pub fn new(height: i32) -> Self {
        let mut hasher: Sha256 = Sha256::default();
        let mut empty_hashes = Vec::<HashConvenient>::new();

        empty_hashes.push(HashConvenient::zero_hash());

        for i in 1..height as usize {
            let bytes = empty_hashes[i - 1].bytes_borrow();

            // two times, h(0 + 0)
            hasher.input(bytes);
            hasher.input(bytes);

            let digest = hasher.result_reset();
            empty_hashes.push(HashConvenient::new(digest));
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

        for digest in &empty_hashes {
            println!("{}", digest.to_string());
        }

        let curr = (1 as SizeT) << (height - 1);

        let mut sibling_cache = Vec::<TreeNode>::new();
        for _ in 0..height {
            sibling_cache.push(TreeNode::dummy());
        }

        return MerkleTree {
            height: height,
            curr: curr,
            hasher: hasher,
            empty_hashes: empty_hashes,
            hash_cache: hash_cache,
            sibling_cache: sibling_cache,
            store: LocalKeyValueStore::new()
        };
    }

    pub fn prove(&mut self, mut n: IndexT, data: &[u8]) -> bool {
        let mut hash = HashConvenient::hash_bytes(&mut self.hasher, data);

        while n > 1 {
            let sibling = self.store.get(n ^ 1);

            if n & 1 == 0 {
                self.hasher.input(sibling);
                self.hasher.input(hash.bytes_borrow());
            } else {
                self.hasher.input(hash.bytes_borrow());
                self.hasher.input(sibling);
            }

            hash = HashConvenient::new(self.hasher.result_reset());

            n >>= 1;

            println!("n: {}", n);
            println!("prov hash: {}", hash.to_string());
            println!("real hash: {}", encode(self.store.get(n)));
        }

        println!("prov hash: {}", hash.to_string());
        println!("real hash: {}", encode(self.store.get(1)));

        encode(self.store.get(1)) == hash.to_string()
    }

    /// appends a piece of data you want everybody to remember
    pub fn append(&mut self, data: &[u8]) -> IndexT {
        let mut curr_hash = HashConvenient::hash_bytes(&mut self.hasher, data);

        for (i, e) in self.hash_cache.iter_mut().enumerate() {
            println!("{}", i);
            println!("{}", e.hash.to_string());
            let n = self.curr >> i;
            
            if n == 1 {
                *e = TreeNode{
                    index: n,
                    hash: curr_hash.clone()
                };
            } else if n & 1 == 0 {
                if e.index != n {
                    e.persistent_save(&mut self.store);
                    self.sibling_cache[i] = e.clone();
                }

                *e = TreeNode{
                    index: n,
                    hash: curr_hash.clone()
                };

                let sibling_hash = &self.empty_hashes[i];
                self.hasher.input(curr_hash.bytes_borrow());
                self.hasher.input(sibling_hash.bytes_borrow());
                curr_hash = HashConvenient::new(self.hasher.result_reset());
            } else {
                if e.index != n {
                    e.persistent_save(&mut self.store);
                    self.sibling_cache[i] = e.clone();
                }

                *e = TreeNode{
                    index: n,
                    hash: curr_hash.clone()
                };
                
                let sibling_hash = &self.sibling_cache[i].hash;
                self.hasher.input(curr_hash.bytes_borrow());
                self.hasher.input(sibling_hash.bytes_borrow());
                curr_hash = HashConvenient::new(self.hasher.result_reset());
            }
        }

        let ret = self.curr;
        self.curr += 1;

        println!("self.curr: {}", self.curr);
        if self.curr == (1 as IndexT) << self.height {
            // this is the last element in our tree, we can't insert anymore
            println!("we are dumping our cache, it's over");
            for e in self.hash_cache.iter() {
                e.persistent_save(&mut self.store);
            }
        }

        ret
    }

    pub fn show_all(&self) {
        self.store.show_all()
    }
}
