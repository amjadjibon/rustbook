use serde::Deserialize;
use std::fmt::{Formatter, Display, Result};

fn default_dsn() -> String {
    "postgres://postgres:postgres@localhost:5432/postgres?sslmode=disable".to_string()
}

fn default_log_level() -> String {
  "debug".to_string()
}

#[derive(Deserialize, Debug)]
pub struct Config {
  #[serde(default="default_dsn")]
  pub dsn: String,
  #[serde(default="default_log_level")]
  pub log_level: String, 
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter) -> Result {
      write!(f, "Display Config\ndsn: {}\nlog_level: {}\n", self.dsn, self.log_level)
  }
}

pub fn config() -> Config {
  match envy::from_env::<Config>() {
    Ok(config) => config,
    Err(error) => panic!("{:#?}", error)
  }
}
