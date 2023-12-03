use log::trace;

use crate::adapters::data_store::DataStore;

pub trait RevisionService {
  fn resolve(&self, revision: &str) -> Result<String, String>;

  /// Finds a reference by its name.
  ///
  /// # Arguments
  ///
  /// * `ref_name` - The name of the reference to find.
  ///
  /// # Returns
  ///
  /// * `Ok(String)` - The found reference. (don't read the content)
  /// * `Err(String)` - An error message if the reference could not be found.
  fn find_ref(&self, ref_name: &str) -> Result<String, String>;

  fn read_ref(&self, revision: &str) -> Result<String, String>;
}

pub struct RevisionServiceImpl<'a> {
  store: &'a dyn DataStore,
}

impl<'a> RevisionServiceImpl<'a> {
  pub fn new(store: &'a dyn DataStore) -> Self {
    return Self { store };
  }
}

impl<'a> RevisionService for RevisionServiceImpl<'a> {
  fn resolve(&self, revision: &str) -> Result<String, String> {
    trace!("resolve: {}", revision);
    let ref_name = self.find_ref(revision)?;
    trace!("ref_name: {}", ref_name);
    let content = self.read_ref(&ref_name)?;
    trace!("content: {}", content);
    Ok(content)
  }

  fn read_ref(&self, ref_name: &str) -> Result<String, String> {
    let content_raw = self.store.read(ref_name).map_err(|e| e.to_string())?;
    let content = String::from_utf8_lossy(&content_raw).trim_end().to_string();
    if is_symbolic_ref(&content_raw) {
      let ref_name = content.trim_start_matches("ref: ").trim_end().to_string();
      return self.read_ref(&ref_name);
    }
    Ok(content)
  }

  fn find_ref(&self, ref_name: &str) -> Result<String, String> {
    let ref_rules = [
      |name: &str| name.to_string(),
      |name: &str| format!("refs/{}", name),
      |name: &str| format!("refs/tags/{}", name),
      |name: &str| format!("refs/heads/{}", name),
      |name: &str| format!("refs/remotes/{}", name),
      |name: &str| format!("refs/remotes/{}/HEAD", name),
    ];

    ref_rules
      .iter()
      .map(|rule| rule(ref_name))
      .find(|candidate| self.store.exists(candidate).unwrap_or(false))
      .map_or_else(|| Err(format!("Not found ref: {}", ref_name)), Ok)
  }
}

fn is_symbolic_ref(ref_content: &[u8]) -> bool {
  ref_content.starts_with(b"ref: ")
}

#[cfg(test)]
mod test {
  use crate::{
    adapters::data_store::DataStore,
    infrastructures::memory_store::MemoryStore,
    use_cases::core::revision_service::{RevisionService, RevisionServiceImpl},
  };

  #[test]
  fn resolve_head() {
    // given
    // set HEAD to 1234567890123456789012345678901234567890
    let memory_store = MemoryStore::new();
    memory_store
      .write("HEAD", "ref: refs/heads/main".as_bytes())
      .unwrap();
    memory_store
      .write(
        "refs/heads/main",
        "1234567890123456789012345678901234567890".as_bytes(),
      )
      .unwrap();
    let revision_service = RevisionServiceImpl::new(&memory_store);
    let revision = "HEAD".to_string();

    // when HEAD is resolved
    let result = revision_service.resolve(&revision).unwrap();

    // then
    assert_eq!(result, "1234567890123456789012345678901234567890")
  }

  #[test]
  fn find_ref_location() {
    // given
    // set HEAD to 1234567890123456789012345678901234567890
    let memory_store = MemoryStore::new();
    memory_store
      .write("refs/remotes/main/HEAD", "ref: refs/heads/main".as_bytes())
      .unwrap();
    let revision_service = RevisionServiceImpl::new(&memory_store);
    let ref_name = "main".to_string();

    // when HEAD is resolved
    let result = revision_service.find_ref(&ref_name);
    println!("result: {:?}", result);

    // then
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), "refs/remotes/main/HEAD")
  }

  #[test]
  fn read_ref() {
    // given
    // set HEAD to 1234567890123456789012345678901234567890
    let memory_store = MemoryStore::new();
    memory_store
      .write("refs/remotes/main/HEAD", "ref: refs/heads/main".as_bytes())
      .unwrap();
    memory_store
      .write(
        "refs/heads/main",
        "1234567890123456789012345678901234567890".as_bytes(),
      )
      .unwrap();

    let revision_service = RevisionServiceImpl::new(&memory_store);
    let ref_name = "refs/remotes/main/HEAD".to_string();

    // when HEAD is resolved
    let result = revision_service.read_ref(&ref_name);
    println!("result: {:?}", result);

    // then
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), "1234567890123456789012345678901234567890")
  }

  #[test]
  fn read_ref_not_exist() {
    // given
    // set HEAD to 1234567890123456789012345678901234567890
    let memory_store = MemoryStore::new();

    let revision_service = RevisionServiceImpl::new(&memory_store);
    let ref_name = "refs/remotes/main/HEAD".to_string();

    // when HEAD is resolved
    let result = revision_service.read_ref(&ref_name);

    // then
    assert_eq!(result.is_err(), true);
  }
}
