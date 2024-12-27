use std::fmt;
use serde::{Deserialize, Serialize};

use super::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Action {
  pub class: Option<ActivationClass>,
  pub mana_cost: Option<i32>,
  pub condition: Option<String>,
  pub cost: Option<FlowResourceCost>,
  pub duration: Option<Duration>,
  pub target: Option<String>,
  pub rules: Option<Vec<RuleElement>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ActivationClass {
  Boon,
  Action,
  Interaction,
  Reaction,
  Reflex,
  Trigger,
  ComplexAction,
  ExtendedAction,
  FreeAction,
}

impl fmt::Display for ActivationClass {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      ActivationClass::Boon => "Boon",
      ActivationClass::Action => "Action",
      ActivationClass::Interaction => "Interaction",
      ActivationClass::Reaction => "Reaction",
      ActivationClass::Reflex => "Reflex",
      ActivationClass::Trigger => "Trigger",
      ActivationClass::ComplexAction => "Complex Action",
      ActivationClass::ExtendedAction => "Extended Action",
      ActivationClass::FreeAction => "Free Action",
    } )
  }
}
