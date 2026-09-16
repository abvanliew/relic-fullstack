use serde::{Deserialize, Serialize};
use std::fmt;

use crate::progression::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterAttribute {
  Physique,
  Warfare,
  Presence,
  Manipulation,
  Fortitude,
  Resolve,
  Insight,
  Dodge,
  Expertise(String),
}

impl fmt::Display for CharacterAttribute {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CharacterAttribute::Physique => "Physique",
        CharacterAttribute::Warfare => "Warfare",
        CharacterAttribute::Presence => "Presence",
        CharacterAttribute::Manipulation => "Manipulation",
        CharacterAttribute::Fortitude => "Fortitude",
        CharacterAttribute::Resolve => "Resolve",
        CharacterAttribute::Insight => "Insight",
        CharacterAttribute::Dodge => "Dodge",
        CharacterAttribute::Expertise(title) => title,
      }
    )
  }
}

impl CharacterAttribute {
  pub fn ordered() -> Vec<CharacterAttribute> {
    return vec![
      CharacterAttribute::Physique,
      CharacterAttribute::Warfare,
      CharacterAttribute::Presence,
      CharacterAttribute::Manipulation,
      CharacterAttribute::Fortitude,
      CharacterAttribute::Resolve,
      CharacterAttribute::Insight,
      CharacterAttribute::Dodge,
    ];
  }

  pub fn iter<'a>() -> impl Iterator<Item = &'a Self> {
    return [
      CharacterAttribute::Physique,
      CharacterAttribute::Warfare,
      CharacterAttribute::Presence,
      CharacterAttribute::Manipulation,
      CharacterAttribute::Fortitude,
      CharacterAttribute::Resolve,
      CharacterAttribute::Insight,
      CharacterAttribute::Dodge,
    ].iter();
  }

  pub fn capability_iter<'a>() -> impl Iterator<Item = &'a Self> {
    return [
      CharacterAttribute::Physique,
      CharacterAttribute::Warfare,
      CharacterAttribute::Presence,
      CharacterAttribute::Manipulation,
    ].iter();
  }

  pub fn defense_iter<'a>() -> impl Iterator<Item = &'a Self> {
    return [
      CharacterAttribute::Fortitude,
      CharacterAttribute::Resolve,
      CharacterAttribute::Insight,
      CharacterAttribute::Dodge,
    ].iter();
  }

  pub fn display_as(&self) -> RankDisplay {
    return match &self {
      CharacterAttribute::Physique
      | CharacterAttribute::Warfare
      | CharacterAttribute::Presence
      | CharacterAttribute::Manipulation
      | CharacterAttribute::Expertise(_) => RankDisplay::Bonus,
      CharacterAttribute::Dodge
      | CharacterAttribute::Fortitude
      | CharacterAttribute::Resolve
      | CharacterAttribute::Insight => RankDisplay::Defense,
    };
  }

  pub fn is_capacity(&self) -> bool {
    return match &self {
      CharacterAttribute::Physique
      | CharacterAttribute::Warfare
      | CharacterAttribute::Presence
      | CharacterAttribute::Manipulation => true,
      CharacterAttribute::Dodge
      | CharacterAttribute::Fortitude
      | CharacterAttribute::Resolve
      | CharacterAttribute::Insight
      | CharacterAttribute::Expertise(_) => false,
    };
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Capability {
  Physique,
  Warfare,
  Spirit,
  Presence,
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
        Capability::Spirit => "Presence",
        Capability::Presence => "Presence",
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
