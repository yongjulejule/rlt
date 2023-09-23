use sha1::Sha1;
use sha2::{Digest, Sha256};

pub trait Hasher {
  fn hash(&self, data: &String) -> String;
}

pub struct Sha256Hasher {}

impl Sha256Hasher {
  pub fn new() -> Self {
    return Self {};
  }
}

impl Hasher for Sha256Hasher {
  fn hash(&self, data: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    return format!("{:x}", result);
  }
}

pub struct Sha1Hasher {}

impl Sha1Hasher {
  pub fn new() -> Self {
    return Self {};
  }
}

impl Hasher for Sha1Hasher {
  fn hash(&self, data: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    return format!("{:x}", result);
  }
}
