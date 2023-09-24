use crate::compressor;
use crate::data_store::data_store::DataStore;
use crate::workspace_provider::workspace_provider::WorkspaceProvider;

use super::hasher::Hasher;

pub struct HashObject<'a> {
  store: &'a dyn DataStore,
  provider: &'a dyn WorkspaceProvider,
  hasher: &'a dyn Hasher,
  write: bool,
  object_type: String,
  path: Vec<String>,
}

impl<'a> HashObject<'a> {
  pub fn new(
    store: &'a dyn DataStore,
    provider: &'a dyn WorkspaceProvider,
    hasher: &'a dyn Hasher,
    write: bool,
    object_type: String,
    path: Vec<String>,
  ) -> Self {
    return Self {
      store,
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
            .store
            .write(hashed_key.as_str(), &zipped)
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
  use crate::{
    data_store::memory_store::MemoryStore, hasher,
    workspace_provider::test_content_provider::TestContentProvider,
  };

  #[test]
  fn test_hash_object_sha1() {
    let store = MemoryStore::new();
    store.write("test", b"test-body").expect("write test");
    let mut provider = TestContentProvider::new();
    provider.set_contents("test".to_string(), "test-body".to_string());
    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());

    let write = false;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object = HashObject::new(
      &store,
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
    store.write("test", b"test-body").expect("write test");
    let mut provider = TestContentProvider::new();
    provider.set_contents("test".to_string(), "test-body".to_string());
    let hasher = hasher::HasherFactory::new().get_hasher("sha256".to_string());

    let write = false;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object = HashObject::new(
      &store,
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
    let mut provider = TestContentProvider::new();
    provider.set_contents("test".to_string(), "test-body".to_string());
    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());

    let write = true;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object = HashObject::new(
      &store,
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
        .store
        .read("5f8ab8d1d6ed50d5b2a6c8102bac4228b4e7f973")
        .unwrap(),
      compressor::compress("test-body".as_bytes())
    )
  }
}
