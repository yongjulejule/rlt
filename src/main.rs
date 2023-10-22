use clap::Parser;
use log::{debug, error, info, trace, warn};

use crate::{
  adapters::command_executor::{CommandExecutionContext, CommandExecutor},
  cli::parser::Cli,
};
mod adapters;
mod cli;
mod entities;
mod infrastructures;
mod use_cases;

fn main() -> Result<(), String> {
  env_logger::init();
  // trace!("trace");
  // warn!("warn");
  // info!("info");
  // debug!("debug");
  // error!("error");

  let args = Cli::parse();
  trace!("args: {:?}", args);

  let ctx = CommandExecutionContext::setup(
    &args.execute_path,
    &args.work_tree,
    &args.git_dir,
  );
  let result = CommandExecutor::new(ctx).execute(args.command);
  if result.is_err() {
    error!("error: {:?}", result.as_ref().err());
  }

  result
}
