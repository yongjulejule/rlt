pub trait DataStore {
  fn read(&self, key: &str) -> Result<Vec<u8>, std::io::Error>;
  fn exists(&self, key: &str) -> bool;
  fn write(&self, key: &str, data: &[u8]) -> Result<(), std::io::Error>;
}
