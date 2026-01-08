mod activation;
mod aspect;
pub mod component;
mod duration;
mod filters;
mod target;

use std::cmp::Ordering;
use std::collections::HashSet;

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
  pub paths: Option<HashSet<ObjectId>>,
  pub pick_one_keyword: Option<Vec<ObjectId>>,
  pub modifiers: Option<ModifierSet>,
  pub ranked: Option<bool>,
  pub core: Option<bool>,
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
      ranked: None,
      core: None,
    }
  }
}

impl PartialOrd for Skill {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self.tier.partial_cmp( &other.tier ) {
      Some(Ordering::Equal) => (),
      ord => return ord,
    }
    // match self.is_core().partial_cmp( &other.is_core() ) {
    //   Some(Ordering::Equal) => (),
    //   Some(ord) => return Some( ord.reverse() ),
    //   None => return None,
    // }
    match self.weight().partial_cmp( &other.weight() ) {
      Some(Ordering::Equal) => (),
      Some(ord) => return Some( ord.reverse() ),
      None => return None,
    }
    match self.order.partial_cmp( &other.order ) {
      Some(Ordering::Equal) => (),
      ord => return ord,
    }
    match self.resource_cost().partial_cmp(&other.resource_cost()) {
      Some(Ordering::Equal) => (),
      ord => return ord,
    }
    return self.title.partial_cmp( &other.title );
  }

  fn lt(&self, other: &Self) -> bool {
    matches!(self.partial_cmp(other), Some(Ordering::Less))
  }

  fn le(&self, other: &Self) -> bool {
    matches!(
      self.partial_cmp(other),
      Some(Ordering::Less | Ordering::Equal)
    )
  }

  fn gt(&self, other: &Self) -> bool {
    matches!(self.partial_cmp(other), Some(Ordering::Greater))
  }

  fn ge(&self, other: &Self) -> bool {
    matches!(
      self.partial_cmp(other),
      Some(Ordering::Greater | Ordering::Equal)
    )
  }
}

impl Ord for Skill {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.tier.cmp(&other.tier) {
      Ordering::Equal => (),
      ord => return ord,
    }
    match self.order.cmp(&other.order) {
      Ordering::Equal => (),
      ord => return ord,
    };
    return self.title.cmp(&other.title);
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
