use std::{collections::HashSet, fmt};
use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{character::prelude::*, rule::snippet::SnippetSetDetails};
use super::snippet::Snippet;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct Roll {
  pub class: RollClass,
  pub capability: Capability,
  pub defense: Defense,
  pub each: Option<bool>,
  pub keyword: Option<String>,
}

impl fmt::Display for Roll {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    let keyword = self.keyword.clone().unwrap_or( "".into() );
    match &self.each {
      Some( true ) => write!( f, "Make a {} vs {} {} {} roll against each target.", self.capability, self.defense, keyword, self.class ),
      _ => write!( f, "Make a {} vs {} {} {} roll against the target.", self.capability, self.defense, keyword, self.class ),
    }
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
pub struct RollOutcome {
  pub result: RollResult,
  pub rules: Vec<Snippet>,
}

impl RollOutcome {
  pub fn get_keyword_ids( &self ) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    for rule in &self.rules {
      ids.extend( rule.get_keyword_ids() );
    }
    return ids;
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
      RollResult::Success => "Success",
      RollResult::CriticalSuccess => "Critical Success",
    } )
  }
}

#[component]
pub fn RollSnippet( roll: Roll ) -> Element {
  rsx!( span { "{roll}" } )
}

#[component]
pub fn OutcomesSnippet( outcomes: Vec<RollOutcome> ) -> Element {
  rsx!(
    div {
      class: "indent grid dim-keywords",
      for outcome in outcomes {
        div {
          class: "uv-title highlight",
          "{outcome.result}"
        }
        div {
          class: "uv-details",
          SnippetSetDetails { rules: outcome.rules }
        }
      }
    }
  )
}
