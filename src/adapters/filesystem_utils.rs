use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

// one possible implementation of walking a directory only visiting files
pub fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        visit_dirs(&path, cb)?;
      } else {
        cb(&entry);
      }
    }
  }
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;
  use std::path::PathBuf;

  #[test]
  fn test_visit_dirs() {
    let mut files = Vec::new();
    let cb = &mut |entry: &DirEntry| {
      let path = entry.path();
      let file_name = path.file_name().unwrap().to_str().unwrap();
      files.push(file_name.to_string());
    };
    let dir = PathBuf::from("./src");
    visit_dirs(&dir, cb).unwrap();
    println!("{:?}", files);
    // assert_eq!(files.len(), 2);
    // assert_eq!(files[0], "test.txt");
    // assert_eq!(files[1], "test2.txt");
  }
}
