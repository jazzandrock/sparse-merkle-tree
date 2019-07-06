use sha2::{Sha256, Digest};
use sha2::digest::FixedOutput;
use sha2::digest::generic_array::GenericArray;
use hex::encode;
use std::clone::Clone;

const HASH_SIZE: usize = 32;
type Hash = GenericArray<u8, <sha2::Sha256 as FixedOutput>::OutputSize>;

pub struct HashConvenient {
    hash: Hash
}

impl HashConvenient {
    pub fn new(hash: Hash) -> Self {
        HashConvenient {
            hash: hash
        }
    }

    pub fn zero_hash() -> Self {
        let hash = *Hash::from_slice(&[0 as u8; HASH_SIZE]);
        HashConvenient::new(hash)
    }

    pub fn hash_bytes(hasher: &mut Sha256, data: &[u8]) -> HashConvenient {
        hasher.input(data);
        HashConvenient::new(hasher.result_reset())
    }

    pub fn bytes_borrow(&self) -> &[u8] {
        &self.hash
    }

    pub fn to_string(&self) -> String {
        encode(self.bytes_borrow())
    }
}

impl Clone for HashConvenient {
    fn clone(&self) -> HashConvenient {
        HashConvenient {
            hash: self.hash.clone()
        }
    }
}
