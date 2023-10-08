#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
  pub object_type: String,
  pub hash: String,
  pub data: Vec<u8>,
  pub size: usize,
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
