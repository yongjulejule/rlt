#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
  pub object_type: String,
  pub hash: String,
  pub data: Vec<u8>,
  pub size: usize,
}

impl Object {
  pub fn new(object_type: &str, hash: &str, data: &[u8], size: usize) -> Self {
    Self {
      object_type: object_type.to_string(),
      hash: hash.to_string(),
      data: data.to_vec(),
      size,
    }
  }
}

// pub struct BlobObject {
//   pub object_type: String,
//   pub data: Vec<u8>,
//   pub hash: String,
//   pub size: usize,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeObject {
  pub object_type: String,
  pub hash: String,
  pub entries: Vec<TreeElement>,
  pub size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeElement {
  pub mode: String,
  pub name: String,
  pub hash: String,
}

impl TreeObject {
  pub fn parse(hash: &str, content: &[u8]) -> Result<TreeObject, String> {
    let mut entries = Vec::new();
    let mut i = 0;
    let content_len = content.len();

    while i < content_len {
      let mode_end = content[i..]
        .iter()
        .position(|&b| b == b' ')
        .ok_or("Failed to find space after mode")?
        + i;
      let mode = std::str::from_utf8(&content[i..mode_end]).map_err(|_| {
        format!("Failed to parse mode as UTF-8: {:?}", &content[i..mode_end])
      })?;

      let name_end = content[mode_end + 1..]
        .iter()
        .position(|&b| b == b'\0')
        .ok_or("Failed to find null terminator after name")?
        + mode_end
        + 1;

      let name = std::str::from_utf8(&content[mode_end + 1..name_end])
        .map_err(|_| {
          format!(
            "Failed to parse name as UTF-8: {:?}",
            &content[mode_end + 1..name_end]
          )
        })?;

      let hash_start = name_end + 1;
      let hash_end = hash_start + 20; // SHA-1 hash size is 20 bytes
      if hash_end > content_len {
        return Err("Incomplete hash".into());
      }
      let hash = &content[hash_start..hash_end]
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

      entries.push(TreeElement {
        mode: mode.to_string(),
        name: name.to_string(),
        hash: hash.to_string(),
      });

      i = hash_end;
    }

    Ok(TreeObject {
      object_type: "tree".to_string(),
      hash: hash.to_string(),
      entries,
      size: content.len(),
    })
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitObject {
  pub object_type: String,
  pub hash: String,
  pub tree: String,
  pub parents: Vec<String>,
  pub author: String,
  pub committer: String,
  pub message: String,
  pub size: usize,
  pub gpg_sig: Option<String>,
}

impl CommitObject {
  pub fn parse(hash: &str, data: &str) -> Result<Self, String> {
    let mut lines = data.lines();
    let mut tree = String::new();
    let mut parents = Vec::new();
    let mut author = String::new();
    let mut committer = String::new();
    let mut gpg_sig = None;
    let mut message = String::new();
    let mut reading_message = false;
    let mut reading_gpg_sig = false;
    let mut gpg_sig_content = String::new();

    while let Some(line) = lines.next() {
      if reading_message {
        message += line;
        message.push('\n');
      } else if reading_gpg_sig {
        gpg_sig_content.push_str(line);
        gpg_sig_content.push('\n');
        if line == "-----END PGP SIGNATURE-----" {
          reading_gpg_sig = false;
          gpg_sig = Some(gpg_sig_content.clone());
        }
      } else {
        if line.starts_with("tree ") {
          tree = line[5..].to_string();
        } else if line.starts_with("parent ") {
          parents.push(line[7..].to_string());
        } else if line.starts_with("author ") {
          author = line[7..].to_string();
        } else if line.starts_with("committer ") {
          committer = line[10..].to_string();
        } else if line.starts_with("gpgsig ") {
          reading_gpg_sig = true;
          gpg_sig_content.push_str(&line[7..]);
          gpg_sig_content.push('\n');
        } else if line.is_empty() {
          reading_message = true;
        }
      }
    }

    // Remove trailing newline from message if present
    if message.ends_with('\n') {
      message.pop();
    }

    if tree.is_empty() || author.is_empty() || committer.is_empty() {
      return Err("Missing essential fields in commit object data".to_string());
    }

    Ok(Self {
      object_type: "commit".to_string(),
      hash: hash.to_string(),
      tree,
      parents,
      author,
      committer,
      message,
      size: data.len(),
      gpg_sig,
    })
  }
}

//  asd
#[cfg(test)]
mod tests {
  use crate::entities::object::CommitObject;

  use super::{TreeElement, TreeObject};

  #[test]
  fn parse_commit() {
    let hash = "test-hash";
    let content = "tree test-tree\nparent test-parent\nauthor test-author\ncommitter test-committer\n\nmessage";

    let expected = CommitObject {
      tree: "test-tree".to_string(),
      parents: vec!["test-parent".to_string()],
      hash: hash.to_string(),
      object_type: "commit".to_string(),
      author: "test-author".to_string(),
      committer: "test-committer".to_string(),
      message: "message".to_string(),
      size: content.len(),
      gpg_sig: None,
    };

    let result = CommitObject::parse(hash, &content).unwrap();

    assert_eq!(result, expected);
  }

  #[test]
  fn parse_tree() {
    let hash = "test-hash";
    let content = b"100644 test-name\0test-hash###########40000 test-dir\0test-hash2##########";

    let expected = TreeObject {
      hash: hash.to_string(),
      object_type: "tree".to_string(),
      entries: vec![
        TreeElement {
          mode: "100644".to_string(),
          name: "test-name".to_string(),
          hash: "test-hash###########".to_string(),
        },
        TreeElement {
          mode: "40000".to_string(),
          name: "test-dir".to_string(),
          hash: "test-hash2##########".to_string(),
        },
      ],
      size: content.len(),
    };

    let result = TreeObject::parse(hash, content).unwrap();

    assert_eq!(result, expected);
  }
}
