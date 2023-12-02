use std::collections::BTreeSet;

use log::trace;

use crate::{
  entities::object::{TreeElement, TreeObject},
  use_cases::core::object_service::ObjectService,
};

#[derive(Debug)]
pub struct LsTreeOptions {
  pub recurse: bool,
  pub tree_ish: String,
  pub path: Vec<String>,
}

pub struct LsTree<'a> {
  object_service: &'a dyn ObjectService,
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
    options: LsTreeOptions,
  ) -> Self {
    return Self {
      object_service,
      options,
    };
  }

  pub fn run(&self) -> Result<String, String> {
    trace!("LsTree: {:?}", self.options);

    let raw_object = self.object_service.find(&self.options.tree_ish)?;
    let tree = TreeObject::parse(&self.options.tree_ish, &raw_object.data)?;
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
        let (object_type, object_mode) = determine_type(entry.mode.as_str());

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

fn determine_type(mode: &str) -> (&str, &str) {
  match mode {
    "40000" => (ITEM_TYPE_TREE, "040000"),
    "100644" | "100755" => (ITEM_TYPE_BLOB, mode),
    "120000" => (ITEM_TYPE_COMMIT, mode),
    _ => panic!("Unknown mode: {}", mode),
  }
}
