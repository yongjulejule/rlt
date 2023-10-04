use std::collections::HashMap;

use super::workspace_provider::WorkspaceProvider;

pub struct TestContentProvider {
  contents: HashMap<String, String>,
}

#[allow(dead_code)]
impl TestContentProvider {
  pub fn new() -> Self {
    return Self {
      contents: HashMap::new(),
    };
  }
}

impl WorkspaceProvider for TestContentProvider {
  fn get_contents(&self, key: String) -> String {
    return self
      .contents
      .get(&key)
      .unwrap_or(&"".to_string())
      .to_string();
  }

  fn set_contents(&mut self, key: String, contents: String) {
    self.contents.insert(key, contents);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_contents() {
    let mut provider = TestContentProvider::new();
    provider.set_contents("foo".to_string(), "bar".to_string());
    assert_eq!(provider.get_contents("foo".to_string()), "bar".to_string());
  }

  #[test]
  fn test_set_contents() {
    let mut provider = TestContentProvider::new();
    provider.set_contents("foo".to_string(), "bar".to_string());
    assert_eq!(provider.get_contents("foo".to_string()), "bar".to_string());
  }
}
