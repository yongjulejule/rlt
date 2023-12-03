use std::collections::BTreeSet;

use log::trace;

use crate::{
  entities::object::{CommitObject, TreeElement, TreeObject},
  use_cases::core::{
    object_service::ObjectService, revision_service::RevisionService,
  },
};

#[derive(Debug)]
pub struct LsTreeOptions {
  pub recurse: bool,
  pub tree_ish: String,
  pub path: Vec<String>,
}

pub struct LsTree<'a> {
  object_service: &'a dyn ObjectService,
  revision_service: &'a dyn RevisionService,
  options: LsTreeOptions,
}

const MAX_DEPTH: usize = 42;
const MODE_TREE: &str = "040000";
const ITEM_TYPE_TREE: &str = "tree";
const ITEM_TYPE_BLOB: &str = "blob";
const ITEM_TYPE_COMMIT: &str = "commit";

impl<'a> LsTree<'a> {
  pub fn new(
    object_service: &'a dyn ObjectService,
    revision_service: &'a dyn RevisionService,
    options: LsTreeOptions,
  ) -> Self {
    return Self {
      object_service,
      revision_service,
      options,
    };
  }

  pub fn run(&self) -> Result<String, String> {
    trace!("LsTree: {:?}", self.options);

    let raw_hash =
      match self.object_service.is_object_hash(&self.options.tree_ish) {
        true => self.options.tree_ish.clone(),
        false => {
          let commit_hash =
            self.revision_service.resolve(&self.options.tree_ish)?;
          let commit_object_raw = self.object_service.find(&commit_hash)?;
          CommitObject::parse(&commit_hash, &commit_object_raw.data)?.tree
        }
      };

    trace!("raw_hash: {}", raw_hash);
    let raw_object = self.object_service.find(&raw_hash)?;
    trace!("raw_object: {:?}", raw_object);
    trace!(
      "raw_object.data: {:?}",
      String::from_utf8(raw_object.data.clone())
    );
    let tree = TreeObject::parse(&raw_hash, &raw_object.data)?;
    trace!("tree: {:?}", tree);

    let paths = BTreeSet::from_iter(self.options.path.clone());

    let result = self
      .list_tree(None, &tree.entries, &paths, self.options.recurse, 0)?
      .concat();
    trace!("result: {:?}", result);
    Ok(result)
  }

  fn list_tree(
    &self,
    parent_directory: Option<&str>,
    entries: &[TreeElement],
    paths: &BTreeSet<String>,
    recurse: bool,
    depth: usize,
  ) -> Result<Vec<String>, String> {
    if depth > MAX_DEPTH {
      return Err(format!("Max depth exceeded in tree: {}", MAX_DEPTH));
    }
    entries
      .iter()
      .filter(|entry| {
        // NOTE: path arguments not supported in recursive mode
        recurse || paths.is_empty() || paths.contains(&entry.name)
      })
      .map(|entry| {
        let full_name = parent_directory
          .map(|dir| format!("{}/{}", dir, entry.name))
          .unwrap_or_else(|| entry.name.clone());
        let (object_type, object_mode) = determine_type(entry.mode.as_str())?;

        match (object_type, recurse) {
          (ITEM_TYPE_TREE, true) => {
            let raw_object = self.object_service.find(&entry.hash)?;
            let subtree = TreeObject::parse(&entry.hash, &raw_object.data)?;
            self.list_tree(
              Some(&full_name),
              &subtree.entries,
              paths,
              recurse,
              depth + 1,
            )
          }
          _ => Ok(vec![format!(
            "{} {} {}\t{}\n",
            object_mode, object_type, entry.hash, full_name
          )]),
        }
      })
      .collect::<Result<Vec<_>, _>>()
      .map(|lines| lines.concat())
  }
}

fn determine_type(mode: &str) -> Result<(&str, &str), String> {
  match mode {
    "40000" => Ok((ITEM_TYPE_TREE, MODE_TREE)),
    "100644" | "100755" => Ok((ITEM_TYPE_BLOB, mode)),
    "120000" => Ok((ITEM_TYPE_COMMIT, mode)),
    _ => Err(format!("Unknown mode: {}", mode)),
  }
}
