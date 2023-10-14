pub mod command_executor;
pub mod compressor;
pub mod data_store;
pub mod hasher;
pub mod object_manager;
pub mod workspace_provider;
pub mod filesystem_utils;

pub trait Manager {
  fn new(root_path: String) -> Self;
}
