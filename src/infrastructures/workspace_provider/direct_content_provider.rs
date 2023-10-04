use std::collections::HashMap;

use super::workspace_provider::WorkspaceProvider;

pub struct DirectContentProvider {
  pub contents: HashMap<String, String>,
}

#[allow(dead_code)]
impl DirectContentProvider {
  pub fn new() -> Self {
    return Self {
      contents: HashMap::new(),
    };
  }
}

impl WorkspaceProvider for DirectContentProvider {
  fn get_contents(&self, key: String) -> String {
    return self.contents.get(&key).unwrap().to_string();
  }

  fn set_contents(&mut self, key: String, contents: String) {
    self.contents.insert(key, contents);
  }
}
