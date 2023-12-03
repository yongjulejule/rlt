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
  fn read(&self, key: &str) -> Result<Vec<u8>, std::io::Error> {
    let src_path = self.store_dir.join(key);
    let mut file = OpenOptions::new().read(true).open(&src_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap_or(0);
    Ok(data)
  }

  fn exists(&self, key: &str) -> bool {
    let src_path = self.store_dir.join(key);
    src_path.exists()
  }

  fn write(&self, key: &str, data: &[u8]) -> Result<(), std::io::Error> {
    let dest_path = self.store_dir.join(key);
    if key.ends_with("/") {
      return create_dir_all(dest_path);
    }
    create_dir_all(dest_path.parent().unwrap())?;
    let mut file = File::create(&dest_path)?;
    file.write_all(data)?;
    return Ok(());
  }
}
