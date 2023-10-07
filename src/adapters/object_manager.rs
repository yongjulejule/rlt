use super::data_store::DataStore;

pub trait ObjectManagement {
  fn write(
    &self,
    key: &str,
    object_type: &str,
    data: &[u8],
  ) -> Result<(), String>;
  fn read(&self, key: &str, object_type: &str) -> Result<Vec<u8>, String>;
}

pub struct ObjectManager<'a> {
  data_store: &'a dyn DataStore,
  object_directory: String,
}

impl<'a> ObjectManager<'a> {
  pub fn new(store: &'a dyn DataStore) -> Self {
    return Self {
      data_store: store,
      object_directory: "objects".to_string(),
    };
  }
}

fn parse_content(content: &[u8]) -> Result<(String, usize, Vec<u8>), String> {
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

fn check_object_type(
  contents_type: &str,
  object_type: &str,
) -> Result<(), String> {
  if contents_type != object_type {
    return Err("invalid object type".to_string());
  }
  return Ok(());
}

fn check_content_size(contents_size: usize, size: usize) -> Result<(), String> {
  if contents_size != size {
    return Err("invalid content size".to_string());
  }
  return Ok(());
}

impl<'a> ObjectManagement for ObjectManager<'a> {
  fn write(
    &self,
    key: &str,
    object_type: &str,
    data: &[u8],
  ) -> Result<(), String> {
    let header = format!("{} {}\0", object_type, data.len());
    let object: Vec<u8> = [header.as_bytes(), data].concat();

    let path = format!("{}/{}/{}", self.object_directory, &key[..2], &key[2..]);
    self
      .data_store
      .write(&path, &object)
      .map_err(|_| "Failed to write object".to_string())
  }

  fn read(&self, key: &str, object_type: &str) -> Result<Vec<u8>, String> {
    let path = format!("{}/{}/{}", self.object_directory, &key[..2], &key[2..]);
    let data = self.data_store.read(&path).expect("fail to read object");

    let (content_type, content_length, content) = parse_content(&data)?;
    check_object_type(&content_type, object_type)?;
    check_content_size(content.len(), content_length)?;
    Ok(content)
  }
}

#[cfg(test)]
mod tests {
  use crate::infrastructures::memory_store::MemoryStore;

  use super::*;

  #[test]
  fn test_parse_content() {
    let content = "blob 4\0test".as_bytes().to_vec();
    let (object_type, size, body) = parse_content(&content).unwrap();
    assert_eq!(object_type, "blob");
    assert_eq!(size, 4);
    assert_eq!(body, "test".as_bytes());
  }

  #[test]
  fn test_object_rw() {
    let memory_store = MemoryStore::new();
    let manager = ObjectManager::new(&memory_store);

    let object_key = "test";
    let object_type = "blob";
    let object_content = b"test";

    manager
      .write(object_key, object_type, object_content)
      .unwrap();
    let read_data = manager.read(object_key, object_type).unwrap();
    assert_eq!(object_content, read_data.as_slice());
  }

  #[test]
  fn test_object_invalid_type() {
    let memory_store = Box::new(MemoryStore::new());
    let manager = ObjectManager::new(memory_store.as_ref());

    let object_key = "test";
    let object_type = "blob";
    let object_content = "test";
    let object = format!(
      "{} {}\0{}",
      object_type,
      object_content.len(),
      object_content
    );

    manager
      .write(object_key, object_type, object.as_bytes())
      .unwrap();
    let read_data = manager.read(object_key, "commit");
    assert_eq!(read_data.is_err(), true);
  }
}
