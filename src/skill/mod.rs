mod activation;
mod aspect;
pub mod component;
mod cost;
mod duration;
mod filters;
mod target;

use std::cmp::Ordering;
use std::collections::HashSet;

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::modifiers::prelude::*;
use crate::rules::prelude::*;
use activation::RelicAction;
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
  pub action: RelicAction,
  #[serde(default)]
  pub sub_actions: Vec<RelicAction>,
  pub paths: Option<HashSet<ObjectId>>,
  pub modifiers: Option<ModifierSet>,
  pub ranked: Option<bool>,
  pub core: Option<bool>,
  #[serde(default)]
  pub category: Categorization,
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
      sub_actions: Vec::new(),
      paths: None,
      modifiers: None,
      ranked: None,
      core: None,
      category: Categorization::Unknown,
    }
  }
}

impl Ord for Skill {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.tier.cmp(&other.tier) {
      Ordering::Equal => (),
      ord => return ord,
    }
    match self.training_cost.cmp(&other.training_cost) {
      Ordering::Equal => (),
      ord => return ord,
    }
    match (self.is_ranked()).cmp(&(other.is_ranked())) {
      Ordering::Equal => (),
      ord => return ord,
    }
    match self.category.cmp(&other.category) {
      Ordering::Equal => (),
      ord => return ord,
    };
    match self.resource_cost().cmp(&other.resource_cost()) {
      Ordering::Equal => (),
      ord => return ord,
    }
    return self.title.cmp(&other.title);
  }
}

impl PartialOrd for Skill {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
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

pub mod prelude {
  pub use super::activation::{RelicAction, Activation};
  pub use super::aspect::{Property, TrainingCost, Categorization};
  pub use super::cost::{ResourceCost, ResourcePool};
  pub use super::duration::Duration;
  pub use super::filters::{keywords_from_skills, partitioned_sorted_skills};
  pub use super::target::{Selection, Target, TargetClass};
  pub use super::Skill;
}
