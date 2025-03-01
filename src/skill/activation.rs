use std::collections::HashSet;
use std::fmt;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::character::prelude::*;
use crate::rule::prelude::*;
use crate::skill::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Action {
  pub class: Activation,
  pub initial: Option<bool>,
  pub condition: Option<Vec<Snippet>>,
  pub cost: Option<ResourceCost>,
  pub duration: Option<Duration>,
  pub target: Option<Target>,
  pub rules: Option<Vec<Snippet>>,
}

impl Action {
  pub fn activation( &self ) -> String {
    return match self.initial {
      Some( true ) => format!( "Initial {}", self.class ),
      _ => format!( "{}", self.class ),
    }
  }
  
  pub fn get_keyword_ids( &self ) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    if let Some( condition ) = &self.condition {
      for rule in condition {
        ids.extend( rule.get_keyword_ids() );
      }
    }
    if let Some( condition ) = &self.condition {
      for rule in condition {
        ids.extend( rule.get_keyword_ids() );
      }
    }
    return ids;
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
