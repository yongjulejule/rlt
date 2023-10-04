pub struct ObjectManager {
  data_store: DataStore,
  object_directory: String,
}

impl ObjectManager {
  pub fn new(store: DataStore) -> Self {
    return Self {
      data_store: store,
      object_directory: "objects".to_string(),
    };
  }

  pub fn write(&self, key: &str, data: &[u8]) -> Result<(), std::io::Error> {
    return self.data_store.write(
      self.object_directory.clone() + "/" + &key[..2] + "/" + &key[2..],
      data,
    );
  }

  pub fn read(&self, key: &str) -> Result<Vec<u8>, std::io::Error> {
    return self
      .data_store
      .read(self.object_directory.clone() + "/" + &key[..2] + "/" + &key[2..]);
  }
}

#[cfg(test)]
mod new_tests {
  use super::*;
  use crate::adapters::manager::Manager;

  #[test]
  fn test_new() {
    let manager = ObjectManager::new("test-root".to_string());
    assert_eq!(manager.core.root, "test-root");
  }
}
