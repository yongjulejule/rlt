pub struct ObjectManager {
  core: Core,
  data_store: FileStore,
}

impl ObjectManager {
  pub fn new(root_path: String) -> Self {
    let core = Core::new(root_path.clone());
    let data_store = FileStore::new(&root_path);
    return Self { core, data_store };
  }
}

impl Manager for ObjectManager {
  fn new(root_path: String) -> Self {
    return Self::new(root_path);
  }
}

#[cfg(test)]
mod new_tests {
  use super::*;
  use crate::manager::manager::Manager;

  #[test]
  fn test_new() {
    let manager = ObjectManager::new("test-root".to_string());
    assert_eq!(manager.core.root, "test-root");
  }
}
