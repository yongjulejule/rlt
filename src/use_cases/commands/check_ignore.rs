use log::trace;

use crate::use_cases::core::ignore_service::IgnoreService;

pub struct CheckIgnore<'a> {
  ignore_service: &'a dyn IgnoreService,
  paths: Vec<String>,
}

impl<'a> CheckIgnore<'a> {
  pub fn new(
    ignore_service: &'a dyn IgnoreService,
    paths: Vec<String>,
  ) -> Self {
    return Self {
      ignore_service,
      paths,
    };
  }

  pub fn run(&self) -> Vec<String> {
    trace!("CheckIgnore: {:?}", self.paths);
    let ignored: Vec<String> = self
      .paths
      .iter()
      .filter(|&path| self.ignore_service.is_ignored(path))
      .cloned()
      .collect();

    ignored
  }
}

#[cfg(test)]
mod tests {
  use crate::use_cases::core::ignore_service::IgnoreServiceImpl;

  use super::*;

  #[test]
  fn test_check_ignore() {
    let ignore_service =
      IgnoreServiceImpl::new(vec!["a".to_string()], vec!["a/c".to_string()]);
    let paths = ["a".to_string(), "b".to_string(), "a/c".to_string()].to_vec();
    let check_ignore = CheckIgnore::new(&ignore_service, paths);
    let result = check_ignore.run();

    assert_eq!(result, ["a".to_string()]);
  }
}
