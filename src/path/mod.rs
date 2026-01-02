pub mod components;
mod keystone;
mod selection;

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use selection::PathSelectionClass;

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
  pub skill_ids: Option<Vec<ObjectId>>,
  pub inherient: Option<bool>,
  pub order: Option<RelicOrdering>,
  pub selections: Option<HashMap<PathSelectionClass, i32>>,
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
    match (&self.order, &other.order) {
      (None, None) => (),
      (Some(_), None) => return Ordering::Less,
      (None, Some(_)) => return Ordering::Greater,
      (Some(self_order), Some(other_order)) => match self_order.cmp(&other_order) {
        Ordering::Equal => (),
        ord => return ord,
      },
    }
    return self.title.cmp(&other.title);
  }
}

pub mod prelude {
  pub use super::Path;
  pub use super::selection::{PathSelectionClass, PathFilter};
}
