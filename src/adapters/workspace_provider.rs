pub trait WorkspaceProvider {
  fn get_contents(&self, key: String) -> String;
  fn set_contents(&mut self, key: String, contents: String);
}
