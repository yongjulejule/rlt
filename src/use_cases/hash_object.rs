use crate::adapters::{
  compressor, hasher::Hasher, object_manager::ObjectManagement,
  workspace_provider::WorkspaceProvider,
};

pub struct HashObject<'a> {
  object_manager: &'a dyn ObjectManagement,
  provider: &'a dyn WorkspaceProvider,
  hasher: &'a dyn Hasher,
  write: bool,
  object_type: String,
  path: Vec<String>,
}

impl<'a> HashObject<'a> {
  pub fn new(
    object_manager: &'a dyn ObjectManagement,
    provider: &'a dyn WorkspaceProvider,
    hasher: &'a dyn Hasher,
    write: bool,
    object_type: String,
    path: Vec<String>,
  ) -> Self {
    return Self {
      object_manager,
      provider,
      hasher,
      write,
      object_type,
      path,
    };
  }

  pub fn run(&self) -> Result<Vec<String>, i32> {
    let result: Vec<String> = self
      .path
      .iter()
      .map(|p| {
        // hash with object type & content in path
        let content = self.provider.get_contents(p.to_string());
        let key =
          format!("{} {}\0{}", self.object_type, content.len(), content);
        let hashed_key = self.hasher.hash(&key);
        if self.write {
          let content = self.provider.get_contents(p.to_string());
          let zipped = compressor::compress(content.as_bytes());
          self
            .object_manager
            .write(hashed_key.as_str(), &self.object_type, &zipped)
            .expect("write hash");
        }
        return hashed_key;
      })
      .collect();

    return Ok(result);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::adapters::hasher;
  use crate::adapters::object_manager::ObjectManager;
  use crate::infrastructures::memory_store::MemoryStore;
  use crate::infrastructures::test_content_provider::TestContentProvider;

  #[test]
  fn test_hash_object_sha1() {
    let store = MemoryStore::new();
    let object_manager = ObjectManager::new(&store);
    object_manager
      .write("test", "blob", b"test-body")
      .expect("write test");
    let mut provider = TestContentProvider::new();
    provider.set_contents("test".to_string(), "test-body".to_string());
    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());

    let write = false;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object = HashObject::new(
      &object_manager,
      &provider,
      hasher.as_ref(),
      write,
      object_type,
      path,
    );
    let result = hash_object.run().unwrap();
    assert_eq!(
      result.first().unwrap().to_owned(),
      "5f8ab8d1d6ed50d5b2a6c8102bac4228b4e7f973".to_string()
    );
  }

  #[test]
  fn test_hash_object_sha256() {
    let store = MemoryStore::new();
    let object_manager = ObjectManager::new(&store);
    object_manager
      .write("test", "blob", b"test-body")
      .expect("write test");
    let mut provider = TestContentProvider::new();
    provider.set_contents("test".to_string(), "test-body".to_string());
    let hasher = hasher::HasherFactory::new().get_hasher("sha256".to_string());

    let write = false;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object = HashObject::new(
      &object_manager,
      &provider,
      hasher.as_ref(),
      write,
      object_type,
      path,
    );
    let result = hash_object.run().unwrap();
    assert_eq!(
      result.first().unwrap().to_owned(),
      "459e301b232432c38d4d3cb64884fe94c42408916c035ae208b74f6fbd30d66d"
        .to_string()
    );
  }

  #[test]
  fn test_hash_object_write() {
    let store = MemoryStore::new();
    let object_manager = ObjectManager::new(&store);
    let mut provider = TestContentProvider::new();
    provider.set_contents("test".to_string(), "test-body".to_string());
    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());

    let write = true;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object = HashObject::new(
      &object_manager,
      &provider,
      hasher.as_ref(),
      write,
      object_type,
      path,
    );
    let result = hash_object.run().unwrap();
    assert_eq!(
      result.first().unwrap().to_owned(),
      "5f8ab8d1d6ed50d5b2a6c8102bac4228b4e7f973".to_string()
    );
    assert_eq!(
      hash_object
        .object_manager
        .read("5f8ab8d1d6ed50d5b2a6c8102bac4228b4e7f973", "blob")
        .unwrap(),
      compressor::compress("test-body".as_bytes())
    )
  }
}
