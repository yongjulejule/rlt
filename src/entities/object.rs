pub enum ObjectType {
  Blob = "blob",
  Tree = "tree",
  Commit = "commit",
  Tag = "tag",
}

pub struct Object {
  pub object_type: ObjectType,
  pub data: Vec<u8>,
  pub key: String,
}
