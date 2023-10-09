#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
  pub object_type: String,
  pub hash: String,
  pub data: Vec<u8>,
  pub size: usize,
}

impl Object {
  pub fn new(object_type: &str, hash: &str, data: &[u8], size: usize) -> Self {
    Self {
      object_type: object_type.to_string(),
      hash: hash.to_string(),
      data: data.to_vec(),
      size,
    }
  }
}

pub struct BlobObject {
  pub object_type: String,
  pub data: Vec<u8>,
  pub hash: String,
  pub size: usize,
}

pub struct TreeObject {
  pub object_type: String,
  pub hash: String,
  pub entries: Vec<TreeElement>,
  pub size: usize,
}

pub struct TreeElement {
  pub mode: String,
  pub name: String,
  pub hash: String,
}

pub struct CommitObject {
  pub object_type: String,
  pub hash: String,
  pub tree: String,
  pub parents: Vec<String>,
  pub author: String,
  pub committer: String,
  pub message: String,
  pub size: usize,
}
