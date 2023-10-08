use clap::Parser;

use crate::{
  adapters::command_executor::{CommandExecutionContext, CommandExecutor},
  cli::parser::Cli,
};
mod adapters;
mod cli;
mod infrastructures;
mod use_cases;
mod entities;

fn main() {
  let args = Cli::parse();
  println!("args: {:?}\n", args);

  let ctx = CommandExecutionContext::setup(
    &args.execute_path,
    &args.work_tree,
    &args.git_dir,
  );
  CommandExecutor::new(ctx).execute(args.command);
}
