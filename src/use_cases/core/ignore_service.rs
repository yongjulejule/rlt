use log::trace;

use super::pathspec::is_matched;

pub trait IgnoreService {
  fn is_ignored(&self, path: &str) -> bool;
}

pub struct IgnoreServiceImpl {
  ignore_list: Vec<String>,
  negated_ignore_list: Vec<String>,
}

impl IgnoreServiceImpl {
  pub fn new(
    ignore_list: Vec<String>,
    negated_ignore_list: Vec<String>,
  ) -> Self {
    return Self {
      ignore_list,
      negated_ignore_list,
    };
  }

  pub fn from_raw(raw: &[u8]) -> Result<Self, String> {
    let mut ignore_list: Vec<String> = Vec::new();
    let mut negated_ignore_list: Vec<String> = Vec::new();
    let ignore = String::from_utf8_lossy(raw);

    ignore.lines().for_each(|line| {
      if line.starts_with("#") || line.is_empty() {
        return;
      }
      let trimmed_line = line.trim_end();
      if trimmed_line.starts_with("!") {
        negated_ignore_list.push(trimmed_line[1..].to_string());
      } else {
        ignore_list.push(trimmed_line.to_string());
      }
    });
    trace!("ignore_list: {:?}", ignore_list);
    trace!("negated_ignore_list: {:?}", negated_ignore_list);

    Ok(Self::new(ignore_list, negated_ignore_list))
  }
}

impl IgnoreService for IgnoreServiceImpl {
  fn is_ignored(&self, path: &str) -> bool {
    let ignored = self
      .ignore_list
      .iter()
      .any(|pattern| is_matched(pattern, path));
    let negated = self
      .negated_ignore_list
      .iter()
      .any(|pattern| is_matched(pattern, path));
    ignored && !negated
  }
}

#[cfg(test)]
mod tests_construction {
  use super::*;

  #[test]
  fn test_ignore_service() {
    let ignore_list = vec!["a".to_string(), "b".to_string()];
    let negated_ignore_list = vec!["a/c".to_string()];
    let ignore_service =
      IgnoreServiceImpl::new(ignore_list, negated_ignore_list);

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), true);
    assert_eq!(ignore_service.is_ignored("a/c"), false);
    assert_eq!(ignore_service.is_ignored("c"), false);
    assert_eq!(ignore_service.is_ignored("d"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_simple() {
    let raw = b"a\nb";
    let ignore_service = IgnoreServiceImpl::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), true);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_comment() {
    let raw = b"a\n#b";
    let ignore_service = IgnoreServiceImpl::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), false);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_empty_line() {
    let raw = b"a\n\nb\n\n\n";
    let ignore_service = IgnoreServiceImpl::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), true);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_empty() {
    let raw = b"";
    let ignore_service = IgnoreServiceImpl::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), false);
    assert_eq!(ignore_service.is_ignored("b"), false);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_with_space() {
    let raw = b"a\n b";
    let ignore_service = IgnoreServiceImpl::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), false);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_with_trailing_spaces() {
    let raw = b"a\t\t\t\t\n b\n# c";
    let ignore_service = IgnoreServiceImpl::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), false);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_with_trailing_spaces_and_empty_lines() {
    let raw = b"a\t\t\t\t\n\n\n b\n# c\n\n\n";
    let ignore_service = IgnoreServiceImpl::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), false);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }
}
