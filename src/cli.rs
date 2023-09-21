use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

// use std::ffi::OsStr;
use std::ffi::OsString;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "rlt")]
#[command(about = "git written by rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        short = 'C',
        required = false,
        value_name = "path",
        default_value = "."
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
    Stash(StashArgs),
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct InitArgs {
    #[arg(short = 'q', long = "quiet", required = false)]
    quiet: bool,

    #[arg(long = "bare", required = false)]
    bare: bool,

    #[arg(long = "template", required = false, value_name = "template_directory")]
    template: Option<PathBuf>,

    #[arg(long = "separate-git-dir", required = false, value_name = "git_dir")]
    separate_git_dir: Option<PathBuf>,

    #[arg(long = "object-format", required = false, value_name = "sha1|sha256")]
    object_format: Option<String>,

    #[arg(default_value = ".")]
    directory: Option<PathBuf>,
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
        self.to_possible_value()
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