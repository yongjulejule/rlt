use log::trace;

use crate::use_cases::{
  commands::utils::{check_content_size, check_object_type},
  core::object_service::ObjectService,
};

pub struct CatFile<'a> {
  object_service: &'a dyn ObjectService,
  object_type: String,
  object_hash: String,
}

impl<'a> CatFile<'a> {
  pub fn new(
    object_service: &'a dyn ObjectService,
    object_type: String,
    object_hash: String,
  ) -> Self {
    return Self {
      object_service,
      object_type,
      object_hash,
    };
  }

  pub fn run(&self) -> Result<String, String> {
    trace!("cat_file: {:?}", self.object_hash);
    let object = self.object_service.find(&self.object_hash)?;

    check_object_type(&object.object_type, &self.object_type)?;
    check_content_size(object.size, object.data.len())?;

    String::from_utf8_lossy(&object.data)
      .parse()
      .map_err(|_| "invalid type".to_string())
  }
}

#[cfg(test)]
mod run_tests {
  use log::info;

  use super::*;
  use crate::{
    adapters::hasher, adapters::object_manager::ObjectManager,
    entities::object::Object, infrastructures::memory_store::MemoryStore,
    use_cases::core::object_service::ObjectServiceImpl,
  };

  #[test]
  fn test_cat_file() {
    // setup
    let test_content = "test-content";
    let store = Box::new(MemoryStore::new());
    let object_manager = ObjectManager::new(store.as_ref());
    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());
    let object_service =
      ObjectServiceImpl::new(&object_manager, hasher.as_ref());
    let key = object_service.create_key("blob", test_content);
    let _ = object_service.save(&Object::new(
      "blob",
      &key,
      test_content.as_bytes(),
      test_content.len(),
    ));

    // run
    let cat_file = CatFile::new(&object_service, "blob".to_string(), key);
    let content = cat_file.run();
    if content.is_err() {
      info!("error: {:?}", content);
      assert!(false);
    }
    assert_eq!(content.unwrap(), test_content.to_string());
  }
}
