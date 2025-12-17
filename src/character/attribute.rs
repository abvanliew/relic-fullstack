use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AttributeRanks {
  pub physique: i32,
  pub warfare: i32,
  pub spirit: i32,
  pub manipulation: i32,
  pub tenacity: i32,
  pub fortitude: i32,
  pub resolve: i32,
  pub insight: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Capability {
  Physique,
  Warfare,
  Spirit,
  Manipulation,
}

impl fmt::Display for Capability {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Capability::Physique => "Physique",
        Capability::Warfare => "Warfare",
        Capability::Spirit => "Spirit",
        Capability::Manipulation => "Manipulation",
      }
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Defense {
  Tenacity,
  Fortitude,
  Resolve,
  Insight,
  Dodge,
}

impl fmt::Display for Defense {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Defense::Tenacity => "Tenacity",
        Defense::Fortitude => "Fortitude",
        Defense::Resolve => "Resolve",
        Defense::Insight => "Insight",
        Defense::Dodge => "Dodge",
      }
    )
  }
}
