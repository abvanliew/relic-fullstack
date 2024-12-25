use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

use super::action::Action;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Tier {
  Initiate,
  Journeyman,
  Master,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EditingState {
  Ready,
  Editing,
  #[serde(rename = "In Progress")]
  InProgress,
  Concept,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
  #[serde(rename = "_id")]
  pub oid: ObjectId,
  pub tier: Tier,
  pub title: String,
  pub description: String,
  pub training_cost_id: ObjectId,
  pub training_cost: Option<String>,
  pub action: Action,
  pub order: Ordering,
  pub paths: Option<Vec<PathRef>>,
  pub editing_state: EditingState,
}

impl Skill {
  pub fn tier( &self ) -> String {
    return match &self.tier {
      Tier::Initiate => "Initiate",
      Tier::Journeyman => "Journeyman",
      Tier::Master => "Master",
    }.into();
  }

  pub fn cost( &self ) -> String {
    return match &self.training_cost {
      Some( cost ) => cost,
      None => "Unknown",
    }.into();
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ordering {
  category: i32,
  training_cost: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
  id: ObjectId,
  title: String,
}
