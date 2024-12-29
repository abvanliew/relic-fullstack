use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Tier {
  Initiate,
  Journeyman,
  Master,
}

impl fmt::Display for Tier {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
    Tier::Initiate => "Initiate",
    Tier::Journeyman => "Journeyman",
    Tier::Master => "Master",
    } )
  }
}
