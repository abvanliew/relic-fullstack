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

impl Default for Action {
  fn default() -> Self {
    Self {
      class: Activation::Boon,
      initial: Default::default(),
      condition: Default::default(),
      cost: Default::default(),
      duration: Default::default(),
      target: Default::default(),
      rules: Default::default(),
    }
  }
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
    if let Some( snippets ) = &self.condition {
      for snippet in snippets {
        ids.extend( snippet.get_keyword_ids() );
      }
    }
    if let Some( snippets ) = &self.rules {
      for snippet in snippets {
        ids.extend( snippet.get_keyword_ids() );
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
