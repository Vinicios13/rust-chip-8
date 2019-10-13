use std::fmt;

pub struct Config {
  filename: String,
}

pub struct ConfigError {
  message: String,
}

impl ConfigError {
  pub fn new(message: &str) -> ConfigError {
    ConfigError {
      message: message.to_owned(),
    }
  }
}

impl fmt::Display for ConfigError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl fmt::Debug for ConfigError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ConfigError {{ message: {} }}", self.message)
  }
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, ConfigError> {
    if args.len() < 2 {
      Err(ConfigError::new("not enough arguments"))
    } else {
      Ok(Config::parse_config(args))
    }
  }

  fn parse_config(args: &[String]) -> Config {
    let filename = args[1].clone();
    Config { filename }
  }

  pub fn get_filename(&self) -> &str {
    self.filename.as_ref()
  }
}
