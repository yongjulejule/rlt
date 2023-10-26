use log::trace;

use crate::{
  entities::object::TreeObject, use_cases::core::object_service::ObjectService,
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
    let result = tree
      .entries
      .iter()
      .map(|entry| match entry.mode.as_str() {
        "40000" => {
          format!("{} {} {}\t{}\n", "040000", "tree", entry.hash, entry.name)
        }
        _ => {
          format!("{} {} {}\t{}\n", entry.mode, "blob", entry.hash, entry.name)
        }
      })
      .collect::<Vec<String>>()
      .join("");

    // let mut result = String::new();
    // for (name, hash) in tree.entries.iter() {
    //   let object = self.object_service.find(hash)?;
    //   result.push_str(&format!("{}\t{}\t{}\n", object.object_type, hash, name));
    // }
    Ok(result)
  }
}
