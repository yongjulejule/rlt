use std::ffi::OsStr;

use log::trace;

use crate::{
  cli::parser::Commands,
  infrastructures::{
    file_store::FileStore, local_filesystem_provider::LocalFilesystemProvider,
  },
  use_cases::{
    commands::{
      cat_file::CatFile,
      check_ignore::CheckIgnore,
      hash_object::HashObject,
      init,
      log::{Log, LogOptions},
      ls_files::LsFiles,
      ls_tree::{LsTree, LsTreeOptions},
      status::Status,
    },
    core::{
      ignore_service::IgnoreServiceImpl, object_service::ObjectServiceImpl,
      revision_service::RevisionServiceImpl,
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
    let ignore_raw = provider.get_contents(".gitignore".to_string()).unwrap();
    let ignore_service = IgnoreServiceImpl::from_raw(&ignore_raw)?;
    let revision_service = RevisionServiceImpl::new(store.as_ref());

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
        println!("{}", result.ok().unwrap().join("\n"));
        Ok(())
      }

      Commands::CheckIgnore { paths } => {
        trace!("Checking ignore for {:?}", paths);

        let result = CheckIgnore::new(&ignore_service, paths).run()?;
        println!("{}", result.join("\n"));
        Ok(())
      }
      Commands::Log(log_args) => {
        trace!("Log");

        let options = LogOptions::new(
          log_args.is_oneline,
          log_args.abbrev_commit,
          log_args.no_abbrev_commit,
          log_args.revision_range,
          log_args.stat,
        );

        let result =
          Log::new(store.as_ref(), &object_service, &revision_service, options)
            .run()?;

        println!("{}", result);
        Ok(())
      }
      Commands::LsTree(ls_tree_args) => {
        trace!("LsTree");

        let ls_tree_options = LsTreeOptions {
          recurse: ls_tree_args.recurse,
          tree_ish: ls_tree_args.tree_ish,
          path: ls_tree_args.path,
        };

        let result =
          LsTree::new(&object_service, &revision_service, ls_tree_options)
            .run()?;
        result.iter().for_each(|r| {
          println!("{} {} {}\t{}\n", r.object_type, r.mode, r.hash, r.name)
        });
        Ok(())
      }

      Commands::Status {} => {
        trace!("Status");
        let result = Status::new(
          store.as_ref(),
          provider.as_ref(),
          &ignore_service,
          &object_service,
          &revision_service,
        )
        .run()?;

        println!("Result for status 👋 \n");
        let printer = &|(status, path): &(String, String)| {
          let red = "\x1b[31m";
          let green = "\x1b[32m";
          let reset = "\x1b[0m";
          match status.as_str() {
            "deleted" => println!("💩 {}\t{}: {}{}", red, status, path, reset),
            "modified" => {
              println!("🪄 {}\t{}: {}{}", green, status, path, reset)
            }
            "new file" => {
              println!("✨ {}\t{}: {}{}", green, status, path, reset)
            }
            _ => {}
          }
        };
        println!("Changes to be committed 💌 :");
        result.staged.iter().for_each(printer);
        println!("Changes not staged for commit 💤 :");
        result.unstaged.iter().for_each(printer);
        println!("Untracked files 👽 :");
        result.untracked.iter().for_each(|path| {
          println!("\t{}:\t{}\n", "new file", path);
        });
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
