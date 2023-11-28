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

    let result = self.list_tree("", &tree, self.options.recurse)?.concat();
    Ok(result)
  }

  // FIXME: 개비효율적임 나중에 수정하기~
  fn list_tree(
    &self,
    parent_directory: &str,
    tree: &TreeObject,
    recurse: bool,
  ) -> Result<Vec<String>, String> {
    tree
      .entries
      .iter()
      .map(|entry| {
        let full_name = if parent_directory.is_empty() {
          entry.name.clone()
        } else {
          format!("{}/{}", parent_directory, entry.name)
        };

        let is_entry_tree = is_tree(entry.mode.as_str());
        match (is_entry_tree, recurse) {
          (true, true) => {
            let raw_object = self.object_service.find(&entry.hash)?;
            let subtree = TreeObject::parse(&entry.hash, &raw_object.data)?;
            self.list_tree(&full_name, &subtree, recurse)
          }
          _ => {
            let line = if is_tree(entry.mode.as_str()) {
              format!("{} {} {}\t{}\n", "040000", "tree", entry.hash, full_name)
            } else {
              format!(
                "{} {} {}\t{}\n",
                entry.mode, "blob", entry.hash, full_name
              )
            };
            Ok(vec![line])
          }
        }
      })
      .collect::<Result<Vec<_>, String>>()
      .map(|lines| lines.concat())
  }
}

fn is_tree(mode: &str) -> bool {
  mode == "40000"
}
