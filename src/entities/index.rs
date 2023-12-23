use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Index {
  pub signature: String, // DIRC
  pub version: String,
  pub entries_count: u32,
  pub entries: BTreeMap<String, IndexEntry>,
  pub extensions: Vec<IndexExtension>,
  pub checksum: Vec<u8>,
}

impl Index {
  pub fn new() -> Self {
    let index = Index {
      signature: "DIRC".to_string(),
      version: "2".to_string(),
      entries_count: 0,
      entries: BTreeMap::new(),
      extensions: Vec::new(),
      checksum: Vec::new(),
    };
    index
  }
}

#[derive(Debug, Clone)]
pub struct IndexEntry {
  pub ctime: i64,
  pub ctime_nsec: i64,
  pub mtime: i64,
  pub mtime_nsec: i64,
  pub dev: u64,
  pub ino: u64,
  pub mode: u32,
  pub uid: u32,
  pub gid: u32,
  pub size: u64,
  pub hash: Vec<u8>,
  pub flags: u16,
  pub name: String,
}

#[derive(Debug, Clone)]
pub struct IndexExtension {
  pub signature: String,
  pub size: u32,
  pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_index_new() {
    let index = Index::new();
    assert_eq!(index.signature, "DIRC");
    assert_eq!(index.version, "2");
    assert_eq!(index.entries_count, 0);
    assert_eq!(index.entries.len(), 0);
    assert_eq!(index.extensions.len(), 0);
    assert_eq!(index.checksum.len(), 0);
  }
}
