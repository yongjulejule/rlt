use super::pathspec::is_matched;

struct IgnoreService {
  ignore_list: Vec<String>,
  negated_ignore_list: Vec<String>,
}

impl IgnoreService {
  fn new(ignore_list: Vec<String>, negated_ignore_list: Vec<String>) -> Self {
    return Self {
      ignore_list,
      negated_ignore_list,
    };
  }

  fn from_raw(raw: &str) -> Result<Self, String> {
    let mut ignore_list: Vec<String> = Vec::new();
    let mut negated_ignore_list: Vec<String> = Vec::new();

    for line in raw.lines() {
      if line.starts_with("#") || line.is_empty() {
        continue;
      }

      let trimmed_line = line.trim_end();

      if trimmed_line.starts_with("!") {
        negated_ignore_list.push(trimmed_line[1..].to_string());
      } else {
        ignore_list.push(trimmed_line.to_string());
      }
    }

    Ok(Self::new(ignore_list, negated_ignore_list))
  }

  fn is_ignored(&self, path: &str) -> bool {
    self
      .ignore_list
      .iter()
      .any(|pattern| is_matched(path, pattern))
      && !self
        .negated_ignore_list
        .iter()
        .any(|pattern| is_matched(path, pattern))
  }
}

#[cfg(test)]
mod tests_construction {
  use super::*;

  #[test]
  fn test_ignore_service() {
    let ignore_list = vec!["a".to_string(), "b".to_string()];
    let negated_ignore_list = vec!["c".to_string()];
    let ignore_service = IgnoreService::new(ignore_list, negated_ignore_list);

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), true);
    assert_eq!(ignore_service.is_ignored("c"), false);
    assert_eq!(ignore_service.is_ignored("d"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_simple() {
    let raw = "a\nb";
    let ignore_service = IgnoreService::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), true);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_comment() {
    let raw = "a\n#b";
    let ignore_service = IgnoreService::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), false);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_empty_line() {
    let raw = "a\n\nb\n\n\n";
    let ignore_service = IgnoreService::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), true);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_empty() {
    let raw = "";
    let ignore_service = IgnoreService::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), false);
    assert_eq!(ignore_service.is_ignored("b"), false);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_with_space() {
    let raw = "a\n b";
    let ignore_service = IgnoreService::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), false);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_with_trailing_spaces() {
    let raw = "a\t\t\t\t\n b\n# c";
    let ignore_service = IgnoreService::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), false);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }

  #[test]
  fn test_ignore_service_from_raw_with_trailing_spaces_and_empty_lines() {
    let raw = "a\t\t\t\t\n\n\n b\n# c\n\n\n";
    let ignore_service = IgnoreService::from_raw(raw).unwrap();

    assert_eq!(ignore_service.is_ignored("a"), true);
    assert_eq!(ignore_service.is_ignored("b"), false);
    assert_eq!(ignore_service.is_ignored("c"), false);
  }
}
