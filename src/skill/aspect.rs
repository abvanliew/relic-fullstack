use super::Skill;
use crate::rule::prelude::*;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd, Eq)]
pub enum TrainingCost {
  Inherient,
  Keystone,
  Full,
  Half,
  Cantrip,
  Spell,
}

impl fmt::Display for TrainingCost {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        TrainingCost::Inherient => "Inherient",
        TrainingCost::Full => "Feature",
        TrainingCost::Half => "Minor Feature",
        TrainingCost::Keystone => "Keystone",
        TrainingCost::Cantrip => "Cantrip",
        TrainingCost::Spell => "Spell",
      }
    )
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EditingState {
  Ready,
  Editing,
  #[serde(rename = "In Progress")]
  InProgress,
  Concept,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, PartialOrd, Ord, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelicOrdering {
  category: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
  id: ObjectId,
  title: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum KeywordClass {
  Classifier,
  Stack,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub class: Option<KeywordClass>,
  pub title: String,
  pub blurb: Option<String>,
  pub rules: Option<Vec<Snippet>>,
}

impl Skill {
  pub fn get_keyword_ids(&self) -> HashSet<ObjectId> {
    return self.action.get_keyword_ids();
  }

  pub fn minor_feature_cost(&self) -> u32 {
    match &self.training_cost {
      TrainingCost::Inherient | TrainingCost::Keystone => 0,
      TrainingCost::Full | TrainingCost::Spell => 2,
      TrainingCost::Half | TrainingCost::Cantrip => 1,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Property {
  pub title: String,
  pub rules: RuleBlocks,
}

impl Property {
  pub fn get_keyword_ids(&self) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    for rule in &self.rules {
      ids.extend(rule.get_keyword_ids());
    }
    return ids;
  }
}
