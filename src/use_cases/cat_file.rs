use crate::use_cases::utils::{check_content_size, check_object_type};

use super::object_service::ObjectService;

pub struct CatFile<'a> {
  object_helper: &'a dyn ObjectService,
  object_type: String,
  object_hash: String,
}

impl<'a> CatFile<'a> {
  pub fn new(
    object_helper: &'a dyn ObjectService,
    object_type: String,
    object_hash: String,
  ) -> Self {
    return Self {
      object_helper,
      object_type,
      object_hash,
    };
  }

  pub fn run(&self) -> Result<String, String> {
    println!("cat_file: {:?}", self.object_hash);
    let object = self.object_helper.find(&self.object_hash)?;

    check_object_type(&object.object_type, &self.object_type)?;
    check_content_size(object.size, object.data.len())?;

    String::from_utf8_lossy(&object.data)
      .parse()
      .map_err(|_| "invalid type".to_string())
  }
}

#[cfg(test)]
mod run_tests {
  use super::*;
  use crate::{
    adapters::hasher,
    adapters::{
      object_manager::{ObjectManagement, ObjectManager},
      workspace_provider::WorkspaceProvider,
    },
    infrastructures::{
      memory_store::MemoryStore, test_content_provider::TestContentProvider,
    },
    use_cases::{hash_object::HashObject, object_service::ObjectHelper},
  };

  #[test]
  fn test_cat_file() {
    let test_key = "test-key";
    let test_content = "test-content";
    let store = Box::new(MemoryStore::new());
    let object_manager = ObjectManager::new(store.as_ref());
    let mut provider = TestContentProvider::new();
    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());
    let object_service = ObjectHelper::new(&object_manager, hasher.as_ref());
    provider.set_contents(test_key.to_string(), test_content.to_string());

    let hash_object = HashObject::new(
      &object_service,
      &provider,
      true,
      "blob".to_string(),
      vec![test_key.to_string()],
    );
    let hash = hash_object.run().unwrap().pop().unwrap();
    println!("hash: {:?}", hash);
    println!(
      "read data: {:?}",
      String::from_utf8_lossy(&object_manager.read(&hash).unwrap())
    );

    let cat_file = CatFile::new(&object_service, "blob".to_string(), hash);
    let content = cat_file.run().unwrap();
    assert_eq!(content, test_content.to_string());
  }
}
