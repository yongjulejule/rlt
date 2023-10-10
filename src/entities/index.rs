#[derive(Debug, Clone)]
pub struct Index {
  pub signature: String, // DIRC
  pub version: String,
  pub entries_count: u32,
  pub entries: Vec<IndexEntry>,
  pub extensions: Vec<IndexExtension>,
  pub checksum: Vec<u8>,
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
  pub hash: String,
  pub flags: u16,
  pub name: String,
}

#[derive(Debug, Clone)]
struct IndexExtension {
  pub signature: String,
  pub size: u32,
  pub data: Vec<u8>,
}
