pub mod compressor;
pub mod data_store;
pub mod hasher;
pub mod object_manager;
pub mod object_store;
pub mod workspace_provider;

pub trait Manager {
  fn new(root_path: String) -> Self;
}
