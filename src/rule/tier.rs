use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd, Ord, Eq)]
pub enum Tier {
  Initiate,
  Journeyman,
  Master,
}

impl fmt::Display for Tier {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Tier::Initiate => "Initiate",
        Tier::Journeyman => "Journeyman",
        Tier::Master => "Master",
      }
    )
  }
}
