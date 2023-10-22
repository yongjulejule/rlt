pub struct CheckIgnore {
  ignore_service: IgnoreService,
  path: String,
}

impl CheckIgnore {
  pub fn new(ignore_service: IgnoreService, path: String) -> Self {
    return Self {
      ignore_service,
      path,
    };
  }

  pub fn run(&self) -> bool {
    return self.ignore_service.is_ignored(&self.path);
  }
}
