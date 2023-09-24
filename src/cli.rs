use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

// use std::ffi::OsStr;
use std::ffi::OsString;

#[derive(Debug, Parser)]
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
  pub execute_path: PathBuf,

  #[arg(
    long = "work-tree",
    required = false,
    value_name = "path",
    default_value = "."
  )]
  pub work_tree: PathBuf,

  #[arg(
    long = "git-dir",
    required = false,
    value_name = "path",
    default_value = "."
  )]
  pub git_dir: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
  #[command(about = "initializes a git repository")]
  Init(InitArgs),

  #[command(about = "hashes objects")]
  HashObject(HashObjectArgs),

  #[command(
    about = "Provide content or type and size information for repository objects"
  )]
  CatFile(CatFileArgs),

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

#[derive(Debug, Args)]
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

#[derive(Debug, Args)]
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

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CatFileArgs {
  #[arg(short = 't', required = false)]
  print_object_type: bool,

  #[arg(short = 's', long = "size", required = false)]
  size: bool,

  #[arg(short = 'p', long = "pretty", required = false)]
  pretty: bool,

  // if any option present, type is not required
  #[arg(value_name = "type", required_unless_all = ["size", "pretty", "print_object_type"])]
  object_type: Option<String>,

  #[arg(value_name = "object", required = false)]
  object: Option<String>,
}

impl CatFileArgs {
  pub fn any_option_present(&self) -> bool {
    self.print_object_type || self.size || self.pretty
  }
}

#[derive(Debug, Args)]
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
