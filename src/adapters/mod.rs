mod object_manager;

pub trait Manager {
  fn new(root_path: String) -> Self;
}
