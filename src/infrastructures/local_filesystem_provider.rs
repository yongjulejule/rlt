use std::{
  fs::{File, OpenOptions},
  io::{Read, Write},
  path::PathBuf,
};

use crate::adapters::workspace_provider::WorkspaceProvider;

pub struct LocalFilesystemProvider {
  workspace_root: PathBuf,
}

impl LocalFilesystemProvider {
  pub fn new(workspace_root: &str) -> Self {
    return Self {
      workspace_root: PathBuf::from(workspace_root),
    };
  }
}

impl WorkspaceProvider for LocalFilesystemProvider {
  fn get_contents(&self, key: String) -> Result<Vec<u8>, String> {
    let path = self.workspace_root.join(key);
    let mut file: File = OpenOptions::new()
      .read(true)
      .open(path)
      .map_err(|e| e.to_string())?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    return Ok(buf);
  }

  fn set_contents(
    &mut self,
    key: String,
    contents: &[u8],
  ) -> Result<(), String> {
    let path = self.workspace_root.join(key);
    let mut file: File = OpenOptions::new()
      .write(true)
      .create(true)
      .open(path)
      .map_err(|e| e.to_string())?;
    file.write_all(contents).map_err(|e| e.to_string())
  }
}
