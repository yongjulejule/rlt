use std::{fs::Metadata, os::unix::prelude::MetadataExt, ptr::metadata};

use crate::entities::index::{Index, IndexEntry};

pub trait IndexService {
  fn get_index(&self) -> &Index;
  fn update_entry(&self) -> Result<String, String>;
  fn delete_entry(&self, key: &str) -> Result<(), String>;
  fn update_extension(&self, extension_type: &str) -> Result<String, String>;
  fn delete_extension(&self) -> Result<(), String>;
  fn delete(&self) -> Result<(), String>;
  fn update_entry_from_file(
    &self,
    key: &str,
    file_path: &str,
  ) -> Result<String, String>;
}

#[derive(Debug, Clone)]
pub struct IndexServiceImpl {
  index: Index,
}

impl IndexServiceImpl {
  pub fn new() -> Self {
    let index = Index {
      signature: "DIRC".to_string(),
      version: "2".to_string(),
      entries_count: 0,
      entries: Vec::new(),
      extensions: Vec::new(),
      checksum: Vec::new(),
    };
    Self { index }
  }
}
impl IndexService for IndexServiceImpl {
  fn get_index(&self) -> &Index {
    &self.index
  }

  fn update_entry_from_file(
    &self,
    key: &str,
    file_path: &str,
  ) -> Result<String, String> {
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

    self.index.entries.push(entry);
    Ok("".to_string())
  }

  fn update_entry(&self) -> Result<String, String> {
    todo!()
  }

  fn delete_entry(&self, key: &str) -> Result<(), String> {
    todo!()
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
