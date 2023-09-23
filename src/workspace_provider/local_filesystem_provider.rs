use std::{fs, path::PathBuf};

use super::workspace_provider::WorkspaceProvider;

pub struct LocalFilesystemProvider {
  workspace_root: PathBuf,
}

impl LocalFilesystemProvider {
  pub fn new(workspace_root: PathBuf) -> Self {
    return Self { workspace_root };
  }
}

impl WorkspaceProvider for LocalFilesystemProvider {
  fn get_contents(&self, key: String) -> String {
    let path = self.workspace_root.join(key);
    return fs::read_to_string(path).unwrap();
  }

  fn set_contents(&mut self, key: String, contents: String) {
    let path = self.workspace_root.join(key);
    fs::write(path, contents).unwrap();
  }
}
