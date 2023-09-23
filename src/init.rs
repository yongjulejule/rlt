use crate::data_store::data_store::DataStore;

const CONFIG_PATH: &str = "config";
const DEFAULT_CONFIG_CONTENT: &str = "[core]
repositoryformatversion = 0
filemode = true
bare = false
logallrefupdates = true
ignorecase = true
precomposeunicode = true";

const HEAD_PATH: &str = "HEAD";
const DEFAULT_HEAD_CONTENT: &str = "ref: refs/heads/main";

const DESCRIPTION_PATH: &str = "description";
const DEFAULT_DESCRIPTION_CONTENT: &str =
  "Unnamed repository; edit this file 'description' to name the repository.";

const INFO_EXCLUDE_PATH: &str = "info/exclude";
const DEFAULT_INFO_EXCLUDE_CONTENT: &str = "";

const OBJECTS_PATH: &str = "objects/";
const DEFAULT_OBJECTS_CONTENT: &str = "";

const OBJECTS_INFO_PATH: &str = "objects/info";
const DEFAULT_OBJECTS_INFO_CONTENT: &str = "";

const OBJECTS_PACK_PATH: &str = "objects/pack";
const DEFAULT_OBJECTS_PACK_CONTENT: &str = "";

const REFS_PATH: &str = "refs/";
const DEFAULT_REFS_CONTENT: &str = "";

const REFS_HEADS_PATH: &str = "refs/heads";
const DEFAULT_REFS_HEADS_CONTENT: &str = "";

const REFS_TAGS_PATH: &str = "refs/tags";
const DEFAULT_REFS_TAGS_CONTENT: &str = "";

pub fn run(store: &Box<dyn DataStore> /* , path: String */) {
  {
    let data = DEFAULT_CONFIG_CONTENT.as_bytes();
    store.write(CONFIG_PATH, data).expect("write config");
  }
  {
    let data = DEFAULT_HEAD_CONTENT.as_bytes();
    store.write(HEAD_PATH, data).expect("write HEAD");
  }
  {
    let data = DEFAULT_DESCRIPTION_CONTENT.as_bytes();
    store
      .write(DESCRIPTION_PATH, data)
      .expect("write description");
  }
  {
    let data = DEFAULT_INFO_EXCLUDE_CONTENT.as_bytes();
    store
      .write(INFO_EXCLUDE_PATH, data)
      .expect("write info/exclude");
  }
  {
    let data = DEFAULT_OBJECTS_CONTENT.as_bytes();
    store.write(OBJECTS_PATH, data).expect("write objects");
  }
  {
    let data = DEFAULT_OBJECTS_INFO_CONTENT.as_bytes();
    store
      .write(OBJECTS_INFO_PATH, data)
      .expect("write objects/info");
  }
  {
    let data = DEFAULT_OBJECTS_PACK_CONTENT.as_bytes();
    store
      .write(OBJECTS_PACK_PATH, data)
      .expect("write objects/pack");
  }
  {
    let data = DEFAULT_REFS_CONTENT.as_bytes();
    store.write(REFS_PATH, data).expect("write refs");
  }
  {
    let data = DEFAULT_REFS_HEADS_CONTENT.as_bytes();
    store
      .write(REFS_HEADS_PATH, data)
      .expect("write refs/heads");
  }
  {
    let data = DEFAULT_REFS_TAGS_CONTENT.as_bytes();
    store.write(REFS_TAGS_PATH, data).expect("write refs/tags");
  }
}

#[cfg(test)]
mod run_tests {
  use super::*;
  use crate::data_store::{file_store::FileStore, memory_store::MemoryStore};

  #[test]
  fn test_init() {
    // let store: Box<dyn DataStore> = Box::new(MemoryStore::new());
    let store: Box<dyn DataStore> = Box::new(FileStore::new("/tmp/.git"));
    run(&store);

    assert_eq!(
      store.read(CONFIG_PATH).unwrap(),
      DEFAULT_CONFIG_CONTENT.as_bytes()
    );
    assert_eq!(
      store.read(HEAD_PATH).unwrap(),
      DEFAULT_HEAD_CONTENT.as_bytes()
    );
    assert_eq!(
      store.read(DESCRIPTION_PATH).unwrap(),
      DEFAULT_DESCRIPTION_CONTENT.as_bytes()
    );
    assert_eq!(
      store.read(INFO_EXCLUDE_PATH).unwrap(),
      DEFAULT_INFO_EXCLUDE_CONTENT.as_bytes()
    );
    assert_eq!(
      store.read(OBJECTS_PATH).unwrap(),
      DEFAULT_OBJECTS_CONTENT.as_bytes()
    );
    assert_eq!(
      store.read(OBJECTS_INFO_PATH).unwrap(),
      DEFAULT_OBJECTS_INFO_CONTENT.as_bytes()
    );
    assert_eq!(
      store.read(OBJECTS_PACK_PATH).unwrap(),
      DEFAULT_OBJECTS_PACK_CONTENT.as_bytes()
    );
    assert_eq!(
      store.read(REFS_PATH).unwrap(),
      DEFAULT_REFS_CONTENT.as_bytes()
    );
    assert_eq!(
      store.read(REFS_HEADS_PATH).unwrap(),
      DEFAULT_REFS_HEADS_CONTENT.as_bytes()
    );
    assert_eq!(
      store.read(REFS_TAGS_PATH).unwrap(),
      DEFAULT_REFS_TAGS_CONTENT.as_bytes()
    );
  }
}
