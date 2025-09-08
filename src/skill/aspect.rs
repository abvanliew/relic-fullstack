use std::collections::HashSet;
use std::fmt;
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use crate::rule::prelude::*;
use super::Skill;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd)]
pub enum TrainingCost {
  Inherient,
  Keystone,
  Full,
  Half,
  Cantrip,
  Spell
}

impl fmt::Display for TrainingCost {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      TrainingCost::Inherient => "Inherient",
      TrainingCost::Full => "Feature",
      TrainingCost::Half => "Minor Feature",
      TrainingCost::Keystone => "Keystone",
      TrainingCost::Cantrip => "Cantrip",
      TrainingCost::Spell => "Spell",
    } )
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct RelicOrdering {
  category: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
  id: ObjectId,
  title: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub title: String,
  pub blurb: Option<String>,
  pub rules: Option<Vec<Snippet>>,
}

impl Skill {
  pub fn get_keyword_ids( &self ) -> HashSet<ObjectId> {
    return self.action.get_keyword_ids();
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Property {
  pub title: String,
  pub rules: RuleBlocks,
}

impl Property {
  pub fn get_keyword_ids( &self ) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    for rule in &self.rules {
      ids.extend( rule.get_keyword_ids() );
    }
    return ids;
  }
}