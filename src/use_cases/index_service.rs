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
    // convert [00, 00, 00, 02] -> 2
    let version =
      u32::from_be_bytes(data[4..8].try_into().unwrap()).to_string();
    let entries_count = u32::from_be_bytes(data[8..12].try_into().unwrap());

    let mut start = 12;
    for _i in 0..entries_count {
      let ctime =
        i32::from_be_bytes(data[start..start + 4].try_into().unwrap());
      let ctime_nsec =
        i32::from_be_bytes(data[start + 4..start + 8].try_into().unwrap());
      let mtime =
        i32::from_be_bytes(data[start + 8..start + 12].try_into().unwrap());
      let mtime_nsec =
        i32::from_be_bytes(data[start + 12..start + 16].try_into().unwrap());
      let dev =
        u32::from_be_bytes(data[start + 16..start + 20].try_into().unwrap());
      let ino =
        u32::from_be_bytes(data[start + 20..start + 24].try_into().unwrap());
      let mode =
        u32::from_be_bytes(data[start + 24..start + 28].try_into().unwrap());
      //  check validity later
      // valid mode: 0000 0000 0000 0000 {1000 | 1010 | 1110} 000{111101101(0755) | 110100100(0644) | 000000000(links)}
      let uid =
        u32::from_be_bytes(data[start + 28..start + 32].try_into().unwrap());
      let gid =
        u32::from_be_bytes(data[start + 32..start + 36].try_into().unwrap());
      let size =
        u32::from_be_bytes(data[start + 36..start + 40].try_into().unwrap());
      let hash =
        String::from_utf8_lossy(&data[start + 40..start + 60]).to_string();
      let flags =
        u16::from_be_bytes(data[start + 60..start + 62].try_into().unwrap());

      let name_length = if (flags & 0xfff) >= 0xfff {
        data[start + 62..]
          .iter()
          .position(|&x| x == 0)
          .ok_or("Fail")?
          - start
          + 62
      } else {
        (flags & 0xfff).into()
      };

      let name = String::from_utf8_lossy(
        &data[start + 62..start + 62 + name_length as usize],
      )
      .to_string();

      // let name =
      //   String::from_utf8_lossy(&data[start + 62..start + 72]).to_string();

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
      start += 62 + name_length as usize;
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

  fn serialize(&self) -> Result<String, String> {
    todo!()
  }

  fn deserialize(&self) -> Result<Index, String> {
    todo!()
  }
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
