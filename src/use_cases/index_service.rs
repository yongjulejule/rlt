use std::os::unix::prelude::MetadataExt;

use crate::entities::index::{Index, IndexEntry};

pub trait IndexService {
  fn get_index(&self) -> &Index;
  fn save_entry(&mut self, entry: IndexEntry);
  fn delete_entry(&mut self, key: &str) -> Result<(), String>;
  fn update_extension(&self, extension_type: &str) -> Result<String, String>;
  fn delete_extension(&self) -> Result<(), String>;
  fn delete(&self) -> Result<(), String>;
  fn create_entry_from_file(
    &self,
    key: &str,
    file_path: &str,
  ) -> Result<IndexEntry, String>;
}

#[derive(Debug, Clone)]
pub struct IndexServiceImpl {
  index: Index,
}

impl IndexServiceImpl {
  pub fn new() -> Self {
    let index = Index::new();
    Self { index }
  }

  pub fn from_index(index: Index) -> Self {
    Self { index }
  }

  pub fn from_string(index_string: &str) -> Self {
    todo!()
  }

  fn serialize(&self) -> Result<String, String> {
    todo!()
  }

  fn deserialize(&self) -> Result<Index, String> {
    todo!()
  }
}

impl IndexService for IndexServiceImpl {
  fn get_index(&self) -> &Index {
    &self.index
  }

  fn create_entry_from_file(
    &self,
    key: &str,
    file_path: &str,
  ) -> Result<IndexEntry, String> {
    let metadata =
      std::fs::metadata(file_path).map_err(|_| "Fail to get metadata")?;
    let entry = IndexEntry {
      ctime: metadata.ctime(),
      ctime_nsec: metadata.ctime_nsec(),
      mtime: metadata.mtime(),
      mtime_nsec: metadata.mtime_nsec(),
      dev: metadata.dev(),
      ino: metadata.ino(),
      mode: metadata.mode(),
      uid: metadata.uid(),
      gid: metadata.gid(),
      size: metadata.size(),
      hash: key.to_string(),
      flags: 0,
      name: file_path.to_string(),
    };
    Ok(entry)
  }

  fn save_entry(&mut self, entry: IndexEntry) {
    self.index.entries.insert(entry.name.clone(), entry);
    self.index.entries_count += 1;
  }

  fn delete_entry(&mut self, key: &str) -> Result<(), String> {
    self.index.entries.remove(key);
    self.index.entries_count -= 1;
    Ok(())
  }

  fn update_extension(&self, extension_type: &str) -> Result<String, String> {
    todo!()
  }

  fn delete_extension(&self) -> Result<(), String> {
    todo!()
  }

  fn delete(&self) -> Result<(), String> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use crate::use_cases::index_service::{IndexService, IndexServiceImpl};

  #[test]
  fn test_entry_creation() {
    let index_service = IndexServiceImpl::new();
    let test_file = "test-index";
    let entry = index_service.create_entry_from_file("test-key", test_file);

    println!("{:?}", entry);
    assert_eq!(entry.is_ok(), true);
    assert_eq!(entry.unwrap().name, test_file);
  }

  #[test]
  fn test_entry_save() {
    let mut index_service = IndexServiceImpl::new();
    let test_file = "test-index";
    let entry = index_service.create_entry_from_file("test-key", test_file);
    index_service.save_entry(entry.unwrap());

    assert_eq!(index_service.get_index().entries_count, 1);
    assert_eq!(
      index_service.get_index().entries.contains_key(test_file),
      true
    );
  }
}
