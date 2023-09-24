use crate::{compressor, data_store::data_store::DataStore};

pub struct CatFile<'a> {
  store: &'a dyn DataStore,
  object_type: String,
  object_hash: String,
}

impl<'a> CatFile<'a> {
  pub fn new(
    store: &'a dyn DataStore,
    object_type: String,
    object_hash: String,
  ) -> Self {
    return Self {
      store,
      object_type,
      object_hash,
    };
  }

  pub fn run(&self) -> Result<String, i32> {
    let content = self.store.read(&self.object_hash).unwrap();
    let unzipped = compressor::decompress(&content);
    return Ok(String::from_utf8(unzipped).unwrap_or("".to_string()));
  }
}

#[cfg(test)]
mod run_tests {
  use super::*;
  #[allow(unused_imports)]
  use crate::data_store::{file_store::FileStore, memory_store::MemoryStore};
  use crate::{
    hash_object::HashObject, hasher,
    workspace_provider::test_content_provider::TestContentProvider,
    workspace_provider::workspace_provider::WorkspaceProvider,
  };

  #[test]
  fn test_cat_file() {
    let test_key = "test-key";
    let test_content = "test-content";
    let store = MemoryStore::new();
    let mut provider = TestContentProvider::new();
    provider.set_contents(test_key.to_string(), test_content.to_string());

    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());
    let hash_object = HashObject::new(
      &store,
      &provider,
      hasher.as_ref(),
      true,
      "blob".to_string(),
      vec![test_key.to_string()],
    );
    let hash = hash_object.run().unwrap().pop().unwrap();

    let cat_file = CatFile::new(&store, "blob".to_string(), hash);
    let content = cat_file.run().unwrap();
    assert_eq!(content, test_content.to_string());
  }
}
