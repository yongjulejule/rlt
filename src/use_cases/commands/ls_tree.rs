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
    let mut result = String::new();
    self.list_tree(&tree, &mut result, &self.options.recurse)?;
    Ok(result)
  }

  fn list_tree(
    &self,
    tree: &TreeObject,
    result: &mut String,
    recurse: &bool,
  ) -> Result<(), String> {
    for entry in &tree.entries {
      match entry.mode.as_str() {
        "40000" => {
          result.push_str(&format!(
            "{} {} {}\t{}\n",
            "040000", "tree", entry.hash, entry.name
          ));
          if *recurse {
            let raw_object = self.object_service.find(&entry.hash)?;
            let subtree = TreeObject::parse(&entry.hash, &raw_object.data)?;
            self.list_tree(&subtree, result, recurse)?;
          }
        }
        _ => {
          result.push_str(&format!(
            "{} {} {}\t{}\n",
            entry.mode, "blob", entry.hash, entry.name
          ));
        }
      }
    }
    Ok(())
  }
}
