pub trait DataStore {
  fn read(&self, key: &str) -> Result<Vec<u8>, std::io::Error>;
  fn write(&self, key: &str, data: &[u8]) -> Result<(), std::io::Error>;
}
