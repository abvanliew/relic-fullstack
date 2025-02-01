use std::fmt;
use serde::{Deserialize, Serialize};

use crate::character::prelude::*;
use crate::rule::prelude::*;
use crate::skill::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Action {
  pub class: Activation,
  pub initial: Option<bool>,
  pub condition: Option<String>,
  pub cost: Option<ResourceCost>,
  pub duration: Option<Duration>,
  pub target: Option<Target>,
  pub rules: Option<Vec<RuleSnippet>>,
}

impl Action {
  pub fn activation( &self ) -> String {
    return match self.initial {
      Some( true ) => format!( "Initial {}", self.class ),
      _ => format!( "{}", self.class ),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Activation {
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

impl fmt::Display for Activation {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      Activation::Boon => "Boon",
      Activation::Action => "Action",
      Activation::Interaction => "Interaction",
      Activation::Reaction => "Reaction",
      Activation::Reflex => "Reflex",
      Activation::Trigger => "Trigger",
      Activation::ComplexAction => "Complex Action",
      Activation::ExtendedAction => "Extended Action",
      Activation::FreeAction => "Free Action",
    } )
  }
}
