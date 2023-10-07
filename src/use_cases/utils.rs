pub mod object_helper {
  pub fn parse_content(
    content: &[u8],
  ) -> Result<(String, usize, Vec<u8>), String> {
    let parts: Vec<&[u8]> =
      content.splitn(3, |&c| c == b' ' || c == b'\0').collect();
    if parts.len() != 3 {
      return Err("Invalid content format".to_string());
    }

    let contents_type = String::from_utf8(parts[0].to_vec())
      .map_err(|_| "Failed to parse content type".to_string())?;

    let size_str = String::from_utf8(parts[1].to_vec())
      .map_err(|_| "Failed to parse content size".to_string())?;
    let size = size_str
      .parse::<usize>()
      .map_err(|_| "Failed to parse content size".to_string())?;

    let body = parts[2].to_vec();

    Ok((contents_type, size, body))
  }

  pub fn check_object_type(
    contents_type: &str,
    object_type: &str,
  ) -> Result<(), String> {
    if contents_type != object_type {
      return Err("invalid object type".to_string());
    }
    return Ok(());
  }

  pub fn check_content_size(
    contents_size: usize,
    size: usize,
  ) -> Result<(), String> {
    if contents_size != size {
      return Err("invalid content size".to_string());
    }
    return Ok(());
  }
}

#[cfg(test)]
mod tests {
  use crate::use_cases::utils::object_helper::parse_content;

  #[test]
  fn test_parse_content() {
    let content = "blob 4\0test".as_bytes().to_vec();
    let (object_type, size, body) = parse_content(&content).unwrap();
    assert_eq!(object_type, "blob");
    assert_eq!(size, 4);
    assert_eq!(body, "test".as_bytes());
  }
}
