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
