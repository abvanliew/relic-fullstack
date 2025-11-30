use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

use crate::character::prelude::*;
use crate::rules::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Roll {
  pub class: RollClass,
  pub opening: Option<Opening>,

  pub keyword: Option<String>,
  pub capability: Option<Capability>,

  pub defense: Option<Defense>,
  pub alternate_defense: Option<String>,

  pub modifier: Option<Modifier>,
  pub custom_modifier: Option<String>,

  pub each: Option<bool>,
  pub custom_target: Option<String>,
  pub difficulty: Option<String>,
  pub target: Option<Target>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RollClass {
  Attack,
  Check,
  LuckCheck,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Opening {
  Normal,
  Lower,
  None,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Modifier {
  Normal,
  Advantage,
  Disadvantage,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Target {
  None,
  Triggering,
}

impl fmt::Display for RollClass {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        RollClass::Attack => "attack",
        RollClass::Check => "check",
        RollClass::LuckCheck => "Luck check",
      }
    )
  }
}

#[component]
pub fn RollSnippet(roll: Roll) -> Element {
  let defense = match (&roll.defense, &roll.alternate_defense) {
    (_, Some(defense)) => Some(defense.clone()),
    (Some(defense), _) => Some(defense.to_string()),
    (None, None) => None,
  };
  let roll_class = &roll.class;
  let article = match &roll.each {
    Some(true) => "each",
    _ => "the",
  };
  return rsx! {
    match &roll.opening {
      Some(Opening::None) => rsx! {},
      Some(Opening::Lower) => rsx! {span { " make a" }},
      _ => rsx! {span { "Make a" }},
    }
    if let Some( capability ) = roll.capability {
      span { class: "highlight", " {capability}" }
    }
    if let Some( defense ) = defense {
      span { " vs" }
      span { class: "highlight", " {defense}" }
    }
    if let Some( keyword ) = &roll.keyword {
      span { " {keyword}" }
    }
    span { " {roll_class}" }
    match ( &roll.custom_target, &roll.difficulty, &roll.target ) {
      ( Some(target), _, _ ) => rsx! { span { " against {article} {target}" } },
      ( _, Some(difficulty), _ ) => rsx! { span { " difficulty {difficulty}" } },
      ( _, _, Some( Target::Triggering ) ) => rsx! { span { " against {article} triggering target" } },
      ( _, _, Some( Target::None ) ) => rsx! {},
      _ => rsx! { span { " against {article} target" } },
    }
    match (&roll.custom_modifier, &roll.modifier) {
      ( Some(custom_modifier), _ ) => rsx! { span { " {custom_modifier}" } },
      ( _, Some(Modifier::Advantage) ) => rsx! { span { " with advantage" } },
      ( _, Some(Modifier::Disadvantage) ) => rsx! { span { " with disadvantage" } },
      _ => rsx! {},
    }
    span { "." }
  };
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
  pub result: RollResult,
  pub rules: RulesBlocks,
}

impl Outcome {
  pub fn get_keyword_ids(&self) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    for rule in &self.rules {
      ids.extend(rule.get_keyword_ids());
    }
    return ids;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RollResult {
  Botch,
  Miss,
  Hit,
  Critical,
  CriticalFailure,
  Failure,
  Success,
  CriticalSuccess,
}

impl fmt::Display for RollResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        RollResult::CriticalFailure | RollResult::Botch => "Botch",
        RollResult::Miss => "Miss",
        RollResult::Failure => "Failure",
        RollResult::Hit => "Hit",
        RollResult::Success => "Success",
        RollResult::Critical | RollResult::CriticalSuccess => "Critical",
      }
    )
  }
}

#[component]
pub fn OutcomeDetail(outcomes: Vec<Outcome>) -> Element {
  rsx!(
    for outcome in outcomes {
      div {
        class: "uv-title indent highlight",
        "{outcome.result}"
      }
      div {
        class: "uv-details",
        RulesBlockSet { blocks: outcome.rules }
      }
    }
  )
}
