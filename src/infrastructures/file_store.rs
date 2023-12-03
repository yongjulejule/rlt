use std::{
  fs::{create_dir_all, File, OpenOptions},
  io::{Read, Write},
  path::PathBuf,
};

use crate::adapters::data_store::DataStore;

pub struct FileStore {
  store_dir: PathBuf,
}

#[allow(dead_code)]
impl FileStore {
  pub fn new(store_dir: &str) -> Self {
    return Self {
      store_dir: PathBuf::from(store_dir),
    };
  }
}

impl DataStore for FileStore {
  fn read(&self, key: &str) -> Result<Vec<u8>, String> {
    let src_path = self.store_dir.join(key);
    let mut file = OpenOptions::new()
      .read(true)
      .open(&src_path)
      .map_err(|e| e.to_string())?;
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap_or(0);
    Ok(data)
  }

  fn exists(&self, key: &str) -> Result<bool, String> {
    let src_path = self.store_dir.join(key);
    Ok(src_path.exists())
  }

  fn write(&self, key: &str, data: &[u8]) -> Result<(), String> {
    let dest_path = self.store_dir.join(key);
    if key.ends_with("/") {
      return create_dir_all(dest_path).map_err(|e| e.to_string());
    }
    create_dir_all(dest_path.parent().unwrap()).map_err(|e| e.to_string())?;
    let mut file = File::create(&dest_path).map_err(|e| e.to_string())?;
    file.write_all(data).map_err(|e| e.to_string())?;
    return Ok(());
  }
}
