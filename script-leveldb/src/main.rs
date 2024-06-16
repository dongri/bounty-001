use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::WriteOptions;
use leveldb::options::{Options, ReadOptions};
use std::path::Path;

fn main() {
    let path = Path::new("db/leveldb");

    let mut options = Options::new();
    options.create_if_missing = true;
    let database: Database<i32> = match Database::open(path, options) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to open database: {:?}", e);
            return;
        }
    };

    let key: i32 = 123;
    let value = b"dongri";

    let write_options = WriteOptions::new();

    match database.put(write_options, key, value) {
        Ok(_) => {
            println!("Successfully wrote value to the database");
        }
        Err(e) => {
            eprintln!("Failed to write value: {:?}", e);
            return;
        }
    }

    let read_options = ReadOptions::new();

    match database.get(read_options, key) {
        Ok(Some(value)) => {
            match String::from_utf8(value) {
                Ok(string_value) => println!("Fetched value: {}", string_value),
                Err(e) => eprintln!("Failed to convert value to string: {:?}", e),
            }
        }
        Ok(None) => {
            println!("Key not found");
        }
        Err(e) => {
            eprintln!("Failed to fetch value: {:?}", e);
        }
    }
}
