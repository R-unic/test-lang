pub struct Logger;

impl Logger {
  pub fn report_error(&self, error_type: &str, message: &str) {
    println!("{}: {}", error_type, message);
  }
}

pub static LOGGER: Logger = Logger {};