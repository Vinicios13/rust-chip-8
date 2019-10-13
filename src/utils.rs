pub struct Config {
  filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Config {
    if args.len() < 2 {
      panic!("not enough arguments");
    }
    Config::parse_config(args)
  }

  fn parse_config(args: &[String]) -> Config {
    let filename = args[1].clone();
    Config { filename }
  }

  pub fn get_filename(&self) -> &str {
    self.filename.as_ref()
  }
}
