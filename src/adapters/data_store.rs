pub trait DataStore {
  fn read(&self, key: &str) -> Result<Vec<u8>, String>;
  fn exists(&self, key: &str) -> Result<bool, String>;
  fn write(&self, key: &str, data: &[u8]) -> Result<(), String>;
}
