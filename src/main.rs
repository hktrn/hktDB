use std::io::{self, Write};

mod kv_storage;
use kv_storage::KVStorage;

fn main() {
    let mut kv_storage = KVStorage::new();

    loop {
        println!("Enter command (set, get, exit):");
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

                kv_storage.set(key, value);
                println!("Key-value pair set.");
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
            "exit" => break,
            _ => println!("Unknown command."),
        }
    }
}