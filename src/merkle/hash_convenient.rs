use std::clone::Clone;
use hex;
use sha2::{Sha256, Digest};
use sha2::digest::FixedOutput;
use sha2::digest::generic_array::GenericArray;
use super::{IndexT};

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

    pub fn hash_two_inputs(hasher: &mut Sha256, first: &[u8], second: &[u8]) -> HashConvenient {
        hasher.input(first);
        hasher.input(second);
        HashConvenient::new(hasher.result_reset())
    }

    #[allow(dead_code)]
    pub fn hash_from_hashes(hasher: &mut Sha256, first: String, second: String) -> HashConvenient {
        let first_dec = hex::decode(first).unwrap();
        let second_dec = hex::decode(second).unwrap();
        HashConvenient::hash_two_inputs(hasher, first_dec.as_slice(), second_dec.as_slice())
    }

    pub fn bytes_borrow(&self) -> &[u8] {
        &self.hash
    }

    pub fn to_string(&self) -> String {
        hex::encode(self.bytes_borrow())
    }

    pub fn hash_from_sibling_in_order(hasher: &mut Sha256, data: &[u8], sibling: &[u8], n: IndexT) -> HashConvenient {
        if n & 1 == 0 {
            HashConvenient::hash_two_inputs(hasher, data, sibling)
        } else {
            HashConvenient::hash_two_inputs(hasher, sibling, data)
        }
    }
}

impl Clone for HashConvenient {
    fn clone(&self) -> HashConvenient {
        HashConvenient {
            hash: self.hash.clone()
        }
    }
}
