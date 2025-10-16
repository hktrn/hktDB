use std::collections::HashMap;

pub struct KVStorage {
    store: HashMap<String, String>,
}

impl KVStorage {
    pub fn new() -> Self {
        KVStorage {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }
}