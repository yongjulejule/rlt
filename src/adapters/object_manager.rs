use super::data_store::DataStore;

pub trait ObjectManager {
  fn write(&self, key: &str, data: &[u8]) -> Result<(), String>;
  fn read(&self, key: &str) -> Result<Vec<u8>, String>;
}

pub struct ObjectManagerImpl<'a> {
  data_store: &'a dyn DataStore,
  object_directory: String,
}

impl<'a> ObjectManagerImpl<'a> {
  pub fn new(store: &'a dyn DataStore) -> Self {
    return Self {
      data_store: store,
      object_directory: "objects".to_string(),
    };
  }
}

impl<'a> ObjectManager for ObjectManagerImpl<'a> {
  fn write(&self, key: &str, data: &[u8]) -> Result<(), String> {
    let path = format!("{}/{}/{}", self.object_directory, &key[..2], &key[2..]);
    self
      .data_store
      .write(&path, data)
      .map_err(|_| "Failed to write object".to_string())
  }

  fn read(&self, key: &str) -> Result<Vec<u8>, String> {
    let path = format!("{}/{}/{}", self.object_directory, &key[..2], &key[2..]);
    self
      .data_store
      .read(&path)
      .map_err(|_| "Failed to read object".to_string())
  }
}

#[cfg(test)]
mod tests {
  use crate::infrastructures::memory_store::MemoryStore;

  use super::*;

  #[test]
  fn test_object_rw() {
    let memory_store = MemoryStore::new();
    let manager = ObjectManagerImpl::new(&memory_store);

    let object_key = "test";
    let object_content = b"test";

    manager.write(object_key, object_content).unwrap();
    let read_data = manager.read(object_key).unwrap();
    assert_eq!(object_content, read_data.as_slice());
  }
}
