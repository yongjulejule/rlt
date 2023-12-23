use std::{fs::DirEntry, path::Path};

use crate::{
  adapters::{
    data_store::DataStore, filesystem_utils::visit_dirs,
    workspace_provider::WorkspaceProvider,
  },
  use_cases::core::ignore_service::IgnoreService,
};

pub struct Status<'a> {
  store: &'a dyn DataStore,
  provider: &'a dyn WorkspaceProvider,
  ignore_service: &'a dyn IgnoreService,
}

#[derive(Debug)]
pub struct StatusResult {
  pub staged: Vec<String>,
  pub unstaged: Vec<String>,
  pub untracked: Vec<String>,
}

impl<'a> Status<'a> {
  pub fn new(
    store: &'a dyn DataStore,
    provider: &'a dyn WorkspaceProvider,
    ignore_service: &'a dyn IgnoreService,
  ) -> Self {
    Self {
      store,
      provider,
      ignore_service,
    }
  }

  pub fn run(&self) -> Result<StatusResult, String> {
    let mut staged = vec![];
    let mut unstaged = vec![];
    let mut untracked = vec![];

    let cb = &mut |entry: &DirEntry| {
      let path = entry.path();
      let path_str = path.to_str().unwrap();
      if self.ignore_service.is_ignored(path_str) || path_str.contains(".git") {
        return;
      }
      untracked.push(path.to_str().unwrap().to_string());
    };

    let workspace_root = Path::new("./");

    let _ = visit_dirs(workspace_root, cb);
    println!("untracked: {:?}", untracked);

    Ok(StatusResult {
      staged,
      unstaged,
      untracked,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    infrastructures::{
      memory_store::MemoryStore, test_content_provider::TestContentProvider,
    },
    use_cases::core::ignore_service::IgnoreServiceImpl,
  };

  use super::*;

  #[test]
  fn test_status() {
    let memory_store = MemoryStore::new();
    let store: Box<dyn DataStore> = Box::new(memory_store);
    let ignore_service = IgnoreServiceImpl::from_raw(b"ignored\n").unwrap();
    let mut provider: Box<dyn WorkspaceProvider> =
      Box::new(TestContentProvider::new());
    provider
      .set_contents("./test.txt".to_string(), "test content\n".to_string());
    provider.set_contents(
      "./ignored/test2.txt".to_string(),
      "test content\n".to_string(),
    );
    let status =
      Status::new(store.as_ref(), provider.as_ref(), &ignore_service);
    let result = status.run();
    assert!(result.is_ok());
  }
}
