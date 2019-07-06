use sha2::{Sha256, Digest};
use hex_literal::hex;
use std::vec::Vec;

mod merkle;
use merkle::MerkleTree;
use merkle::utils::log2_64;

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

    let mut a = MerkleTree::new(4);

    // for i in 0..8 {
    //     println!("saved {}", i);
    //     a.append(b"asdf");
    //     println!("###########################");
    // }

    let business_transactions = vec![
        "Alice pays Bob 17$",
        "Lublubah pays Alice 67$",
        "Bob pays Aania 100$",
        "Yeganeh pays Lublubah 70$",
        "Bob pays Alice 82$",
        "Alice pays Bob 39$",
        "Lublubah pays Bob 84$",
        "Alice pays Aania 12$",
    ];

    for t in business_transactions.iter() {
        let n = a.append(t.as_bytes());
    }


    for (i, t) in business_transactions.iter().enumerate() {
        println!("proving {} {}", i, t);
        for j in 0..8 {
            let res = a.prove(j + 8, t.as_bytes());
            assert!(res == (i == j));
        }
    }
    

    a.show_all();
    // for i in 0..60 {
    //     println!("{}: {}", 1u64 << i, log2_64(1u64 << i));
    // }
}
