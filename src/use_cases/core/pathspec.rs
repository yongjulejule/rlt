// normalize path
// check is in git dir

use globset::Glob;
use std::path::{Path, PathBuf};

fn simple_match(pathspec: &str, path: &str) -> bool {
  Glob::new(pathspec)
    .expect("Failed to create glob pattern")
    .compile_matcher()
    .is_match(path)
}

fn match_recursive(pathspec: &str, path: &str) -> bool {
  let parts: Vec<&str> = pathspec.splitn(2, "**/").collect();

  match parts.as_slice() {
    [prefix, suffix] if prefix.is_empty() => {
      let dirs: Vec<&str> = path.split("/").collect();
      dirs.len() != 1 && (path.ends_with(suffix) || simple_match(suffix, path))
    }
    [prefix, suffix] => {
      path.starts_with(prefix)
        && (path[prefix.len()..].ends_with(suffix)
          || simple_match(suffix, &path[prefix.len()..]))
    }
    [single] => path.starts_with(single),
    _ => false,
  }
}

pub fn is_matched(pathspec: &str, path: &str) -> bool {
  let (normalized_pathspec, normalized_path) =
    (normalize_path(pathspec), normalize_path(path));
  match normalized_pathspec.chars().next() {
    Some(':') => {
      todo!("magic is not implemented yet. I think it never be :)");
    }
    _ if normalized_pathspec.contains("**/") => {
      match_recursive(&normalized_pathspec, &normalized_path)
    }
    _ if normalized_pathspec.starts_with("**.") => {
      simple_match(&normalized_pathspec, &normalized_path)
    }
    _ if !normalized_pathspec.contains('/')
      && normalized_path.contains(&format!("{}/", normalized_pathspec)) =>
    {
      true
    }
    _ => simple_match(&normalized_pathspec, &normalized_path),
  }
}

pub fn normalize_path(input: &str) -> String {
  let mut result = PathBuf::new();

  for component in Path::new(input).components() {
    match component {
      std::path::Component::CurDir => {}
      _ => result.push(component),
    }
  }

  // Handle the special case where the result is a parent directory ("..")
  if result == Path::new("..") {
    return "../".to_string();
  }

  result.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_check_simple_path() {
    let pathspec = "a";
    assert_eq!(is_matched(pathspec, "a"), true);
    assert_eq!(is_matched(pathspec, "b"), false);
    assert_eq!(is_matched(pathspec, "a/b"), true);
  }

  #[test]
  fn test_check_wildcard() {
    let pathspec = "a*";
    assert_eq!(is_matched(pathspec, "a"), true);
    assert_eq!(is_matched(pathspec, "b"), false);
    assert_eq!(is_matched(pathspec, "a/b"), true);
    assert_eq!(is_matched(pathspec, "ab/c"), true);
    assert_eq!(is_matched(pathspec, "ab/c/d/e/f"), true);
  }

  #[test]
  fn test_check_wildcard_with_suffix() {
    let pathspec = "a*b";
    assert_eq!(is_matched(pathspec, "a"), false);
    assert_eq!(is_matched(pathspec, "b"), false);
    assert_eq!(is_matched(pathspec, "a/b"), true);
    assert_eq!(is_matched(pathspec, "ab/c"), false);
    assert_eq!(is_matched(pathspec, "a/c/b"), true);
  }

  #[test]
  fn test_check_blob() {
    let pathspec = "**/a.sh";
    assert_eq!(is_matched(pathspec, "a.sh"), false);
    assert_eq!(is_matched(pathspec, "b/a.sh"), true);
    assert_eq!(is_matched(pathspec, "b/c/a.sh"), true);
  }

  #[test]
  fn test_check_blob_with_prefix() {
    let pathspec = "a/**/a.sh";
    assert_eq!(is_matched(pathspec, "a.sh"), false);
    assert_eq!(is_matched(pathspec, "b/a.sh"), false);
    assert_eq!(is_matched(pathspec, "a/b/a.sh"), true);
    assert_eq!(is_matched(pathspec, "a/b/c/a.sh"), true);
  }

  #[test]
  fn test_check_blob_with_suffix() {
    let pathspec = "**/*.sh";
    assert_eq!(is_matched(pathspec, "a.sh"), false);
    assert_eq!(is_matched(pathspec, "b/a.sh"), true);
    assert_eq!(is_matched(pathspec, "b/c/a.sh"), true);
  }

  #[test]
  fn test_check_question() {
    let pathspec = "a?c";
    assert_eq!(is_matched(pathspec, "abc"), true);
    assert_eq!(is_matched(pathspec, "ac"), false);
    assert_eq!(is_matched(pathspec, "b"), false);
    assert_eq!(is_matched(pathspec, "bc"), false);
  }

  #[test]
  fn test_check_bracket() {
    let pathspec = "a[bc]d";
    assert_eq!(is_matched(pathspec, "abd"), true);
    assert_eq!(is_matched(pathspec, "acd"), true);
    assert_eq!(is_matched(pathspec, "abcd"), false);
    assert_eq!(is_matched(pathspec, "ad"), false);
    assert_eq!(is_matched(pathspec, "ab"), false);
  }

  #[test]
  fn test_normalize_path() {
    let paths = [
      ("./", ""),
      ("././././", ""),
      ("../", "../"),
      ("///////////", "/"),
      ("a/b/c", "a/b/c"),
      ("a/b/c/", "a/b/c"),
      ("a/b/c/./", "a/b/c"),
      ("a/b/c/././", "a/b/c"),
      ("a/b/c/./././", "a/b/c"),
      ("a/b/c/././././", "a/b/c"),
      ("a/b/c/./././././", "a/b/c"),
      ("a/b/c/././././././", "a/b/c"),
      ("a/b/c/./././././././", "a/b/c"),
      ("a/b/c/././././././././", "a/b/c"),
      ("a/b/c/./././././././././", "a/b/c"),
    ];

    for (path, expected) in paths.iter() {
      assert_eq!(normalize_path(path), *expected);
    }
  }
}
