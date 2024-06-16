use leveldb::options::{Options, ReadOptions};
use leveldb::db::Database;
use leveldb::error::Error;
use std::path::Path;

const DB_PATH: &str = "/Users/dongri/Library/Application Support/Google/Chrome/Profile 1/Sync Extension Settings/ghdmidohgjfoakfmjifbmhehabjlamka";
const KEY: &str = "key-123";

fn main() -> Result<(), Error> {
    let path = Path::new(DB_PATH);
    
    let mut options = Options::new();
    options.create_if_missing = true;

    let database = Database::open(&path, &options)?;
    let read_ops = ReadOptions::new();

    let key = KEY.to_string();

    let value = database.get(&read_ops, &key)?;

    match value {
        Some(v) => {
            match String::from_utf8(v) {
                Ok(string_value) => println!("Fetched value: {}", string_value),
                Err(e) => eprintln!("Failed to convert value to string: {:?}", e),
            }
        },
        None => {
            println!("Value not found");
        }
    }
    Ok(())
}
