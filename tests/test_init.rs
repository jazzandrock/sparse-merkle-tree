#[macro_use]
extern crate lazy_static;

use sha2::Sha256;
extern crate hello;
use hello::{MerkleTree, IndexT};

lazy_static! {
    static ref BUSINESS_TRANSACTIONS: Vec<&'static str> = {
        vec![
            "Alice pays Bob 17$",
            "Lublubah pays Alice 67$",
            "Bob pays Aania 100$",
            "Yeganeh pays Lublubah 70$",
            "Bob pays Alice 82$",
            "Alice pays Bob 39$",
            "Lublubah pays Bob 84$",
            "Alice pays Aania 12$",
        ]
    };
}


fn fill(tree: &mut MerkleTree) {
    for (i, t) in BUSINESS_TRANSACTIONS.iter().enumerate() {
        let key = tree.append(t.as_bytes());
        assert!(key == i);
    }
}

fn fill_and_prove(tree: &mut MerkleTree, save_state: bool) {
    let mut hasher = Sha256::default();

    fill(tree);
    
    if save_state {
        tree.save_state()
    }

    let root = tree.root();
    let incorrect_value = b"I'm incorrect lalala".to_vec();
    for i in 0..8 as IndexT {
        let (data, proof) = tree.get_value_and_proof(i);  
        // it proves what is shoud
        assert!(proof.check(&mut hasher, &data, &root));

        // and doesn't prove what shouldn't
        assert!( ! proof.check(&mut hasher, &incorrect_value, &root));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proving_full_tree_with_save_state() {
        let mut tree = MerkleTree::new(4);

        fill_and_prove(&mut tree, true);
    }

    #[test]
    fn proving_full_tree_without_save_state() {
        let mut tree = MerkleTree::new(4);

        fill_and_prove(&mut tree, false);
    }


    #[test]
    fn proving_half_empty_tree_with_save_state() {
        let mut tree = MerkleTree::new(63);

        fill_and_prove(&mut tree, true);
    }

    #[test]
    fn proving_half_empty_tree_without_save_state() {
        let mut tree = MerkleTree::new(63);

        fill_and_prove(&mut tree, false);
    }
}
