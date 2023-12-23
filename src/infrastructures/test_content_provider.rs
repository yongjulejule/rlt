use std::collections::HashMap;

use crate::adapters::workspace_provider::WorkspaceProvider;

pub struct TestContentProvider {
  contents: HashMap<String, Vec<u8>>,
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
  fn get_contents(&self, key: String) -> Result<Vec<u8>, String> {
    return Ok(self.contents.get(&key).map(|v| v.to_vec()).unwrap());
  }

  fn set_contents(
    &mut self,
    key: String,
    contents: &[u8],
  ) -> Result<(), String> {
    self.contents.insert(key, contents.to_vec());
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_contents() {
    let mut provider = TestContentProvider::new();
    provider.set_contents("foo".to_string(), b"bar");
    assert_eq!(provider.get_contents("foo".to_string()).unwrap(), b"bar");
  }

  #[test]
  fn test_set_contents() {
    let mut provider = TestContentProvider::new();
    provider.set_contents("foo".to_string(), b"bar");
    assert_eq!(provider.get_contents("foo".to_string()).unwrap(), b"bar");
  }
}
