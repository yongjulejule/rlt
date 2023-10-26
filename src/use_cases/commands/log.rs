use crate::use_cases::core::commit_helper::FormatCommitVisitor;
use log::trace;

use crate::{
  adapters::data_store::DataStore,
  use_cases::core::{
    commit_helper::traverse_commits, object_service::ObjectService,
  },
};

#[derive(Debug)]
pub struct LogOptions {
  is_oneline: bool,
  abbrev_commit: Option<u8>,
  no_abbrev_commit: bool,
  stat: bool,
}

impl LogOptions {
  pub fn new(
    is_oneline: bool,
    abbrev_commit: Option<u8>,
    no_abbrev_commit: bool,
    stat: bool,
  ) -> Self {
    return Self {
      is_oneline,
      abbrev_commit,
      no_abbrev_commit,
      stat,
    };
  }
}

pub struct Log<'a> {
  store: &'a dyn DataStore,
  object_service: &'a dyn ObjectService,
  options: LogOptions,
}

impl<'a> Log<'a> {
  pub fn new(
    store: &'a dyn DataStore,
    object_service: &'a dyn ObjectService,
    options: LogOptions,
  ) -> Self {
    return Self {
      store,
      object_service,
      options,
    };
  }

  pub fn run(&self) -> Result<String, String> {
    trace!("Log: {:?}", self.options);
    let head = self.store.read("HEAD").map_err(|e| e.to_string())?;
    let ref_name = String::from_utf8_lossy(&head)
      .trim_start_matches("ref: ")
      .trim_end()
      .to_string();
    trace!("ref_name: {:?}", ref_name);

    let current_object_hash_raw =
      self.store.read(&ref_name).map_err(|e| e.to_string())?;
    let current_object_hash = String::from_utf8_lossy(&current_object_hash_raw)
      .trim_end()
      .to_string();
    trace!("current_object_hash: {:?}", current_object_hash);
    let head_object_raw = self
      .object_service
      .find(&current_object_hash)
      .map_err(|e| e.to_string())?;
    let head_object = String::from_utf8_lossy(&head_object_raw.data);
    trace!("head_object: {:?}", head_object);

    let abbrev_count = match (
      self.options.abbrev_commit,
      self.options.no_abbrev_commit,
      self.options.is_oneline,
    ) {
      (Some(count), _, _) => count,
      (_, false, true) => 7,
      (_, _, _) => 40,
    };

    let visitor = FormatCommitVisitor::new(
      self.options.is_oneline,
      abbrev_count,
      self.options.stat,
    );
    let result =
      traverse_commits(self.object_service, &current_object_hash, &visitor)?;

    Ok(result.join("\n"))
  }
}
