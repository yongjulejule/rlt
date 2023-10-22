use log::info;

#[allow(dead_code)]
struct RemoteContentProvider {
  pub url: String,
}

#[allow(dead_code)]
impl RemoteContentProvider {
  pub fn new(url: String) -> Self {
    info!("implemented later");

    return Self { url };
  }
}
