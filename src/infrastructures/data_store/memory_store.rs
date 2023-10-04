use std::collections::HashMap;
use std::sync::RwLock;

use super::data_store::DataStore;

#[derive(Debug)]
pub struct MemoryStore {
  store: RwLock<HashMap<String, Vec<u8>>>,
}

impl MemoryStore {
  pub fn new() -> Self {
    MemoryStore {
      store: RwLock::new(HashMap::new()),
    }
  }
}

impl DataStore for MemoryStore {
  fn read(&self, key: &str) -> Result<Vec<u8>, std::io::Error> {
    let store = self.store.read().unwrap();
    store.get(key).cloned().ok_or(std::io::Error::new(
      std::io::ErrorKind::NotFound,
      "Key not found",
    ))
  }

  fn write(&self, key: &str, data: &[u8]) -> Result<(), std::io::Error> {
    let mut store = self.store.write().unwrap();
    store.insert(key.to_string(), data.to_vec());
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_memory_store() {
    let store = MemoryStore::new();
    let key = "test";
    let data = "test data".as_bytes();
    store.write(key, data).unwrap();
    let read_data = store.read(key).unwrap();
    assert_eq!(data, read_data.as_slice());
  }

  #[test]
  fn test_memory_store_not_found() {
    let store = MemoryStore::new();
    let key = "test";
    let data = "test data".as_bytes();
    store.write(key, data).unwrap();
    let read_data = store.read("not found");
    assert!(read_data.is_err());
  }
}
