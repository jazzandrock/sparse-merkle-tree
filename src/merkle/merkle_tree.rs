use sha2::{Sha256, Digest};

use super::hash_ren::HashConvenient;
pub struct MerkleTree {
    height: i32,
    hasher: Sha256,
    emptyHashes: Vec<HashConvenient>
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

        empty_hashes.reverse();

        for digest in &empty_hashes {
            println!("{}", digest.to_string());
        }
 
        return MerkleTree {
            height: 10,
            hasher: hasher,
            emptyHashes: empty_hashes
        };
    }
}

type IndexT = i32;

struct TreeNode {
    index: IndexT,
    hash: HashConvenient
}

type SizeT = usize;

fn sibling(n: SizeT) -> SizeT {
    n ^ 1
}
