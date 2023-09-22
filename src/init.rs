use crate::data_store::data_store::DataStore;

pub fn run<T: DataStore>(store: &T /* , path: String */) {
  {
    let data = "[core]
    repositoryformatversion = 0
    filemode = true
    bare = false
    logallrefupdates = true
    ignorecase = true
    precomposeunicode = true"
      .as_bytes();
    store.write("config", data).expect("write config");
  }
  {
    let data = "ref: refs/heads/master".as_bytes();
    store.write("HEAD", data).expect("write HEAD");
  }
  {
    let data =
      "Unnamed repository; edit this file 'description' to name the repository.".as_bytes();
    store.write("description", data).expect("write description");
  }
  {
    let data = "".as_bytes();
    store
      .write("info/exclude", data)
      .expect("write info/exclude");
  }
  {
    let data = "".as_bytes();
    store.write("objects/", data).expect("write objects");
  }
  {
    let data = "".as_bytes();
    store
      .write("objects/info", data)
      .expect("write objects/info");
  }
  {
    let data = "".as_bytes();
    store
      .write("objects/pack", data)
      .expect("write objects/pack");
  }
  {
    let data = "".as_bytes();
    store.write("refs/", data).expect("write refs");
  }
  {
    let data = "".as_bytes();
    store.write("refs/heads", data).expect("write refs/heads");
  }
  {
    let data = "".as_bytes();
    store.write("refs/tags", data).expect("write refs/tags");
  }
}

#[cfg(test)]
mod run_tests {
  use super::*;
  use crate::data_store::memory_store::MemoryStore;

  #[test]
  fn test_init() {
    let store = MemoryStore::new();
    run(&store);
    println!(
      "{}",
      String::from_utf8(store.read("config").expect("read config")).expect("config is utf8")
    );
    assert_eq!(
      store.read("config").expect("read config"),
      "[core]
    repositoryformatversion = 0
    filemode = true
    bare = false
    logallrefupdates = true
    ignorecase = true
    precomposeunicode = true"
        .as_bytes()
    );
    assert_eq!(
      store.read("HEAD").expect("read HEAD"),
      "ref: refs/heads/master".as_bytes()
    );
    assert_eq!(
      store.read("description").expect("read description"),
      "Unnamed repository; edit this file 'description' to name the repository.".as_bytes()
    );
    assert_eq!(
      store.read("info/exclude").expect("read info/exclude"),
      "".as_bytes()
    );
    assert_eq!(
      store.read("objects/").expect("read objects/"),
      "".as_bytes()
    );
    assert_eq!(
      store.read("objects/info").expect("read objects/info"),
      "".as_bytes()
    );
    assert_eq!(
      store.read("objects/pack").expect("read objects/pack"),
      "".as_bytes()
    );
    assert_eq!(store.read("refs/").expect("read refs/"), "".as_bytes());
    assert_eq!(
      store.read("refs/heads").expect("read refs/heads"),
      "".as_bytes()
    );
    assert_eq!(
      store.read("refs/tags").expect("read refs/tags"),
      "".as_bytes()
    );
  }
}
