use std::{collections::BTreeMap, fs::DirEntry, path::Path};

use crate::{
  adapters::{
    data_store::DataStore, filesystem_utils::visit_dirs, hasher,
    workspace_provider::WorkspaceProvider,
  },
  use_cases::{
    commands::ls_tree::{LsTree, LsTreeOptions},
    core::{
      ignore_service::IgnoreService,
      index_service::{IndexService, IndexServiceImpl},
      object_service::ObjectService,
      revision_service::RevisionService,
    },
  },
};

pub struct Status<'a> {
  store: &'a dyn DataStore,
  provider: &'a dyn WorkspaceProvider,
  ignore_service: &'a dyn IgnoreService,
  object_service: &'a dyn ObjectService,
  revision_service: &'a dyn RevisionService,
}

#[derive(Debug)]
pub struct StatusResult {
  pub staged: Vec<(String, String)>,
  pub unstaged: Vec<(String, String)>,
  pub untracked: Vec<String>,
}

impl<'a> Status<'a> {
  pub fn new(
    store: &'a dyn DataStore,
    provider: &'a dyn WorkspaceProvider,
    ignore_service: &'a dyn IgnoreService,
    object_service: &'a dyn ObjectService,
    revision_service: &'a dyn RevisionService,
  ) -> Self {
    Self {
      store,
      provider,
      ignore_service,
      object_service,
      revision_service,
    }
  }

  pub fn run(&self) -> Result<StatusResult, String> {
    let mut local_file = BTreeMap::new();
    let raw_data = self.store.read("index").map_err(|_| "Fail to read")?;
    let staged_entries = IndexServiceImpl::from_raw(&raw_data)?
      .get_index()
      .clone()
      .entries;
    let mut untracked = vec![];

    let cb = &mut |entry: &DirEntry| {
      let path = entry.path();
      let path_str = path.to_str().unwrap();
      if self.ignore_service.is_ignored(path_str)
        || path_str.starts_with("./.git/")
      {
        return;
      }
      if !staged_entries.contains_key(&path_str[2..]) {
        untracked.push(path_str.to_string());
        return;
      }
      let content = self.provider.get_contents(path_str.to_string()).unwrap();
      local_file.insert(
        path.to_str().unwrap()[2..].to_string(),
        self.object_service.create_key("blob", &content),
      );
    };

    let workspace_root = Path::new(".");
    let _ = visit_dirs(workspace_root, cb);

    let commited = LsTree::new(
      self.object_service,
      self.revision_service,
      LsTreeOptions {
        recurse: true,
        tree_ish: "HEAD".to_string(),
        path: [].to_vec(),
      },
    )
    .run()?;

    let mut unstaged = vec![];
    let mut staged = vec![];
    commited.iter().for_each(|e| {
      if !staged_entries.contains_key(e.name.as_str()) {
        staged.push(("deleted".to_string(), e.name.to_string()));
        return;
      }
      let object_hash_raw: Vec<u8> = staged_entries
        .get(e.name.as_str())
        .unwrap()
        .hash
        .clone()
        .into();

      let object_hash = object_hash_raw
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

      if object_hash != e.hash.as_str() {
        staged.push(("modified".to_string(), e.name.to_string()));
      }
    });

    println!("commited {:?}", commited);
    println!("local {:?}", local_file);

    staged_entries.iter().for_each(|(k, v)| {
      println!("k: {}, v: {:?}", k, v);
      if !commited.iter().any(|e| e.name == k.as_str()) {
        staged.push(("new file".to_string(), k.to_string()));
      }
      if !local_file.contains_key(k.as_str()) {
        unstaged.push(("deleted".to_string(), k.to_string()));
        return;
      }
      let object_hash_raw: Vec<u8> = v.hash.clone().into();
      let object_hash = object_hash_raw
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

      if object_hash != local_file.get(k.as_str()).unwrap().as_str() {
        println!("lo {}", local_file.get(k.as_str()).unwrap().as_str());
        println!("ob {}", object_hash);
        unstaged.push(("modified".to_string(), k.to_string()));
      }

      local_file.remove(k.as_str());
    });
    println!("local_file: {:?}", local_file);

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
    provider.set_contents("./test.txt".to_string(), b"test content\n");
    provider.set_contents("./ignored/test2.txt".to_string(), b"test content\n");
    //    let status =
    //      Status::new(store.as_ref(), provider.as_ref(), &ignore_service);
    //    let result = status.run();
    //    assert!(result.is_ok());
  }
}
