use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{progression::fixed::BASE_DEFENSE, rules::components::Modifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RankDisplay {
  Flat,
  Bonus,
  Defense,
}

impl RankDisplay {
  pub fn from_value(&self, value: i32) -> Element {
    match &self {
      RankDisplay::Flat => rsx! { "{value}" },
      RankDisplay::Bonus => rsx! { div { Modifier { value } } },
      RankDisplay::Defense => {
        let def = value + BASE_DEFENSE;
        rsx! { div { "{def}" } }
      },
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbilityClass {
  Capability,
  Defense,
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterAbility {
  Physique,
  Warfare,
  Presence,
  Manipulation,
  Fortitude,
  Resolve,
  Insight,
  Dodge,
}


impl fmt::Display for CharacterAbility {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CharacterAbility::Physique => "Physique",
        CharacterAbility::Warfare => "Warfare",
        CharacterAbility::Presence => "Focus",
        CharacterAbility::Manipulation => "Manipulation",
        CharacterAbility::Fortitude => "Fortitude",
        CharacterAbility::Resolve => "Resolve",
        CharacterAbility::Insight => "Insight",
        CharacterAbility::Dodge => "Dodge",
      }
    )
  }
}

impl CharacterAbility {
  pub fn capability_iter<'a>() -> impl Iterator<Item = &'a Self> {
    return [
      CharacterAbility::Physique,
      CharacterAbility::Warfare,
      CharacterAbility::Presence,
      CharacterAbility::Manipulation,
    ].iter();
  }

  pub fn defense_iter<'a>() -> impl Iterator<Item = &'a Self> {
    return [
      CharacterAbility::Fortitude,
      CharacterAbility::Resolve,
      CharacterAbility::Insight,
      CharacterAbility::Dodge,
    ].iter();
  }

  pub fn class(&self) -> AbilityClass {
    return match &self {
      CharacterAbility::Physique
      | CharacterAbility::Warfare
      | CharacterAbility::Presence
      | CharacterAbility::Manipulation => AbilityClass::Capability,
      CharacterAbility::Dodge
      | CharacterAbility::Fortitude
      | CharacterAbility::Resolve
      | CharacterAbility::Insight => AbilityClass::Defense,
    };
  }

  pub fn display_as(&self) -> RankDisplay {
    return match &self {
      CharacterAbility::Physique
      | CharacterAbility::Warfare
      | CharacterAbility::Presence
      | CharacterAbility::Manipulation => RankDisplay::Bonus,
      CharacterAbility::Dodge
      | CharacterAbility::Fortitude
      | CharacterAbility::Resolve
      | CharacterAbility::Insight => RankDisplay::Defense,
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
        Capability::Spirit => "Focus",
        Capability::Presence => "Focus",
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
