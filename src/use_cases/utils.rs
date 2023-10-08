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
