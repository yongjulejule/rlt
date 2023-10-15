use std::{collections::BTreeMap, os::unix::prelude::MetadataExt};

use crate::entities::index::{Index, IndexEntry};

pub trait IndexService {
  fn get_index(&self) -> &Index;
  fn save_entry(&mut self, entry: IndexEntry);
  fn delete_entry(&mut self, key: &str) -> Result<(), String>;
  fn update_extension(&self, extension_type: &str) -> Result<String, String>;
  fn delete_extension(&self) -> Result<(), String>;
  fn delete(&self) -> Result<(), String>;
  fn create_entry_from_file(
    &self,
    key: &str,
    file_path: &str,
  ) -> Result<IndexEntry, String>;
}

#[derive(Debug, Clone)]
pub struct IndexServiceImpl {
  index: Index,
}

impl IndexServiceImpl {
  pub fn new() -> Self {
    let index = Index::new();
    Self { index }
  }

  pub fn from_index(index: Index) -> Self {
    Self { index }
  }

  pub fn from_raw(data: &[u8]) -> Result<Self, String> {
    let mut entries: BTreeMap<String, IndexEntry> = BTreeMap::new();
    let signature = String::from_utf8_lossy(&data[0..4]).to_string();
    let version =
      u32::from_be_bytes(data[4..8].try_into().unwrap()).to_string();
    let entries_count = u32::from_be_bytes(data[8..12].try_into().unwrap());
    println!("entries_count: {}", entries_count);

    let mut start = 12;
    for _i in 0..entries_count {
      let ctime = parse_i32(&data, start, CTIME_OFFSET)?;
      let ctime_nsec = parse_i32(&data, start, CTIME_NSEC_OFFSET)?;
      let mtime = parse_i32(&data, start, MTIME_OFFSET)?;
      let mtime_nsec = parse_i32(&data, start, MTIME_NSEC_OFFSET)?;
      let dev = parse_u32(&data, start, DEV_OFFSET)?;
      let ino = parse_u32(&data, start, INO_OFFSET)?;
      let mode = parse_u32(&data, start, MODE_OFFSET)?;
      //  check validity later
      // valid mode: 0000 0000 0000 0000 {1000 | 1010 | 1110} 000{111101101(0755) | 110100100(0644) | 000000000(links)}
      let uid = parse_u32(&data, start, UID_OFFSET)?;
      let gid = parse_u32(&data, start, GID_OFFSET)?;
      let size = parse_u32(&data, start, SIZE_OFFSET)?;

      let hash = String::from_utf8_lossy(
        &data[start + HASH_OFFSET..start + HASH_OFFSET + 20],
      )
      .to_string();
      let flags = parse_u16(&data, start, FLAGS_OFFSET)?;

      let name_length = if (flags & 0xfff) >= 0xfff {
        data[start + NAME_OFFSET..]
          .iter()
          .position(|&x| x == 0)
          .ok_or("Invalid file length")?
          - start
          + NAME_OFFSET
      } else {
        (flags & 0xfff).into()
      };
      println!("name_length: {}", name_length);
      let name = String::from_utf8_lossy(
        &data[start + NAME_OFFSET..start + NAME_OFFSET + name_length as usize],
      )
      .to_string();
      println!("name: {}", name);

      let entry = IndexEntry {
        ctime: ctime as i64,
        ctime_nsec: ctime_nsec as i64,
        mtime: mtime as i64,
        mtime_nsec: mtime_nsec as i64,
        dev: dev as u64,
        ino: ino as u64,
        mode,
        uid,
        gid,
        size: size as u64,
        hash,
        flags,
        name,
      };
      entries.insert(entry.name.clone(), entry.clone());
      let pad = if ENTRY_FIXED_SIZE + name_length as usize % 8 == 0 {
        0
      } else {
        8 - (ENTRY_FIXED_SIZE + name_length as usize) % 8
      };
      start += ENTRY_FIXED_SIZE + name_length as usize + pad;
    }

    Ok(Self {
      index: Index {
        signature,
        version,
        entries_count,
        entries,
        extensions: Vec::new(),
        checksum: data[(data.len() - 20 + 1)..].to_vec(),
      },
    })
  }
}

const ENTRY_START_OFFSET: usize = 12;
const CTIME_OFFSET: usize = 0;
const CTIME_NSEC_OFFSET: usize = 4;
const MTIME_OFFSET: usize = 8;
const MTIME_NSEC_OFFSET: usize = 12;
const DEV_OFFSET: usize = 16;
const INO_OFFSET: usize = 20;
const MODE_OFFSET: usize = 24;
const UID_OFFSET: usize = 28;
const GID_OFFSET: usize = 32;
const SIZE_OFFSET: usize = 36;
const HASH_OFFSET: usize = 40;
const FLAGS_OFFSET: usize = 60;
const NAME_OFFSET: usize = 62;
const ENTRY_FIXED_SIZE: usize = 62;

fn parse_i32(data: &[u8], start: usize, offset: usize) -> Result<i32, String> {
  data[start + offset..start + offset + 4]
    .try_into()
    .map(i32::from_be_bytes)
    .map_err(|_| "Failed to parse i32".to_string())
}

fn parse_u32(data: &[u8], start: usize, offset: usize) -> Result<u32, String> {
  data[start + offset..start + offset + 4]
    .try_into()
    .map(u32::from_be_bytes)
    .map_err(|_| "Failed to parse u32".to_string())
}

fn parse_u16(data: &[u8], start: usize, offset: usize) -> Result<u16, String> {
  data[start + offset..start + offset + 2]
    .try_into()
    .map(u16::from_be_bytes)
    .map_err(|_| "Failed to parse u16".to_string())
}

impl IndexService for IndexServiceImpl {
  fn get_index(&self) -> &Index {
    &self.index
  }

  fn create_entry_from_file(
    &self,
    key: &str,
    file_path: &str,
  ) -> Result<IndexEntry, String> {
    let metadata =
      std::fs::metadata(file_path).map_err(|_| "Fail to get metadata")?;
    let entry = IndexEntry {
      ctime: metadata.ctime(),
      ctime_nsec: metadata.ctime_nsec(),
      mtime: metadata.mtime(),
      mtime_nsec: metadata.mtime_nsec(),
      dev: metadata.dev(),
      ino: metadata.ino(),
      mode: metadata.mode(),
      uid: metadata.uid(),
      gid: metadata.gid(),
      size: metadata.size(),
      hash: key.to_string(),
      flags: 0,
      name: file_path.to_string(),
    };
    Ok(entry)
  }

  fn save_entry(&mut self, entry: IndexEntry) {
    self.index.entries.insert(entry.name.clone(), entry);
    self.index.entries_count += 1;
  }

  fn delete_entry(&mut self, key: &str) -> Result<(), String> {
    self.index.entries.remove(key);
    self.index.entries_count -= 1;
    Ok(())
  }

  fn update_extension(&self, extension_type: &str) -> Result<String, String> {
    todo!()
  }

  fn delete_extension(&self) -> Result<(), String> {
    todo!()
  }

  fn delete(&self) -> Result<(), String> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use std::fs;

  use crate::use_cases::index_service::{IndexService, IndexServiceImpl};

  const TEST_INDEX: &str = "./test/fixtures/index";

  #[test]
  fn test_entry_creation() {
    let index_service = IndexServiceImpl::new();

    let entry = index_service.create_entry_from_file("test-key", TEST_INDEX);

    println!("{:?}", entry);
    assert_eq!(entry.is_ok(), true);
    assert_eq!(entry.unwrap().name, TEST_INDEX);
  }

  #[test]
  fn test_entry_save() {
    let mut index_service = IndexServiceImpl::new();
    let entry = index_service.create_entry_from_file("test-key", TEST_INDEX);
    index_service.save_entry(entry.unwrap());

    assert_eq!(index_service.get_index().entries_count, 1);
    assert_eq!(
      index_service.get_index().entries.contains_key(TEST_INDEX),
      true
    );
  }

  #[test]
  fn test_index_from_raw() {
    let raw = fs::read(TEST_INDEX).unwrap();

    let index_service = IndexServiceImpl::from_raw(&raw).unwrap();
    let index = index_service.get_index();

    println!("{:?}", index);

    assert_eq!(index.entries_count, 1);
    assert_eq!(index.entries.get("a/b").is_some(), true);
    assert_eq!(index.checksum, &raw[(raw.len() - 20 + 1)..]);
  }
}
