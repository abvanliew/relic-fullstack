use serde::{Deserialize, Serialize};
use std::fmt;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RankDisplay {
  Bonus,
  Defense,
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
