use serde::{Deserialize, Serialize};
use std::fmt;

use crate::progression::prelude::*;

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
pub enum CharacterAttribute {
  Physique,
  Warfare,
  Spirit,
  Manipulation,
  Tenacity,
  Fortitude,
  Resolve,
  Insight,
}

impl fmt::Display for CharacterAttribute {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CharacterAttribute::Physique => "Physique",
        CharacterAttribute::Warfare => "Warfare",
        CharacterAttribute::Spirit => "Spirit",
        CharacterAttribute::Manipulation => "Manipulation",
        CharacterAttribute::Tenacity => "Tenacity",
        CharacterAttribute::Fortitude => "Fortitude",
        CharacterAttribute::Resolve => "Resolve",
        CharacterAttribute::Insight => "Insight",
      }
    )
  }
}

impl CharacterAttribute {
  pub fn ordered() -> Vec<CharacterAttribute> {
    return vec![
      CharacterAttribute::Physique,
      CharacterAttribute::Warfare,
      CharacterAttribute::Spirit,
      CharacterAttribute::Manipulation,
      CharacterAttribute::Tenacity,
      CharacterAttribute::Fortitude,
      CharacterAttribute::Resolve,
      CharacterAttribute::Insight,
    ];
  }

  pub fn display_as(&self) -> RankDisplay {
    return match &self {
      CharacterAttribute::Physique
      | CharacterAttribute::Warfare
      | CharacterAttribute::Spirit
      | CharacterAttribute::Manipulation => RankDisplay::Bonus,
      CharacterAttribute::Tenacity
      | CharacterAttribute::Fortitude
      | CharacterAttribute::Resolve
      | CharacterAttribute::Insight => RankDisplay::Defense,
    };
  }

  pub fn is_capacity(&self) -> bool {
    return match &self {
      CharacterAttribute::Physique
      | CharacterAttribute::Warfare
      | CharacterAttribute::Spirit
      | CharacterAttribute::Manipulation => true,
      CharacterAttribute::Tenacity
      | CharacterAttribute::Fortitude
      | CharacterAttribute::Resolve
      | CharacterAttribute::Insight => false,
    };
  }
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
