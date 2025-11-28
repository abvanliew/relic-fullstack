mod activation;
mod aspect;
pub mod component;
mod duration;
mod target;

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::rule::prelude::*;
use activation::Action;
use aspect::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub tier: Tier,
  pub title: String,
  pub summary: Option<String>,
  pub description: Option<String>,
  pub training_cost: TrainingCost,
  pub action: Action,
  pub sub_actions: Option<Vec<Action>>,
  pub order: RelicOrdering,
  pub paths: Option<Vec<PathRef>>,
  pub pick_one_keyword: Option<Vec<ObjectId>>,
}

impl Default for Skill {
  fn default() -> Self {
    Self {
      id: ObjectId::new(),
      tier: Tier::Initiate,
      title: "Undefined".into(),
      summary: None,
      description: None,
      training_cost: TrainingCost::Inherient,
      action: Default::default(),
      sub_actions: None,
      order: Default::default(),
      paths: None,
      pick_one_keyword: Default::default(),
    }
  }
}

pub mod prelude {
  pub use super::activation::Action;
  pub use super::aspect::{
    Keyword, KeywordClass, KeywordClassified, Property, RelicOrdering, TrainingCost,
  };
  pub use super::component::{SkillCard, SkillDescription, SkillTable, SkillTermDisplay};
  pub use super::duration::Duration;
  pub use super::target::Target;
  pub use super::Skill;
}
