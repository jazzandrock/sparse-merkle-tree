use sha2::Sha256;
use sha2::digest::FixedOutput;
use sha2::digest::generic_array::GenericArray;
use hex::encode;

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

    pub fn bytes_borrow(&self) -> &[u8] {
        &self.hash
    }

    pub fn to_string(&self) -> String {
        encode(self.bytes_borrow())
    }
}
