pub mod components;
mod selection;

use bson::oid::ObjectId;
use selection::SkillFilter;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use crate::rules::prelude::*;
use crate::skill::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Path {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub tier: Tier,
  pub title: String,
  pub summary: Option<String>,
  #[serde(default)]
  pub skill_ids: HashSet<ObjectId>,
  #[serde(default)]
  pub inherient: bool,
  #[serde(default)]
  pub selections: HashMap<SkillFilter, i32>,
  #[serde(default)]
  pub category: Categorization,
}

impl PartialOrd for Path {
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

impl Ord for Path {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.tier.cmp(&other.tier) {
      Ordering::Equal => (),
      ord => return ord,
    }
    match self.category.cmp(&other.category) {
      Ordering::Equal => (),
      ord => return ord,
    }
    return self.title.cmp(&other.title);
  }
}

pub mod prelude {
  pub use super::selection::{Constraint, PathFilter, SelectionFilter, SkillFilter};
  pub use super::Path;
}
