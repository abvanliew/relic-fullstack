pub mod components;
mod keystone;
mod feature;
mod skills;

use feature::Feature;

use keystone::Keystone;
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
  pub features: Option<Vec<Feature>>,
  pub keystones: Option<Vec<Keystone>>,
  pub inherient: Option<bool>,
}
