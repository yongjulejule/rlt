pub struct Core {
  root: String,
  object_format: String,
}

impl Core {
  pub fn new(root: String) -> Self {
    return Self {
      root,
      object_format: "sha1".to_string(),
    };
  }

  pub fn get_object_key(&self, hash: &str) -> String {
    let mut key = self.root.clone();
    key.push_str("/objects/");
    key.push_str(&hash[0..2]);
    key.push_str("/");
    key.push_str(&hash[2..]);
    return key;
  }

  pub fn is_object_hash(&self, hash: &str) -> bool {
    // sha-1
    return hash.len() == 40
      && hash
        .chars()
        .all(|c| !c.is_ascii_uppercase() && c.is_ascii_hexdigit());
  }

  pub fn resolve_revision(&self, revision: &str) -> String {
    if self.is_object_hash(revision) {
      return revision.to_string();
    }
    return "not implemented".to_string();
  }
}

// set sha-1 or sha-256
// read object
// resolve revisions
// write object
// quiet ?
// bare
// resolve .git dir (bare || separate-git-dir || work-tree)

// set_object_format

/*
setup() {
  resolve .git dir
  resolve_core_path();
  set_object_format();

  set sha-1 or sha-256

}
 */

#[cfg(test)]
mod run_tests {
  use super::*;

  #[test]
  fn test_get_object_identifier() {
    let core = Core::new(".".to_string());
    assert_eq!(
      core.get_object_identifier("1234567890123456789012345678901234567890"),
      "./.git/objects/12/34567890123456789012345678901234567890"
    );
  }

  #[test]
  fn test_is_object_hash() {
    let core = Core::new(".".to_string());
    assert_eq!(
      core.is_object_hash("1234567890123456789012345678901234567890"),
      true
    );
    assert_eq!(
      core.is_object_hash("123456789012345678901234567890123456789"),
      false
    );
    assert_eq!(
      core.is_object_hash("12345678901234567890123456789012345678901"),
      false
    );
    assert_eq!(
      core.is_object_hash("123456789012345678901234567890123456789g"),
      false
    );
    assert_eq!(
      core.is_object_hash("123456789012345678901234567890123456789G"),
      false
    );
  }

  #[test]
  fn test_resolve_revision() {
    let core = Core::new(".".to_string());
    assert_eq!(
      core.resolve_revision("1234567890123456789012345678901234567890"),
      "1234567890123456789012345678901234567890"
    );
  }
}
