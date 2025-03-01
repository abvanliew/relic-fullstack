use std::{collections::HashSet, fmt};
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

use super::Skill;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TrainingCost {
  Inherient,
  Full,
  Half,
  Keystone,
  Cantrip,
  Spell
}

impl fmt::Display for TrainingCost {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      TrainingCost::Inherient => "Inherient",
      TrainingCost::Full => "Full Feature",
      TrainingCost::Half => "Half Feature",
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillOrdering {
  category: i32,
  training_cost: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
  id: ObjectId,
  title: String,
}

impl Skill {
  pub fn get_keyword_ids( &self ) -> HashSet<ObjectId> {
    return self.action.get_keyword_ids();
  }
}
