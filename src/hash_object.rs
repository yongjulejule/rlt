use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::{fs::OpenOptions, io};

use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

use crate::data_store::data_store::DataStore;

pub struct HashObject {
  store: Box<dyn DataStore>,
  hash_strategy: String,
  write: bool,
  object_type: String,
  from_stdin: bool,
  path: Vec<String>,
}

impl HashObject {
  pub fn new(
    store: Box<dyn DataStore>,
    hash_strategy: String,
    write: bool,
    object_type: String,
    from_stdin: bool,
    path: Vec<String>,
  ) -> Self {
    return Self {
      store,
      hash_strategy,
      write,
      object_type,
      from_stdin,
      path,
    };
  }

  pub fn run(&self) -> Result<(), i32> {
    if self.from_stdin {
      let mut content = String::new();

      // todo : Encapsulate stdin
      println!("Enter blob content: ");

      io::stdin()
        .read_line(&mut content)
        .expect("failed to read from stdin");
      content
    } else {
      self.path.iter().for_each(|p| {
        // hash with object type & content in path
        let content = fs::read(p).expect("failed to read file");
        let hash = hash(&content);

        println!("{} {}, type: {}", hash, p, self.object_type);
      });
      return Ok(());
    };
    return Ok(());
  }
}

fn hash(content: &[u8]) -> String {
  let mut hasher = Sha1::new();
  // todo : Encapsulate hasher && support sha256
  hasher.update(content);
  return hasher
    .finalize()
    .iter()
    .map(|b| format!("{:02x}", b))
    .collect();
}
