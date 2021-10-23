use serde::Deserialize;
use std::fmt::{Formatter, Display, Result};

fn default_dsn() -> String {
    "postgres://postgres:postgres@localhost:5432/postgres?sslmode=disable".to_string()
}

#[derive(Deserialize, Debug)]
pub struct Config {
  #[serde(default="default_dsn")]
  pub dsn: String
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter) -> Result {
      write!(f, "Display Config\ndsn: {}", self.dsn
    )
  }
}

pub fn config() -> Config {
  match envy::from_env::<Config>() {
    Ok(config) => config,
    Err(error) => panic!("{:#?}", error)
  }
}
