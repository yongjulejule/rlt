pub trait WorkspaceProvider {
  fn get_contents(&self, key: String) -> Result<Vec<u8>, String>;
  fn set_contents(
    &mut self,
    key: String,
    contents: &[u8],
  ) -> Result<(), String>;
}
