mod activation;
mod aspect;
pub mod component;
mod duration;
mod target;

use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

use activation::Action;
use aspect::*;
use crate::rule::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
  pub order: SkillOrdering,
  pub paths: Option<Vec<PathRef>>,
}

impl Default for Skill {
  fn default() -> Self {
    Self {
      id: ObjectId::new(),
      tier: Tier::Initiate,
      title: "Undefined".into(),
      summary: Default::default(),
      description: Default::default(),
      training_cost: TrainingCost::Inherient,
      action: Default::default(),
      order: Default::default(),
      paths: Default::default(),
    }
  }
}

pub mod prelude {
  pub use super::Skill;
  pub use super::aspect::{Keyword,TrainingCost};
  pub use super::activation::Action;
  pub use super::duration::Duration;
  pub use super::component::{SkillDescription, SkillTable};
  pub use super::target::Target;
}