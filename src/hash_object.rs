use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

use crate::data_store::data_store::DataStore;
use crate::hasher::{Sha1Hasher, Sha256Hasher};
use crate::workspace_provider::workspace_provider::WorkspaceProvider;

use super::hasher::Hasher;

pub struct HashObject<'a> {
  store: &'a dyn DataStore,
  provider: &'a dyn WorkspaceProvider,
  hasher: Box<dyn Hasher>,
  write: bool,
  object_type: String,
  path: Vec<String>,
}

impl<'a> HashObject<'a> {
  pub fn new(
    store: &'a dyn DataStore,
    provider: &'a dyn WorkspaceProvider,
    hash_strategy: String,
    write: bool,
    object_type: String,
    path: Vec<String>,
  ) -> Self {
    return Self {
      store,
      provider,
      hasher: match hash_strategy.as_str() {
        "sha256" => Box::new(Sha256Hasher::new()),
        _ => Box::new(Sha1Hasher::new()),
      },
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
        let key = format!("{} {}\0{}", self.object_type, content.len(), content);
        let hashed_key = self.hasher.hash(&key);
        if self.write {
          let content = self.provider.get_contents(p.to_string());
          println!("content: {:?}", content.as_bytes());
          let zipped = compressor(content);
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

fn compressor(content: String) -> Vec<u8> {
  let mut compressed = ZlibEncoder::new(Vec::new(), Compression::default());
  compressed
    .write_all(content.as_bytes())
    .expect("failed to write to compressed file");
  return compressed
    .finish()
    .expect("failed to finish compressed file");
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    data_store::memory_store::MemoryStore,
    workspace_provider::test_content_provider::TestContentProvider,
  };

  #[test]
  fn test_hash_object_sha1() {
    let store = MemoryStore::new();
    store.write("test", b"test-body").expect("write test");
    let mut provider = TestContentProvider::new();
    provider.set_contents("test".to_string(), "test-body".to_string());

    let hash_strategy = "sha1".to_string();
    let write = false;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object = HashObject::new(&store, &provider, hash_strategy, write, object_type, path);
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

    let hash_strategy = "sha256".to_string();
    let write = false;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object = HashObject::new(&store, &provider, hash_strategy, write, object_type, path);
    let result = hash_object.run().unwrap();
    assert_eq!(
      result.first().unwrap().to_owned(),
      "459e301b232432c38d4d3cb64884fe94c42408916c035ae208b74f6fbd30d66d".to_string()
    );
  }

  #[test]
  fn test_hash_object_write() {
    let store = MemoryStore::new();
    let mut provider = TestContentProvider::new();
    provider.set_contents("test".to_string(), "test-body".to_string());

    let hash_strategy = "sha1".to_string();
    let write = true;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object = HashObject::new(&store, &provider, hash_strategy, write, object_type, path);
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
      compressor("test-body".to_string())
    )
  }
}
