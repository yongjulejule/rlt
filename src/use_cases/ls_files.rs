use crate::adapters::data_store::DataStore;

use super::index_service::{IndexService, IndexServiceImpl};

pub struct LsFiles<'a> {
  store: &'a dyn DataStore,
}

impl<'a> LsFiles<'a> {
  pub fn new(store: &'a dyn DataStore) -> Self {
    return Self { store };
  }
  pub fn run(&self) -> Result<Vec<String>, String> {
    let raw_data = self.store.read("index").map_err(|_| "Fail to read")?;
    let index = IndexServiceImpl::from_raw(&raw_data)?.get_index().clone();
    Ok(index.entries.keys().cloned().collect())
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    infrastructures::file_store::FileStore, use_cases::ls_files::LsFiles,
  };

  const TEST_INDEX: &str = "./test/fixtures/index";

  #[test]
  fn test_ls_files() {
    // let execution_path = ".";
    let store =
      Box::new(FileStore::new(TEST_INDEX.rsplit_once("/").unwrap().0));

    let ls_files = LsFiles::new(store.as_ref());
    let result: Vec<String> = ls_files.run().unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "a/b".to_string());
  }
}
