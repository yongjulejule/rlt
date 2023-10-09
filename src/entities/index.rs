#[derive(Debug)]
pub struct Index {
  signature: String, // DIRC
  version: String,
  entries_count: u32,
  entries: Vec<IndexEntry>,
  extensions: Vec<IndexExtension>,
  checksum: Vec<u8>,
}

#[derive(Debug)]
struct IndexEntry {
  ctime: u32,
  ctime_nsec: u32,
  mtime: u32,
  mtime_nsec: u32,
  dev: u32,
  ino: u32,
  mode: u32,
  uid: u32,
  gid: u32,
  size: u32,
  hash: String,
  flags: u16,
  name: String,
}

#[derive(Debug)]
struct IndexExtension {
  signature: String,
  size: u32,
  data: Vec<u8>,
}
