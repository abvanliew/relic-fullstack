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
  Debuff,
}

pub trait KeywordClassified {
  fn get_keyword_ids(&self) -> HashSet<ObjectId>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub title: String,
  pub class: Option<KeywordClass>,
  pub blurb: Option<String>,
  pub rules: Option<Vec<Snippet>>,
}

impl Default for Keyword {
  fn default() -> Self {
    Self {
      id: Default::default(),
      title: "undefined".into(),
      class: Default::default(),
      blurb: Default::default(),
      rules: Default::default(),
    }
  }
}

impl Skill {
  pub fn minor_feature_cost(&self) -> u32 {
    match &self.training_cost {
      TrainingCost::Inherient | TrainingCost::Keystone => 0,
      TrainingCost::Full | TrainingCost::Spell => 2,
      TrainingCost::Half | TrainingCost::Cantrip => 1,
    }
  }
}

impl KeywordClassified for Skill {
  fn get_keyword_ids(&self) -> HashSet<ObjectId> {
    return self.action.get_keyword_ids();
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Property {
  pub term: Term,
  pub rules: RuleBlocks,
}

impl Property {
  pub fn get_keyword_ids(&self) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();

    if let Some(id) = self.term.keyword_id {
      ids.insert(id);
    }
    for rule in &self.rules {
      ids.extend(rule.get_keyword_ids());
    }
    return ids;
  }
}
