use clap::Parser;
use cli::{Cli, Commands};
use std::{ffi::OsStr, path::PathBuf};
use workspace_provider::local_filesystem_provider::LocalFilesystemProvider;

use crate::{
  data_store::{
    data_store::DataStore, file_store::FileStore, memory_store::MemoryStore,
  },
  hash_object::HashObject,
};

mod cli;
mod data_store;
mod hash_object;
mod hasher;
mod init;
mod workspace_provider;

fn run(args: Cli) {
  let command = args.command;

  let execute_path = args.execute_path;

  let store: Box<dyn DataStore> = if cfg!(debug_assertions) {
    Box::new(MemoryStore::new())
  } else {
    Box::new(FileStore::new(&execute_path.to_str().unwrap()))
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
    Commands::CatFile(args) => {
      println!("CatFile: {:?}", args);
      todo!()
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
    // Commands::Stash(stash) => {
    //     let stash_cmd = stash.command.unwrap_or(StashCommands::Push(stash.push));
    //     match stash_cmd {
    //         StashCommands::Push(push) => {
    //             println!("Pushing {push:?}");
    //         }
    //         StashCommands::Pop { stash } => {
    //             println!("Popping {stash:?}");
    //         }
    //         StashCommands::Apply { stash } => {
    //             println!("Applying {stash:?}");
    //         }
    //     }
    // }
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
