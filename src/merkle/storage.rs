use std::collections::HashMap;
use std::vec::Vec;

use super::merkle_tree::{IndexT};

pub trait KeyValueStore {
    fn put(&mut self, k: IndexT, v: &[u8]);
    fn get(&self, k: IndexT) -> &[u8];
}

pub struct LocalKeyValueStore {
    map: HashMap<IndexT, Vec<u8>>
}

impl LocalKeyValueStore {
    pub fn new() -> LocalKeyValueStore {
        LocalKeyValueStore{
            map: HashMap::new()
        }
    }

    fn show_all(&self) {
        for (k, v) in self.map.iter() {
            println!("{}: {:x?}", k, v);
        }
    }
}

impl KeyValueStore for LocalKeyValueStore {
    fn put(&mut self, k: IndexT, v: &[u8]) {
        self.map.insert(k, v.to_vec());
    }

    fn get(&self, k: IndexT) -> &[u8] {
        self.map.get(&k).unwrap()
    }
}
