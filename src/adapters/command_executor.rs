use std::ffi::OsStr;

use crate::{
  adapters::object_manager,
  cli::parser::Commands,
  infrastructures::{
    file_store::FileStore, local_filesystem_provider::LocalFilesystemProvider,
  },
  use_cases::{
    commands::{
      cat_file::CatFile, hash_object::HashObject, init, ls_files::LsFiles,
    },
    core::object_service::ObjectServiceImpl,
  },
};

use super::{
  data_store::DataStore, hasher, workspace_provider::WorkspaceProvider,
};

pub struct CommandExecutionContext {
  pub store: Box<dyn DataStore>,
  pub provider: Box<dyn WorkspaceProvider>,
  pub hasher: Box<dyn hasher::Hasher>,
}

impl CommandExecutionContext {
  pub fn new(
    store: Box<dyn DataStore>,
    provider: Box<dyn WorkspaceProvider>,
    hasher: Box<dyn hasher::Hasher>,
  ) -> Self {
    return Self {
      store,
      provider,
      hasher,
    };
  }

  pub fn setup(execution_path: &str, work_tree: &str, git_dir: &str) -> Self {
    let store: Box<dyn DataStore> =
      Box::new(FileStore::new(&vec![execution_path, git_dir].join("/")));
    let provider: Box<dyn WorkspaceProvider> = Box::new(
      LocalFilesystemProvider::new(&vec![execution_path, work_tree].join("/")),
    ); // [execution_path, work_tree].join("/")
    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());
    return Self::new(store, provider, hasher);
  }
}

pub struct CommandExecutor {
  pub ctx: CommandExecutionContext,
}

impl CommandExecutor {
  pub fn new(ctx: CommandExecutionContext) -> Self {
    return Self { ctx };
  }

  pub fn execute(self, command: Commands) {
    let store = self.ctx.store;
    let provider = self.ctx.provider;
    let hasher = self.ctx.hasher;
    let object_manager = object_manager::ObjectManager::new(store.as_ref());
    let object_service =
      ObjectServiceImpl::new(&object_manager, hasher.as_ref());

    match command {
      Commands::Init(init) => {
        println!("Initializing repository at {:?}", init);
        init::run(store.as_ref())
      }
      Commands::HashObject(cli) => {
        let hash_object = HashObject::new(
          &object_service,
          provider.as_ref(),
          cli.write,
          cli.object_type.unwrap_or("blob".to_string()),
          cli.path,
        );
        println!("Hash Object: {:?}", hash_object.run().unwrap());
      }
      Commands::CatFile {
        object,
        object_type,
      } => {
        let result = CatFile::new(&object_service, object_type, object).run();
        print!("{}", result.ok().unwrap());
      }
      Commands::LsFiles {} => {
        let result = LsFiles::new(store.as_ref()).run();
        println!("{}", result.ok().unwrap().join("\n"));
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
      Commands::External(cli) => {
        println!("Calling out to {:?} with {:?}", &cli[0], &cli[1..]);
      }
    }
  }
}
