use clap::{Error, Parser};
use cli::{Cli, Commands};
use std::ffi::OsStr;

use crate::{data_store::memory_store::MemoryStore, hash_object::HashObject};

mod cli;
mod data_store;
mod hash_object;
mod init;

fn run(command: Commands) {
  // CommandExecutorFactory::new(command).execute();

  match command {
    Commands::HashObject(hash_object) => {
      let store = MemoryStore::new();
      let hash_object = HashObject::new(
        Box::new(store),
        "sha1".to_string(),
        hash_object.write,
        hash_object.object_type.unwrap_or("blob".to_string()),
        hash_object.from_stdin,
        hash_object.path,
      );
      hash_object.run().unwrap();
    }
    Commands::Init(init) => {
      println!("Initializing repository at {:?}", init);
      let store = MemoryStore::new();
      init::run(&store)
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
    _ => {
      println!("No subcommand was used");
    }
  }
}

fn main() {
  let args = Cli::parse();

  println!("=========args: {:?}", args);

  run(args.command);
}
