use std::{collections::BTreeMap, fs::DirEntry, path::Path};

use log::error;

use crate::{
  adapters::{
    data_store::DataStore, filesystem_utils::visit_dirs,
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
    let raw_data = self
      .store
      .read("index")
      .map_err(|_| "Fail to read index file")?;
    let staged_entries = IndexServiceImpl::from_raw(&raw_data)?
      .get_index()
      .clone()
      .entries;
    let mut untracked = vec![];

    let cb = &mut |entry: &DirEntry| {
      let path = entry.path();
      let path_str = match path.to_str() {
        Some(s) => s,
        None => {
          error!("Fail to convert path to string");
          return;
        }
      };
      if self.ignore_service.is_ignored(path_str)
        || path_str.starts_with("./.git/")
      {
        return;
      }
      if !staged_entries.contains_key(&path_str[2..]) {
        untracked.push(path_str.to_string());
        return;
      }
      let content = self.provider.get_contents(path_str.to_string());
      if content.is_err() {
        error!("Fail to get contents from path: {}", path_str);
        return;
      }

      local_file.insert(
        path.to_str().unwrap()[2..].to_string(),
        self.object_service.create_key("blob", &content.unwrap()),
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

    staged_entries.iter().for_each(|(k, v)| {
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
        unstaged.push(("modified".to_string(), k.to_string()));
      }

      local_file.remove(k.as_str());
    });

    Ok(StatusResult {
      staged,
      unstaged,
      untracked,
    })
  }
}

#[cfg(test)]
mod tests {

  use log::{debug, trace};

  use crate::{
    adapters::{hasher, object_manager::ObjectManagerImpl},
    infrastructures::{
      memory_store::MemoryStore, test_content_provider::TestContentProvider,
    },
    use_cases::core::{
      ignore_service::IgnoreServiceImpl, object_service::ObjectServiceImpl,
      revision_service::RevisionServiceImpl,
    },
  };

  use super::*;

  #[test]
  fn test_status() {
    let store = Box::new(MemoryStore::new());
    let object_manager = ObjectManagerImpl::new(store.as_ref());
    let provider = Box::new(TestContentProvider::new());
    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());
    let object_service =
      ObjectServiceImpl::new(&object_manager, hasher.as_ref());
    let ignore_service = IgnoreServiceImpl::from_raw(&[]).unwrap();
    let revision_service = RevisionServiceImpl::new(store.as_ref());

    let status = Status::new(
      store.as_ref(),
      provider.as_ref(),
      &ignore_service,
      &object_service,
      &revision_service,
    );

    let result = status.run();
    println!("result: {:?}", result);
    assert_eq!(result.is_err(), true); // TODO!
  }
}
