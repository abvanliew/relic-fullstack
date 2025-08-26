pub mod components;
mod keystone;
mod feature;
mod skills;

use feature::Feature;

use keystone::Keystone;

use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

use crate::rule::prelude::*;
use crate::skill::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Path {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub tier: Tier,
  pub title: String,
  pub summary: Option<String>,
  pub skills: Option<Vec<Skill>>,
  pub skill_ids: Option<Vec<ObjectId>>,
  pub features: Option<Vec<Feature>>,
  pub keystones: Option<Vec<Keystone>>,
  pub inherient: Option<bool>,
  pub order: Option<RelicOrdering>,
}

impl PartialOrd for Path {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self.tier.partial_cmp(&other.tier) {
      Some(Ordering::Equal) => (),
      ord => return ord,
    }
    match self.order.partial_cmp(&other.order) {
      Some(Ordering::Equal) => (),
      ord => return ord,
    }
    self.title.partial_cmp(&other.title)
  }

  fn lt(&self, other: &Self) -> bool {
    matches!(self.partial_cmp(other), Some(Ordering::Less))
  }

  fn le(&self, other: &Self) -> bool {
    matches!(self.partial_cmp(other), Some(Ordering::Less | Ordering::Equal))
  }

  fn gt(&self, other: &Self) -> bool {
    matches!(self.partial_cmp(other), Some(Ordering::Greater))
  }

  fn ge(&self, other: &Self) -> bool {
    matches!(self.partial_cmp(other), Some(Ordering::Greater | Ordering::Equal))
  }
}