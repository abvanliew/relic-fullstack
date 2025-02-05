use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

use crate::rule::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub title: String,
  pub blurb: Option<String>,
  pub details: Option<Vec<Snippet>>,
}
