use std::collections::HashSet;
use std::fmt;
use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::rule::internal::*;
use crate::character::prelude::*;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
#[serde(rename_all = "camelCase")]
pub struct Roll {
  pub class: RollClass,
  pub capability: Option<Capability>,
  pub defense: Option<Defense>,
  pub each: Option<bool>,
  pub keyword: Option<String>,
  pub custom_target: Option<String>,
  pub triggered: Option<bool>,
  pub difficulty: Option<String>,
  pub opening: Option<Opening>,
  pub modifier: Option<Modifier>,
}

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub enum RollClass {
  Attack,
  Check,
  LuckCheck,
}

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub enum Opening {
  Normal,
  Lower,
  None,
}

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub enum Modifier {
  Normal,
  Advantage,
  Disadvantage,
}

impl fmt::Display for RollClass {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      RollClass::Attack => "attack",
      RollClass::Check => "check",
      RollClass::LuckCheck => "Luck Check",
    } )
  }
}

#[component]
pub fn RollSnippet( roll: Roll ) -> Element {
  let keyword = roll.keyword.clone().unwrap_or( "".into() );
  let target = match ( &roll.custom_target, roll.triggered ) {
    ( Some( custom_target ), _ ) => custom_target.clone(),
    ( _, Some( true ) ) => "triggering target".into(),
    _ => "target".into()
  };
  let capability: String = match &roll.capability {
    Some( capability ) => capability.to_string(),
    None => "undefined".into(),
  };
  let defense: String = match &roll.defense {
    Some( defense ) => defense.to_string(),
    None => "undefined".into(),
  };
  let class = roll.class.clone();
  let difficulty = roll.difficulty.clone().unwrap_or( "undefined".into() );
  let article = match &roll.each {
    Some( true ) => "each",
    _ => "the",
  };
  let opening = match &roll.opening {
    Some( Opening::None ) => "",
    Some( Opening::Lower ) => "make a ",
    _ => "Make a ",
  };
  let modifier = match &roll.modifier {
    Some( Modifier::Advantage ) => "with advantage",
    Some( Modifier::Disadvantage ) => "with disadvantage",
    _ => "",
  };
  return match ( &roll.class, &roll.each ) {
    ( RollClass::LuckCheck, _ ) => rsx!(
      div { "{opening}{keyword} " }
      div { class: "highlight", "{class}" }
      div { " difficulty {difficulty}." }
    ),
    _ => rsx!(
      div { "{opening}" }
      div { class: "highlight", "{capability}" }
      div { " vs " }
      div { class: "highlight", "{defense}" }
      div { " {keyword} {class} {modifier} against {article} {target}." }
    ),
  }
}

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct Outcome {
  pub result: RollResult,
  pub rules: RuleBlocks,
}

impl Outcome {
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
  CriticalFailure,
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
      RollResult::CriticalFailure => "Critical Failure",
      RollResult::Failure => "Failure",
      RollResult::Success => "Success",
      RollResult::CriticalSuccess => "Critical",
    } )
  }
}

#[component]
pub fn OutcomeDetail( outcomes: Vec<Outcome> ) -> Element {
  rsx!(
    div {
      class: "indent spacer grid dim-keywords",
      for outcome in outcomes {
        div {
          class: "uv-title highlight",
          "{outcome.result}"
        }
        div {
          class: "uv-details",
          RuleBlockSet { blocks: outcome.rules }
        }
      }
    }
  )
}
