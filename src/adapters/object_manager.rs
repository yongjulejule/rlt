use super::data_store::DataStore;

pub struct ObjectManager {
  data_store: Box<dyn DataStore>,
  object_directory: String,
}

impl ObjectManager {
  pub fn new(store: Box<dyn DataStore>) -> Self {
    return Self {
      data_store: store,
      object_directory: "objects".to_string(),
    };
  }
}

impl DataStore for ObjectManager {
  fn write(&self, key: &str, data: &[u8]) -> Result<(), std::io::Error> {
    return self.data_store.write(
      &(self.object_directory.clone() + "/" + &key[..2] + "/" + &key[2..]),
      data,
    );
  }

  fn read(&self, key: &str) -> Result<Vec<u8>, std::io::Error> {
    println!(
      "read key: {}",
      &(self.object_directory.clone() + "/" + &key[..2] + "/" + &key[2..])
    );
    return self.data_store.read(
      &(self.object_directory.clone() + "/" + &key[..2] + "/" + &key[2..]),
    );
  }
}

#[cfg(test)]
mod tests {
  use crate::infrastructures::memory_store::MemoryStore;

  use super::*;

  #[test]
  fn test_object_rw() {
    let memory_store = Box::new(MemoryStore::new());
    let manager = ObjectManager::new(memory_store);

    manager.write("test", "test data".as_bytes()).unwrap();
    let read_data = manager.read("test").unwrap();
    assert_eq!("test data".as_bytes(), read_data.as_slice());
  }
}
