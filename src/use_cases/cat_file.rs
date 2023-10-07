use crate::adapters::{compressor, object_manager::ObjectManagement};

pub struct CatFile<'a> {
  object_manager: &'a dyn ObjectManagement,
  object_type: String,
  object_hash: String,
}

impl<'a> CatFile<'a> {
  pub fn new(
    object_manager: &'a dyn ObjectManagement,
    object_type: String,
    object_hash: String,
  ) -> Self {
    return Self {
      object_manager,
      object_type,
      object_hash,
    };
  }

  pub fn run(&self) -> Result<String, i32> {
    println!("cat_file: {:?}", self.object_hash);
    let content = self
      .object_manager
      .read(&self.object_hash, &self.object_type)
      .unwrap();
    let unzipped = compressor::decompress(&content);
    return Ok(String::from_utf8(unzipped).unwrap_or("error".to_string()));
  }
}

#[cfg(test)]
mod run_tests {
  use super::*;
  use crate::{
    adapters::hasher,
    adapters::{
      object_manager::ObjectManager, workspace_provider::WorkspaceProvider,
    },
    infrastructures::{
      memory_store::MemoryStore, test_content_provider::TestContentProvider,
    },
    use_cases::hash_object::HashObject,
  };
  #[test]
  fn test_cat_file() {
    let test_key = "test-key";
    let test_content = "test-content";
    let store = Box::new(MemoryStore::new());
    let object_manager = ObjectManager::new(store.as_ref());
    let mut provider = TestContentProvider::new();
    provider.set_contents(test_key.to_string(), test_content.to_string());

    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());
    let hash_object = HashObject::new(
      &object_manager,
      &provider,
      hasher.as_ref(),
      true,
      "blob".to_string(),
      vec![test_key.to_string()],
    );
    let hash = hash_object.run().unwrap().pop().unwrap();

    let cat_file = CatFile::new(&object_manager, "blob".to_string(), hash);
    let content = cat_file.run().unwrap();
    assert_eq!(content, test_content.to_string());
  }
}
