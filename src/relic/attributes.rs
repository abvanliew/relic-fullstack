use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Capability {
  Physique,
  Warfare,
  Spirit,
  Manipulation,
}

impl fmt::Display for Capability {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      Capability::Physique => "Physique",
      Capability::Warfare => "Warfare",
      Capability::Spirit => "Spirit",
      Capability::Manipulation => "Manipulation",
    } )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Defense {
  Tenacity,
  Fortitude,
  Resolve,
  Insight,
  Dodge,
}

impl fmt::Display for Defense {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      Defense::Tenacity => "Tenacity",
      Defense::Fortitude => "Fortitude",
      Defense::Resolve => "Resolve",
      Defense::Insight => "Insight",
      Defense::Dodge => "Dodge",
    } )
  }
}
