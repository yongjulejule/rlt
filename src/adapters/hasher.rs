use sha1::Sha1;
use sha2::{Digest, Sha256};

pub struct HasherFactory {}

impl HasherFactory {
  pub fn new() -> Self {
    return Self {};
  }

  pub fn get_hasher(&self, strategy: String) -> Box<dyn Hasher> {
    match strategy.as_str() {
      "sha256" => Box::new(Sha256Hasher::new()),
      "sha1" => Box::new(Sha1Hasher::new()),
      _ => panic!("Unknown hash strategy"),
    }
  }
}

pub trait Hasher {
  fn hash(&self, data: &[u8]) -> String;
}

pub struct Sha256Hasher {}

impl Sha256Hasher {
  pub fn new() -> Self {
    return Self {};
  }
}

impl Hasher for Sha256Hasher {
  fn hash(&self, data: &[u8]) -> String {
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
  fn hash(&self, data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    return format!("{:x}", result);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sha256_hasher() {
    let hasher = Sha256Hasher::new();
    let result = hasher.hash(b"test");
    assert_eq!(
      result,
      "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
    );
  }

  #[test]
  fn test_sha1_hasher() {
    let hasher = Sha1Hasher::new();
    let result = hasher.hash(b"test");
    assert_eq!(result, "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3");
  }

  #[test]
  fn test_binary_hash() {
    let hasher = Sha1Hasher::new();
    let result = hasher.hash(b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09");
    assert_eq!(result, "494179714a6cd627239dfededf2de9ef994caf03");
  }
}
