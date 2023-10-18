mod pathspec {
  // check given path matched with pathspec
  pub fn check_path(path: &str, pathspec: &str) -> bool {
    let mut path = path.to_string();
    let mut pathspec = pathspec.to_string();
    // 1. *, **, ?, [, ]
    // 2. :(magic)
    // ????

    return true;
  }
}
