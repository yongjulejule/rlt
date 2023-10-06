mod adapters;
mod cli;
mod infrastructures;
mod use_cases;

use crate::adapters::{hasher, object_manager};
use crate::use_cases::cat_file::CatFile;
use crate::use_cases::hash_object::HashObject;
use crate::use_cases::init;

use std::ffi::OsStr;
use std::path::PathBuf;

use adapters::data_store::DataStore;
use clap::Parser;

use cli::{Cli, Commands};
use infrastructures::{
  file_store::FileStore, local_filesystem_provider::LocalFilesystemProvider,
  memory_store::MemoryStore,
};

fn run(args: Cli) {
  let command = args.command;

  let execute_path = args.execute_path;
  let git_dir = args.git_dir;

  let store: Box<dyn DataStore> = if cfg!(debug_assertions) {
    Box::new(MemoryStore::new())
  } else {
    Box::new(FileStore::new(&git_dir.to_str().unwrap()))
  };

  match command {
    Commands::Init(init) => {
      println!("Initializing repository at {:?}", init);
      init::run(store.as_ref())
    }
    Commands::HashObject(args) => {
      // let store = MemoryStore::new();
      let provider = LocalFilesystemProvider::new(PathBuf::from("."));
      let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());
      let hash_object = HashObject::new(
        store.as_ref(),
        &provider,
        hasher.as_ref(),
        args.write,
        args.object_type.unwrap_or("blob".to_string()),
        args.path,
      );
      println!("Hash Object: {:?}", hash_object.run().unwrap());
    }
    Commands::CatFiles {
      object,
      object_type,
    } => {
      println!("CatFile: {:?}, {:?}", object, object_type);
      let object_manager = object_manager::ObjectManager::new(store);
      let result = CatFile::new(&object_manager, object_type, object).run();
      println!("CatFile result: {:?}", result)
    }
    Commands::Add(add) => {
      println!("Adding {:?} ", Some(add.path_spec));
    }
    Commands::Clone { remote } => {
      println!("Cloning {remote}");
    }
    Commands::Diff {
      mut base,
      mut head,
      mut path,
      color,
    } => {
      if path.is_none() {
        path = head;
        head = None;
        if path.is_none() {
          path = base;
          base = None;
        }
      }
      let base = base
        .as_deref()
        .map(|s| s.to_str().unwrap())
        .unwrap_or("stage");
      let head = head
        .as_deref()
        .map(|s| s.to_str().unwrap())
        .unwrap_or("worktree");
      let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
      println!(
        "Diffing {}..{} {} (color={})",
        base,
        head,
        path.to_string_lossy(),
        color
      );
    }
    Commands::Push { remote } => {
      println!("Pushing to {remote}");
    }
    Commands::External(args) => {
      println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
    }
  }
}

fn main() {
  let args = Cli::parse();

  println!("=========args: {:?}", args);

  run(args);
}
