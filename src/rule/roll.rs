use std::{collections::HashSet, fmt};
use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{character::prelude::*, rule::snippet::SnippetSetDetails};
use super::snippet::Snippet;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
#[serde(rename_all = "camelCase")]
pub struct Roll {
  pub class: RollClass,
  pub capability: Option<Capability>,
  pub defense: Option<Defense>,
  pub each: Option<bool>,
  pub keyword: Option<String>,
  pub custom_target: Option<String>,
  pub difficulty: Option<String>,
}

impl fmt::Display for Roll {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    let keyword = self.keyword.clone().unwrap_or( "".into() );
    let target = self.custom_target.clone().unwrap_or( "target".into() );
    let capability: String = match &self.capability {
      Some( capability ) => capability.to_string(),
      None => "undefined".into(),
    };
    let defense: String = match &self.defense {
      Some( defense ) => defense.to_string(),
      None => "undefined".into(),
    };
    let class = self.class.clone();
    let difficulty = self.difficulty.clone().unwrap_or( "undefined".into() );
    match ( &self.class, &self.each ) {
      ( RollClass::LuckCheck, _ ) => write!( f, "Make a {keyword} {class} difficulty {difficulty}." ),
      ( _, Some( true ) ) => write!( f, "Make a {capability} vs {defense} {keyword} {class} roll against each {target}." ),
      _ => write!( f, "Make a {capability} vs {defense} {keyword} {class} roll against the {target}." ),
    }
  }
}

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub enum RollClass {
  Attack,
  Check,
  LuckCheck,
}

impl fmt::Display for RollClass {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      RollClass::Attack => "Attack",
      RollClass::Check => "Check",
      RollClass::LuckCheck => "Luck Check",
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
