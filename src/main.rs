use std::io::{self, Write};
use anyhow::Result;

mod kv_storage;
use kv_storage::KVStorage;

fn main() -> Result<()> {
    let mut kv_storage = KVStorage::new();
    
    if let Err(e) = kv_storage.load_from_file("database.json") {
        eprintln!("Warning: Could not load existing data: {}", e);
        println!("Starting with empty database.");
    }

    loop {
        println!("Enter command (set, get, remove, exit):");
        let mut command = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "set" => {
                println!("Enter key:");
                let mut key = String::new();
                io::stdin().read_line(&mut key).unwrap();
                let key = key.trim().to_string();

                println!("Enter value:");
                let mut value = String::new();
                io::stdin().read_line(&mut value).unwrap();
                let value = value.trim().to_string();

                match kv_storage.set(key, value) {
                    Ok(_) => println!("Key-value pair set."),
                    Err(e) => eprintln!("Error setting key-value pair: {}", e),
                }
            }
            "get" => {
                println!("Enter key:");
                let mut key = String::new();
                io::stdin().read_line(&mut key).unwrap();
                let key = key.trim();

                match kv_storage.get(key) {
                    Some(value) => println!("Value: {}", value),
                    None => println!("Key not found."),
                }
            }
            "remove" => {
                println!("Enter key:");
                let mut key = String::new();
                io::stdin().read_line(&mut key).unwrap();
                let key = key.trim();
                match kv_storage.remove(key) {
                    Ok(Some(value)) => println!("Removed value: {}", value),
                    Ok(None) => println!("Key not found."),
                    Err(e) => eprintln!("Error removing key: {}", e),
                }
            }
            "exit" => break,
            _ => println!("Unknown command."),
        }
    }
    
    Ok(())
}