// use std::path::PathBuf;

use super::data_store::DataStore;

pub struct FileStore;

impl FileStore {
    pub fn new() -> Self {
        return Self;
    }
}

impl DataStore for FileStore {
    fn read(&self, key: &str) -> Result<Vec<u8>, std::io::Error> {
        return Ok(format!("Let's Read the file from {}", key)
            .as_bytes()
            .to_vec());
    }

    fn write(&self, key: &str, data: &[u8]) -> Result<(), std::io::Error> {
        println!(
            "Let's write the file to {} with data {:?}",
            key,
            data.get(0..10).unwrap()
        );
        return Ok(());
    }
}
