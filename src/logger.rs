#[derive(Clone)]
pub struct Logger {
  pub errored: bool
}

impl Logger {
  pub fn new() -> Self {
    Self {
      errored: false
    }
  }

  pub fn report_error(&mut self, error_type: &'static str, message: &'static str, pos: usize, line: usize) {
    println!("[{}:{}] {}: {}", line, pos + 1, error_type, message);
    self.errored = true;
  }
}