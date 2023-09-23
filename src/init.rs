use crate::data_store::data_store::DataStore;

const GIT_FILES: [(&str, &str); 10] = [
  ("config", "[core]\nrepositoryformatversion = 0\nfilemode = true\nbare = false\nlogallrefupdates = true\nignorecase = true\nprecomposeunicode = true"),
  ("HEAD", "ref: refs/heads/main"),
  ("description", "Unnamed repository; edit this file 'description' to name the repository."),
  ("info/exclude", ""),
  ("objects/", ""),
  ("objects/info", ""),
  ("objects/pack", ""),
  ("refs/", ""),
  ("refs/heads", ""),
  ("refs/tags", ""),
];

pub fn run(store: &dyn DataStore /* , path: String */) {
  GIT_FILES.iter().for_each(|(path, content)| {
    store.write(path, content.as_bytes()).expect("write");
  });
}

#[cfg(test)]
mod run_tests {
  use super::*;
  #[allow(unused_imports)]
  use crate::data_store::{file_store::FileStore, memory_store::MemoryStore};

  #[test]
  fn test_init() {
    let store = MemoryStore::new();
    // let store: Box<dyn DataStore> = Box::new(FileStore::new("/tmp/tmp/.git"));
    run(&store);

    GIT_FILES.iter().for_each(|(path, content)| {
      assert_eq!(store.read(path).unwrap(), content.as_bytes());
    });
  }
}
