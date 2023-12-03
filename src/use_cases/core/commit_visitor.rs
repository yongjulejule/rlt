use crate::entities::object::CommitObject;

use super::object_service::ObjectService;

pub trait CommitVisitor {
  fn visit_commit(&self, commit: &CommitObject) -> String;
}

pub struct FormatCommitVisitor {
  is_oneline: bool,
  abbrev_commit: u8,
  stat: bool,
}

impl FormatCommitVisitor {
  pub fn new(is_oneline: bool, abbrev_commit: u8, stat: bool) -> Self {
    return Self {
      is_oneline,
      abbrev_commit,
      stat,
    };
  }
}

impl CommitVisitor for FormatCommitVisitor {
  fn visit_commit(&self, commit: &CommitObject) -> String {
    let author_field = commit.author.split(" ").collect::<Vec<&str>>();
    let author = author_field[0..author_field.len() - 2].join(" ");
    let timestamp_str = author_field[author_field.len() - 2];
    let timezone = author_field[author_field.len() - 1];

    let timestamp = match i64::from_str_radix(timestamp_str, 10) {
      Ok(timestamp) => timestamp,
      Err(_) => 0,
    };

    let naive_datetime =
      chrono::NaiveDateTime::from_timestamp_millis(timestamp * 1000).unwrap();
    let datetime: chrono::DateTime<chrono::Utc> =
      chrono::DateTime::from_naive_utc_and_offset(naive_datetime, chrono::Utc);
    let formatted_timestamp = datetime.format("%a %b %e %T %Y").to_string();

    let abbreviated_commit = commit
      .hash
      .chars()
      .take(self.abbrev_commit as usize)
      .collect::<String>();
    let stat = if self.stat {
      format!("\nStat not implemented yet")
    } else {
      "".to_string()
    };

    if self.is_oneline {
      format!(
        "{} {}",
        abbreviated_commit,
        &commit.message.lines().next().unwrap_or("")
      ) + &stat
    } else {
      format!(
        "commit {}\nAuthor: {}\nDate: {} {}\n{}\n",
        abbreviated_commit,
        author,
        formatted_timestamp,
        timezone,
        &commit.message
      ) + &stat
    }
  }
}

pub fn traverse_commits<'a>(
  service: &(dyn ObjectService + 'a),
  start_hash: &str,
  visitor: &dyn CommitVisitor,
) -> Result<Vec<String>, String> {
  let mut stack: Vec<String> = Vec::new();
  let mut result: Vec<String> = Vec::new();
  stack.push(start_hash.to_string());

  while let Some(current_hash) = stack.pop() {
    match service.find(&current_hash) {
      Ok(raw_object) => {
        let commit = CommitObject::parse(&current_hash, &raw_object.data)?;
        result.push(visitor.visit_commit(&commit));

        for parent_hash in commit.parents.iter() {
          stack.push(parent_hash.clone());
        }
      }
      Err(err) => return Err(err),
    }
  }
  Ok(result)
}
