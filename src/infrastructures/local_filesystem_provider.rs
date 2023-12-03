use std::{fs, path::PathBuf};

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
  fn get_contents(&self, key: String) -> String {
    let path = self.workspace_root.join(key);
    return fs::read_to_string(path).unwrap_or_else(|e| e.to_string());
  }

  fn set_contents(&mut self, key: String, contents: String) {
    let path = self.workspace_root.join(key);
    fs::write(path, contents).unwrap();
  }
}
