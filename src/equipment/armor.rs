use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Armor {
  pub title: String,
  pub physical_resistance: i32,
  pub max_dodge: Option<i32>,
  pub speed_penalty: Option<i32>,
}