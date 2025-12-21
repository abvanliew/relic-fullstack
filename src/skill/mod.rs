mod activation;
mod aspect;
pub mod component;
mod duration;
mod filters;
mod target;

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::modifiers::prelude::*;
use crate::rules::prelude::*;
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
  pub modifiers: Option<ModifierSet>,
  pub stacking: Option<SkillStacking>,
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
      modifiers: None,
      stacking: None,
    }
  }
}

pub mod prelude {
  pub use super::activation::Action;
  pub use super::aspect::{Property, RelicOrdering, TrainingCost};
  pub use super::component::SkillTermDisplay;
  pub use super::duration::Duration;
  pub use super::filters::{keywords_from_skills, partition_skills_by_cost};
  pub use super::target::Target;
  pub use super::Skill;
}

// impl Ord for Skill {
//   fn cmp(&self, other: &Self) -> Ordering {
//     match self.tier.cmp(&other.tier) {
//       Ordering::Equal => (),
//       ord => return ord,
//     }
//     match (&self.order, &other.order) {
//       (None, None) => (),
//       (Some(_), None) => return Ordering::Less,
//       (None, Some(_)) => return Ordering::Greater,
//       (Some(self_order), Some(other_order)) => match self_order.cmp(&other_order) {
//         Ordering::Equal => (),
//         ord => return ord,
//       },
//     }
//     return self.title.cmp(&other.title);
//   }
// }
