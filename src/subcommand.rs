trait SubCommand {
  fn run(&self) -> bool;
  fn usage(&self) -> String;
}
