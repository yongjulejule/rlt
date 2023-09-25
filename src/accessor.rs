/*
use std::io::Error;

use crate::data_store::data_store::DataStore;

pub trait Accessor {
  fn write(&self, key: &str, data: &[u8]) -> Result<(), Error>;
  fn read(&self, key: &str) -> Result<Vec<u8>, Error>;
  fn new(store: &dyn DataStore, root: String) -> Self;
}

struct ConfigAccessor<'a> {
  store: &'a dyn DataStore,
  root: String,
}
struct ObjectAccessor<'a> {
  store: &'a dyn DataStore,
  root: String,
}
struct RefAccessor<'a> {
  store: &'a dyn DataStore,
  root: String,
}

impl<'a> Accessor for ConfigAccessor<'a> {
  fn new(store: &dyn DataStore, root: String) -> Self {
    let root = format!("{}/", root);
    return Self { store, root };
  }
  fn write(&self, key: &str, data: &[u8]) -> Result<(), Error> {
    return self.store.write(key, data);
  }
  fn read(&self, key: &str) -> Result<Vec<u8>, Error> {
    return self.store.read(key);
  }
}

impl<'a> Accessor for ObjectAccessor<'a> {
  fn new(store: &dyn DataStore, root: String) -> Self {
    let root = format!("{}/objects", root);
    return Self { store, root };
  }
  fn write(&self, key: &str, data: &[u8]) -> Result<(), Error> {
    return self.store.write(key, data);
  }
  fn read(&self, key: &str) -> Result<Vec<u8>, Error> {
    return self.store.read(key);
  }
}
impl<'a> Accessor for RefAccessor<'a> {
  fn new(store: &dyn DataStore, root: String) -> Self {
    let root = format!("{}/refs", root);
    return Self { store, root };
  }
  fn write(&self, key: &str, data: &[u8]) -> Result<(), Error> {
    return self.store.write(key, data);
  }
  fn read(&self, key: &str) -> Result<Vec<u8>, Error> {
    return self.store.read(key);
  }
}

*/
