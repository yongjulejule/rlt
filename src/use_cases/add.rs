use crate::adapters::{
  hasher::Hasher, object_manager::ObjectManagement,
  workspace_provider::WorkspaceProvider,
};

pub struct Add<'a> {
  pub root_path: &'a str,
  object_manager: &'a dyn ObjectManagement,
  provider: &'a dyn WorkspaceProvider,
  hasher: &'a dyn Hasher,
}

#[cfg(test)]
mod tests {
  use crate::adapters::workspace_provider::WorkspaceProvider;
  use crate::infrastructures::memory_store::MemoryStore;
  use crate::infrastructures::test_content_provider::TestContentProvider;

  #[test]
  fn test_add() {
    // let execution_path = ".";
    let files = vec![
      ("test.txt", "test-body"),
      ("test/test.txt", "test-body-with-dir"),
    ];
    let mut provider = TestContentProvider::new();
    files.into_iter().for_each(|(k, v)| {
      provider.set_contents(k.to_string(), v.to_string());
    });
    // let store = MemoryStore::new();

    println!("execute add");
    // Add::new()::run();
    assert_eq!("todo!", "todo!")
  }
}
