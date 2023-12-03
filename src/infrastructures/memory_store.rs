use std::collections::HashMap;
use std::sync::RwLock;

use crate::adapters::data_store::DataStore;

#[derive(Debug)]
pub struct MemoryStore {
  store: RwLock<HashMap<String, Vec<u8>>>,
}

impl MemoryStore {
  #[allow(dead_code)] // NOTE: for testing
  pub fn new() -> Self {
    MemoryStore {
      store: RwLock::new(HashMap::new()),
    }
  }
}

impl DataStore for MemoryStore {
  fn read(&self, key: &str) -> Result<Vec<u8>, String> {
    let store = self.store.read().map_err(|e| e.to_string())?;
    store
      .get(key)
      .cloned()
      .ok_or_else(|| format!("Not found: {}", key))
  }

  fn exists(&self, key: &str) -> Result<bool, String> {
    let store = self.store.read().map_err(|e| e.to_string())?;
    Ok(store.contains_key(key))
  }

  fn write(&self, key: &str, data: &[u8]) -> Result<(), String> {
    let mut store = self.store.write().map_err(|e| e.to_string())?;
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

  #[test]
  fn test_exists() {
    let store = MemoryStore::new();
    let key = "test";
    let data = "test data".as_bytes();
    store.write(key, data).unwrap();
    assert!(store.exists(key).unwrap());
    assert!(!store.exists("not found").unwrap());
  }
}
