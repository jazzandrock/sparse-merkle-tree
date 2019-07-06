use sha2::{Sha256, Digest};
use std::clone::Clone;
use super::utils::log2_64;

use super::hash_ren::HashConvenient;

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

    fn persistent_save(&self) {
        println!("persistently saved node {}, {}", self.index, self.hash.to_string());
    }
}

pub struct MerkleTree {
    height: i32,
    curr: SizeT,
    hasher: Sha256,
    empty_hashes: Vec<HashConvenient>,
    hash_cache: Vec<TreeNode>,
    sibling_cache: Vec<TreeNode>
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
            sibling_cache: sibling_cache
        };
    }

    /// appends a piece of data you want everybody to remember
    pub fn append(& mut self, data: &[u8]) -> IndexT {
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
            } else {
                if n & 1 == 0 {
                    if e.index != n {
                        e.persistent_save();
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
                        e.persistent_save();
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
        }

        let ret = self.curr;
        self.curr += 1;
        ret
    }
}
