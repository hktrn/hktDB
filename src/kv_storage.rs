use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;

pub struct KVStorage {
    store: HashMap<String, String>,
    file_path: String,
}

impl KVStorage {
    pub fn new() -> Self {
        KVStorage {
            store: HashMap::new(),
            file_path: "database.json".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn new_with_file(file_path: String) -> Self {
        KVStorage {
            store: HashMap::new(),
            file_path,
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<bool> {
        let key_existed = self.store.contains_key(&key);
        self.store.insert(key, value);
        self.save_to_file(&self.file_path)?;
        Ok(key_existed)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.store.contains_key(key)
    }

    pub fn remove(&mut self, key: &str) -> Result<Option<String>> {
        let result = self.store.remove(key);
        if result.is_some() {
            self.save_to_file(&self.file_path)?;
        }
        Ok(result)
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let json_data = serde_json::to_string_pretty(&self.store)?;
        fs::write(file_path, json_data)?;
        println!("Data saved to {}", file_path);
        Ok(())
    }

    pub fn load_from_file(&mut self, file_path: &str) -> Result<()> {
        if !Path::new(file_path).exists() {
            println!("File {} does not exist, starting with empty storage", file_path);
            return Ok(());
        }

        let file_content = fs::read_to_string(file_path)?;
        let loaded_data: HashMap<String, String> = serde_json::from_str(&file_content)?;
        
        self.store = loaded_data;
        println!("Data loaded from {}", file_path);
        Ok(())
    }
}