mod merkle;
use merkle::MerkleTree;

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

    // let's see if proofs work correctly
    for (i, t) in business_transactions.iter().enumerate() {
        println!("proving {} {}", i, t);
        for j in 0..business_transactions.len() {
            let res = tree.prove(j, t.as_bytes());
            assert!(res == (i == j));
        }
    }
}
