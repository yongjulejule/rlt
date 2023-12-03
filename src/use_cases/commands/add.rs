use crate::adapters::{
  hasher::Hasher, object_manager::ObjectManager,
  workspace_provider::WorkspaceProvider,
};

// TODO
#[allow(dead_code)]
pub struct Add<'a> {
  pub root_path: &'a str,
  object_manager: &'a dyn ObjectManager,
  provider: &'a dyn WorkspaceProvider,
  hasher: &'a dyn Hasher,
}

#[cfg(test)]
mod tests {
  use log::info;

  use crate::adapters::workspace_provider::WorkspaceProvider;
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

    info!("execute add");
    // Add::new()::run();
    assert_eq!("todo!", "todo!")
  }
}
