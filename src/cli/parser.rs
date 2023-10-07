use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

// use std::ffi::OsStr;
use std::ffi::OsString;

#[derive(Debug, Parser, PartialEq, Eq)]
#[command(name = "rlt")]
#[command(about = "git written by rust", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,

  #[arg(
    short = 'C',
    required = false,
    value_name = "path",
    default_value = "./"
  )]
  pub execute_path: String,

  #[arg(
    long = "work-tree",
    required = false,
    value_name = "path",
    default_value = "."
  )]
  pub work_tree: String,

  #[arg(
    long = "git-dir",
    required = false,
    value_name = "path",
    default_value = ".git"
  )]
  pub git_dir: String,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
  #[command(about = "initializes a git repository")]
  Init(InitArgs),

  #[command(about = "hashes objects")]
  HashObject(HashObjectArgs),

  #[command(
    about = "Provide content or type and size information for repository objects"
  )]
  CatFile {
    #[arg(value_name = "type")]
    object_type: String,

    #[arg(value_name = "object")]
    object: String,
  },
  /// Clones repos
  #[command(arg_required_else_help = true)]
  Clone {
    /// The remote to clone
    remote: String,
  },
  /// Compare two commits
  Diff {
    #[arg(value_name = "COMMIT")]
    base: Option<OsString>,
    #[arg(value_name = "COMMIT")]
    head: Option<OsString>,
    #[arg(last = true)]
    path: Option<OsString>,
    #[arg(
            long,
            require_equals = true,
            value_name = "WHEN",
            num_args = 0..=1,
            default_value_t = ColorWhen::Auto,
            default_missing_value = "always",
            value_enum
        )]
    color: ColorWhen,
  },
  /// pushes things
  #[command(arg_required_else_help = true)]
  Push {
    /// The remote to target
    remote: String,
  },
  /// adds things
  #[command(arg_required_else_help = true)]
  Add(AddArgs),
  // Stash(StashArgs),
  #[command(external_subcommand)]
  External(Vec<OsString>),
}

/*
* synopsis:

git cat-file <type> <object>
git cat-file (-e | -p) <object>
git cat-file (-t | -s) [--allow-unknown-type] <object>
git cat-file (--batch | --batch-check | --batch-command) [--batch-all-objects]
    [--buffer] [--follow-symlinks] [--unordered]
    [--textconv | --filters] [-Z]
git cat-file (--textconv | --filters)
    [<rev>:<path|tree-ish> | --path=<path|tree-ish> <rev>]
*/

#[derive(Debug, Args, PartialEq, Eq)]
#[command(args_conflicts_with_subcommands = true)]
pub struct InitArgs {
  #[arg(short = 'q', long = "quiet", required = false)]
  pub quiet: bool,

  #[arg(long = "bare", required = false)]
  pub bare: bool,

  #[arg(
    long = "template",
    required = false,
    value_name = "template_directory"
  )]
  pub template: Option<PathBuf>,

  #[arg(long = "separate-git-dir", required = false, value_name = "git_dir")]
  pub separate_git_dir: Option<PathBuf>,

  #[arg(long = "object-format", required = false, value_name = "sha1|sha256")]
  pub object_format: Option<String>,

  #[arg(default_value = ".")]
  pub directory: Option<PathBuf>,
}

#[derive(Debug, Args, PartialEq, Eq)]
#[command(args_conflicts_with_subcommands = true)]
pub struct HashObjectArgs {
  #[arg(short = 't', long = "type", required = false, value_name = "type")]
  pub object_type: Option<String>,

  #[arg(short = 'w', long = "write", required = false)]
  pub write: bool,

  #[arg(short = 's', long = "stdin", required = false)]
  pub from_stdin: bool,

  #[arg(value_name = "path", required = true)]
  pub path: Vec<String>,
}

#[derive(Debug, Args, PartialEq, Eq)]
#[command(args_conflicts_with_subcommands = true)]
pub struct AddArgs {
  #[arg(value_name = "PATH", required = true)]
  pub path_spec: Vec<PathBuf>,

  // #[command(subcommand)]
  // options: AddOptions,
  #[arg(short = 'i', long = "interactive")]
  interactive: bool,

  #[arg(short, long)]
  verbose: bool,

  #[arg(short = 'n', long = "dry-run")]
  dry_run: bool,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorWhen {
  Always,
  Auto,
  Never,
}

impl std::fmt::Display for ColorWhen {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self
      .to_possible_value()
      .expect("no values are skipped")
      .get_name()
      .fmt(f)
  }
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct StashArgs {
  #[command(subcommand)]
  command: Option<StashCommands>,

  #[command(flatten)]
  push: StashPushArgs,
}

#[derive(Debug, Subcommand)]
enum StashCommands {
  Push(StashPushArgs),
  Pop { stash: Option<String> },
  Apply { stash: Option<String> },
}

#[derive(Debug, Args)]
struct StashPushArgs {
  #[arg(short, long)]
  message: Option<String>,
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_init() {
//     let args = Cli::parse_from(&[
//       "rlt",
//       "init",
//       "-q",
//       "--bare",
//       "--template",
//       "foo",
//       "--separate-git-dir",
//       "bar",
//       "--object-format",
//       "sha256",
//       "baz",
//     ]);
//     assert_eq!(
//       args.command,
//       Commands::Init(InitArgs {
//         quiet: true,
//         bare: true,
//         template: Some(PathBuf::from("foo")),
//         separate_git_dir: Some(PathBuf::from("bar")),
//         object_format: Some(String::from("sha256")),
//         directory: Some(PathBuf::from("baz")),
//       })
//     );
//   }

//   #[test]
//   fn test_hash_object() {
//     let args = Cli::parse_from(&[
//       "rlt",
//       "hash-object",
//       "-t",
//       "foo",
//       "-w",
//       "-s",
//       "bar",
//       "baz",
//     ]);
//     assert_eq!(
//       args.command,
//       Commands::HashObject(HashObjectArgs {
//         object_type: Some(String::from("foo")),
//         write: true,
//         from_stdin: true,
//         path: vec![String::from("bar"), String::from("baz")],
//       })
//     );
//   }

//   #[test]
//   fn test_cat_file_type_and_object_synopsis() {
//     let args = Cli::parse_from(&["rlt", "cat-file", "foo", "bar"]);
//     assert_eq!(
//       args.command,
//       Commands::CatFiles {
//         object_type: Some(String::from("foo")),
//         object: Some(String::from("bar")),
//       }
//     )
//   }

//   #[test]
//   fn test_cat_file_options_synopsis() {
//     // let args = Cli::parse_from(&["rlt", "cat-file", "-e", "-p", "foo"]);
//     // assert_eq!(
//     //   args.command,
//     //   Commands::CatFile(CatFileArgs {
//     //     object_type: None,
//     //     secondary_object: None,
//     //     object: Some(String::from("foo")),
//     //     exists: true,
//     //     pretty: true,
//     //     show_size: false,
//     //     show_type: false,
//     //     allow_unknown_type: false,
//     //   })
//     // )
//   }

//   #[test]
//   fn test_add() {
//     let args = Cli::parse_from(&["rlt", "add", "-i", "-v", "-n", "foo", "bar"]);
//     assert_eq!(
//       args.command,
//       Commands::Add(AddArgs {
//         path_spec: vec![PathBuf::from("foo"), PathBuf::from("bar")],
//         interactive: true,
//         verbose: true,
//         dry_run: true,
//       })
//     );
//   }

//   // #[test]
//   // fn test_clone() {
//   //   let args = Cli::parse_from(&["rlt", "clone", "foo"]);
//   //   assert_eq!(
//   //     args.command,
//   //     Commands::Clone {
//   //       remote: String::from("foo"),
//   //     }
//   //   );
//   // }

//   // #[test]
//   // fn test_diff() {
//   //   let args = Cli::parse_from(&["rlt", "diff", "foo", "bar", "baz"]);
//   //   assert_eq!(
//   //     args.command,
//   //     Commands::Diff {
//   //       base: Some(OsString::from("foo")),
//   //       head: Some(OsString::from("bar")),
//   //       path: Some(OsString::from("baz")),
//   //       color: ColorWhen::Auto,
//   //     }
//   //   );
//   // }

//   // #[test]
//   // fn test_push() {
//   //   let args = Cli::parse_from(&["rlt", "push", "foo"]);
//   //   assert_eq!(
//   //     args.command,
//   //     Commands::Push {
//   //       remote: String::from("foo"),
//   //     }
//   //   );
//   // }
// }
