use std::ffi::OsStr;

use log::trace;

use crate::{
  cli::parser::Commands,
  infrastructures::{
    file_store::FileStore, local_filesystem_provider::LocalFilesystemProvider,
  },
  use_cases::{
    commands::{
      cat_file::CatFile, check_ignore::CheckIgnore, hash_object::HashObject,
      init, ls_files::LsFiles,
    },
    core::{
      commit_helper::{traverse_commits, PrintMessageVisitor},
      ignore_service::IgnoreServiceImpl,
      object_service::ObjectService,
      object_service::ObjectServiceImpl,
    },
  },
};

use super::{
  data_store::DataStore, hasher, object_manager::ObjectManagerImpl,
  workspace_provider::WorkspaceProvider,
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
    trace!(
      "setup:\n\texecution_path: [{}]\n\twork_tree: [{}]\n\tgit_dir: [{}]",
      execution_path,
      work_tree,
      git_dir
    );

    let store: Box<dyn DataStore> =
      Box::new(FileStore::new(&vec![execution_path, git_dir].join("/")));
    let provider: Box<dyn WorkspaceProvider> = Box::new(
      LocalFilesystemProvider::new(&vec![execution_path, work_tree].join("/")),
    ); // [execution_path, work_tree].join("/")
    let hasher = hasher::HasherFactory::new().get_hasher("sha1".to_string());
    Self::new(store, provider, hasher)
  }
}

pub struct CommandExecutor {
  pub ctx: CommandExecutionContext,
}

impl CommandExecutor {
  pub fn new(ctx: CommandExecutionContext) -> Self {
    return Self { ctx };
  }

  pub fn execute(self, command: Commands) -> Result<(), String> {
    let store = self.ctx.store;
    let provider = self.ctx.provider;
    let hasher = self.ctx.hasher;
    let object_manager = ObjectManagerImpl::new(store.as_ref());
    let object_service =
      ObjectServiceImpl::new(&object_manager, hasher.as_ref());
    let ignore_raw = provider.get_contents(".gitignore".to_string());
    let ignore_service = IgnoreServiceImpl::from_raw(&ignore_raw.as_bytes())?;

    trace!("command: {:?}", command);

    match command {
      Commands::Init(init) => {
        trace!("Initializing repository at {:?}", init);
        init::run(store.as_ref());
        Ok(())
      }

      Commands::HashObject(cli) => {
        let hash_object = HashObject::new(
          &object_service,
          provider.as_ref(),
          cli.write,
          cli.object_type.unwrap_or("blob".to_string()),
          cli.path,
        );
        let result = hash_object.run()?;
        trace!("Hash Object: {:?}", result);
        Ok(())
      }

      Commands::CatFile {
        object,
        object_type,
      } => {
        let result =
          CatFile::new(&object_service, object_type, object).run()?;
        println!("{}", result);
        Ok(())
      }

      Commands::LsFiles {} => {
        let result = LsFiles::new(store.as_ref()).run();
        trace!("{}", result.ok().unwrap().join("\n"));
        Ok(())
      }

      Commands::CheckIgnore { paths } => {
        trace!("Checking ignore for {:?}", paths);

        let result = CheckIgnore::new(&ignore_service, paths).run()?;
        println!("{}", result.join("\n"));
        Ok(())
      }
      Commands::Log {} => {
        trace!("Log");
        // TODO: Following two lies are related to rev-parse. Move after implementing rev-parse
        let head = store.read("HEAD").map_err(|e| e.to_string())?;
        let ref_name = String::from_utf8_lossy(&head)
          .trim_start_matches("ref: ")
          .trim_end()
          .to_string();
        trace!("ref_name: {:?}", ref_name);

        let current_object_hash_raw =
          store.read(&ref_name).map_err(|e| e.to_string())?;
        let current_object_hash =
          String::from_utf8_lossy(&current_object_hash_raw)
            .trim_end()
            .to_string();
        trace!("current_object_hash: {:?}", current_object_hash);
        let head_object_raw = object_service
          .find(&current_object_hash)
          .map_err(|e| e.to_string())?;
        let head_object = String::from_utf8_lossy(&head_object_raw.data);
        trace!("head_object: {:?}", head_object);

        let visitor = PrintMessageVisitor;
        traverse_commits(&object_service, &current_object_hash, &visitor)?;

        Ok(())
      }

      Commands::Add(add) => {
        trace!("Adding {:?} ", Some(add.path_spec));
        Ok(())
      }
      Commands::Clone { remote } => {
        trace!("Cloning {remote}");
        Ok(())
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
        trace!(
          "Diffing {}..{} {} (color={})",
          base,
          head,
          path.to_string_lossy(),
          color
        );
        Ok(())
      }
      Commands::Push { remote } => {
        trace!("Pushing to {remote}");
        Ok(())
      }
      Commands::External(cli) => {
        trace!("Calling out to {:?} with {:?}", &cli[0], &cli[1..]);
        Ok(())
      }
    }
  }
}
