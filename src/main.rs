use sha2::Sha256;
mod merkle;
use merkle::MerkleTree;

fn main() {
    let mut tree = MerkleTree::new(4);

    tree.append("Alice pays Bob 17$".as_bytes());
    tree.append("Bob pays Alice 31$".as_bytes());

    // val is "Alice pays Bob 17$" as Vec<u8>
    let (val, proof) = tree.get_value_and_proof(0);

    let mut hasher = Sha256::default();
    let root = tree.root();

    assert!(proof.check(&mut hasher, &val, &root) == true);

    let incorrect = b"incorrect data".to_vec();
    assert!(proof.check(&mut hasher, &incorrect, &root) == false);

    tree.save_state();
}
