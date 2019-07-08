use sha2::Sha256;
mod merkle;
use merkle::{MerkleTree, IndexT, check_proof};

fn main() {
    // create a tree with height=4
    // it can contain up to 8 records
    let mut tree = MerkleTree::new(4);

    // these are the records we will store
    let business_transactions_orig = vec![
        "Alice pays Bob 17$",
        "Lublubah pays Alice 67$",
        "Bob pays Aania 100$",
        "Yeganeh pays Lublubah 70$",
        "Bob pays Alice 82$",
        "Alice pays Bob 39$",
        "Lublubah pays Bob 84$",
        "Alice pays Aania 12$",
    ];

    let business_transactions = &business_transactions_orig[..5];

    for t in business_transactions.iter() {
        let _key = tree.append(t.as_bytes());
    }

    // this is now optional
    tree.save_state();

    // proofs as they should be
    let mut hasher = Sha256::default();
    let incorrect_value = b"i'm incorrect ajajaja".to_vec();
    for i in 0..business_transactions.len() as IndexT {
        let (val, n, hashes) = tree.get_value_and_proof(i);
        let root = tree.root();
        assert!(check_proof(&mut hasher, &val, n, &hashes, &root));
        assert!( ! check_proof(&mut hasher, &incorrect_value, n, &hashes, &root));
    }
}
