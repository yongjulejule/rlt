use crate::{
  adapters::workspace_provider::WorkspaceProvider, entities::object::Object,
  use_cases::core::object_service::ObjectService,
};

pub struct HashObject<'a> {
  object_service: &'a dyn ObjectService,
  provider: &'a dyn WorkspaceProvider,
  write: bool,
  object_type: String,
  path: Vec<String>,
}

impl<'a> HashObject<'a> {
  pub fn new(
    object_service: &'a dyn ObjectService,
    provider: &'a dyn WorkspaceProvider,
    write: bool,
    object_type: String,
    path: Vec<String>,
  ) -> Self {
    return Self {
      object_service,
      provider,
      write,
      object_type,
      path,
    };
  }

  pub fn run(&self) -> Result<Vec<String>, String> {
    let result: Vec<String> = self
      .path
      .iter()
      .map(|p| {
        // hash with object type & content in path
        let content =
          self
            .provider
            .get_contents(p.to_string())
            .unwrap_or_else(|_| {
              panic!("Fail to get contents from path: {}", p);
            });
        let key = self.object_service.create_key(&self.object_type, &content);
        if self.write {
          let object =
            Object::new(&key, &self.object_type, &content, content.len());
          let _ = self.object_service.save(&object);
        }
        return key;
      })
      .collect();

    return Ok(result);
  }
}

#[cfg(test)]
mod tests {
  use crate::adapters::hasher;
  use crate::adapters::object_manager::ObjectManager;
  use crate::adapters::object_manager::ObjectManagerImpl;
  use crate::adapters::workspace_provider::WorkspaceProvider;
  use crate::infrastructures::memory_store::MemoryStore;
  use crate::infrastructures::test_content_provider::TestContentProvider;
  use crate::use_cases::commands::hash_object::HashObject;
  use crate::use_cases::core::object_service::ObjectServiceImpl;

  #[test]
  fn test_hash_object_sha1() {
    let store = MemoryStore::new();
    let object_manager = ObjectManagerImpl::new(&store);
    object_manager
      .write("test", b"test-body")
      .expect("write test");
    let mut provider = TestContentProvider::new();
    provider.set_contents("test".to_string(), b"test-body");
    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());

    let object_service =
      ObjectServiceImpl::new(&object_manager, hasher.as_ref());

    let write = false;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object =
      HashObject::new(&object_service, &provider, write, object_type, path);
    let result = hash_object.run().unwrap();
    assert_eq!(
      result.first().unwrap().to_owned(),
      "5f8ab8d1d6ed50d5b2a6c8102bac4228b4e7f973".to_string()
    );
  }

  #[test]
  fn test_hash_object_sha256() {
    let store = MemoryStore::new();
    let object_manager = ObjectManagerImpl::new(&store);
    object_manager
      .write("test", b"test-body")
      .expect("write test");
    let mut provider = TestContentProvider::new();
    provider.set_contents("test".to_string(), b"test-body");
    let hasher = hasher::HasherFactory::new().get_hasher("sha256".to_string());
    let object_service =
      ObjectServiceImpl::new(&object_manager, hasher.as_ref());

    let write = false;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object =
      HashObject::new(&object_service, &provider, write, object_type, path);
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
    let object_manager = ObjectManagerImpl::new(&store);
    let mut provider = TestContentProvider::new();
    provider.set_contents("test".to_string(), b"test-body");
    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());
    let object_service =
      ObjectServiceImpl::new(&object_manager, hasher.as_ref());

    let write = true;
    let object_type = "blob".to_string();
    let path = vec!["test".to_string()];

    let hash_object =
      HashObject::new(&object_service, &provider, write, object_type, path);
    let result = hash_object.run().unwrap();
    assert_eq!(
      result.first().unwrap().to_owned(),
      "5f8ab8d1d6ed50d5b2a6c8102bac4228b4e7f973".to_string()
    );
  }
}
