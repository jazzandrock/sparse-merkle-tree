use sha2::{Sha256, Digest};
use hex_literal::hex;
use std::vec::Vec;

mod merkle;
use merkle::MerkleTree;

fn main() {
    println!("{:x?}", b"asd");
    println!("Hello, world!");
    let mut hasher: Sha256 = Sha256::new(); 

    // write input message
    hasher.input(b"hello world");

    let result = hasher.result();

    // read hash digest and consume hasher
    assert_eq!(result[..], hex!("
        b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
    ")[..]);

    let a = MerkleTree::new(10);
}
