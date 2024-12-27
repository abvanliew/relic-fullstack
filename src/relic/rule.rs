use std::fmt;
use serde::{ Deserialize, Serialize };

use super::prelude::*;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub enum RuleClass {
  Text,
  Roll,
  Outcome,
}

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct RuleElement {
  pub class: RuleClass,
  pub text: Option<String>,
  pub roll: Option<Roll>,
  pub outcomes: Option<Vec<RollOutcome>>,
}

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct Roll {
  pub class: RollClass,
  pub capability: Capability,
  pub defense: Defense,
}

impl fmt::Display for Roll {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "Make a {} vs {} {} roll.", self.capability, self.defense, self.class )
  }
}

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub enum RollClass {
  Attack,
  Check,
}

impl fmt::Display for RollClass {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      RollClass::Attack => "Attack",
      RollClass::Check => "Check",
    } )
  }
}

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub enum RollResult {
  Miss,
  Hit,
  HitWilling,
  Critical,
  Failure,
  Success,
  CriticalSuccess,
}

impl fmt::Display for RollResult {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      RollResult::Miss => "Miss",
      RollResult::Hit => "Hit",
      RollResult::HitWilling => "Hit/Willing",
      RollResult::Critical => "Critical",
      RollResult::Failure => "Failure",
      RollResult::Success => "Sucess",
      RollResult::CriticalSuccess => "Critical Success",
    } )
  }
}

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct RollOutcome {
  pub result: RollResult,
  pub rules: Vec<RuleElement>,
}
