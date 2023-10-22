use crate::entities::object::CommitObject;

use super::object_service::ObjectService;

pub trait CommitVisitor {
  fn visit_commit(&self, commit: &CommitObject);
}
pub struct PrintMessageVisitor;

impl CommitVisitor for PrintMessageVisitor {
  fn visit_commit(&self, commit: &CommitObject) {
    println!("{} {}", &commit.hash[..7], commit.message);
  }
}

pub fn traverse_commits<S: ObjectService>(
  service: &S,
  start_hash: &str,
  visitor: &dyn CommitVisitor,
) -> Result<(), String> {
  let mut stack: Vec<String> = Vec::new();
  stack.push(start_hash.to_string());

  while let Some(current_hash) = stack.pop() {
    match service.find(&current_hash) {
      Ok(raw_object) => {
        let commit = CommitObject::parse(
          &current_hash,
          &String::from_utf8_lossy(&raw_object.data),
        )?;
        visitor.visit_commit(&commit);

        for parent_hash in commit.parents.iter() {
          stack.push(parent_hash.clone());
        }
      }
      Err(err) => return Err(err),
    }
  }

  Ok(())
}
