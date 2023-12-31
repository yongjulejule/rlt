use crate::{
  adapters::{compressor, hasher::Hasher, object_manager::ObjectManager},
  entities::object::Object,
};

pub trait ObjectService {
  fn save(&self, object: &Object) -> Result<String, String>;
  fn create_key(&self, object_type: &str, content: &[u8]) -> String;
  fn find(&self, key: &str) -> Result<Object, String>;
  fn delete(&self) -> Result<(), String>;
  fn is_object_hash(&self, hash: &str) -> bool {
    // TODO: consider sha256 later
    return hash.len() == 40 && hash.chars().all(|c| c.is_ascii_hexdigit());
  }
}

pub struct ObjectServiceImpl<'a> {
  object_manager: &'a dyn ObjectManager,
  hasher: &'a dyn Hasher,
}

impl<'a> ObjectServiceImpl<'a> {
  pub fn new(
    object_manager: &'a dyn ObjectManager,
    hasher: &'a dyn Hasher,
  ) -> Self {
    return Self {
      object_manager,
      hasher,
    };
  }
}

impl<'a> ObjectService for ObjectServiceImpl<'a> {
  fn create_key(&self, object_type: &str, content: &[u8]) -> String {
    let mut new_object = object_type.as_bytes().to_vec();
    new_object.push(b' ');
    new_object.extend_from_slice(content.len().to_string().as_bytes());
    new_object.push(b'\0');
    new_object.extend_from_slice(&content);
    self.hasher.hash(&new_object)
  }

  fn save(&self, object: &Object) -> Result<String, String> {
    let formatted_object = [
      format!("{} {}\0", object.object_type, object.size).as_bytes(),
      &object.data,
    ]
    .concat();
    let zipped_object = compressor::compress(&formatted_object);
    self
      .object_manager
      .write(object.hash.as_str(), &zipped_object)
      .map_err(|_| "Failed to write object".to_string())?;
    Ok(object.hash.clone())
  }

  fn find(&self, key: &str) -> Result<Object, String> {
    let data = self.object_manager.read(key)?;
    let unzipped = compressor::decompress(&data);
    let (content_type, content_length, content) =
      ObjectServiceImpl::parse_object(&unzipped)?;

    Ok(Object {
      object_type: content_type,
      hash: key.to_string(),
      data: content,
      size: content_length,
    })
  }

  fn delete(&self) -> Result<(), String> {
    todo!()
  }
}

impl<'a> ObjectServiceImpl<'a> {
  fn parse_object(content: &[u8]) -> Result<(String, usize, Vec<u8>), String> {
    let parts: Vec<&[u8]> =
      content.splitn(3, |&c| c == b' ' || c == b'\0').collect();
    if parts.len() != 3 {
      return Err("Invalid content format".to_string());
    }

    let contents_type = String::from_utf8(parts[0].to_vec())
      .map_err(|_| "Failed to parse content type".to_string())?;

    let size_str = String::from_utf8(parts[1].to_vec())
      .map_err(|_| "Failed to parse content size".to_string())?;
    let size = size_str
      .parse::<usize>()
      .map_err(|_| "Failed to parse content size".to_string())?;

    let body = parts[2].to_vec();

    Ok((contents_type, size, body))
  }
}

#[cfg(test)]
mod tests {

  use crate::{
    adapters::{hasher, object_manager::ObjectManagerImpl},
    infrastructures::memory_store::MemoryStore,
  };

  use super::*;

  #[test]
  fn create_key() {
    let test_data = "test-data".as_bytes().to_vec();
    let test_object = Object {
      object_type: "blob".to_string(),
      hash: "test-hash".to_string(),
      data: test_data.clone(),
      size: test_data.len(),
    };
    let memory_store = MemoryStore::new();
    let object_manager = ObjectManagerImpl::new(&memory_store);

    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());

    let object_service =
      ObjectServiceImpl::new(&object_manager, hasher.as_ref());
    let key =
      object_service.create_key(&test_object.object_type, &test_object.data);

    let raw_data = b"blob 9\0test-data";

    let result = hasher.hash(raw_data);

    assert_eq!(key, result);
  }

  #[test]
  fn save() {
    let test_data = "test-data".as_bytes().to_vec();
    let test_object = Object {
      object_type: "blob".to_string(),
      hash: "test-hash".to_string(),
      data: test_data.clone(),
      size: test_data.len(),
    };
    let memory_store = MemoryStore::new();
    let object_manager = ObjectManagerImpl::new(&memory_store);

    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());

    let object_service =
      ObjectServiceImpl::new(&object_manager, hasher.as_ref());
    let key = object_service.save(&test_object).unwrap();

    let object = object_service.find(key.as_str()).unwrap();

    assert_eq!(object, test_object);
  }

  #[test]
  fn find() {
    let test_data = "test-data".as_bytes().to_vec();
    let test_object = Object {
      object_type: "blob".to_string(),
      hash: "test-hash".to_string(),
      data: test_data.clone(),
      size: test_data.len(),
    };
    let memory_store = MemoryStore::new();
    let object_manager = ObjectManagerImpl::new(&memory_store);

    let formatted_object = [
      format!("{} {}\0", test_object.object_type, test_object.size).as_bytes(),
      &test_data,
    ]
    .concat();
    let compressed_data = compressor::compress(&formatted_object);
    object_manager
      .write(test_object.hash.as_str(), &compressed_data)
      .unwrap();

    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());

    let object_service =
      ObjectServiceImpl::new(&object_manager, hasher.as_ref());
    let object = object_service.find(test_object.hash.as_str()).unwrap();

    assert_eq!(object, test_object);
  }
}
