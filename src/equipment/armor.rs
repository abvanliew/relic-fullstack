use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Armor {
  pub title: String,
  pub physical_resistance: i32,
  pub tenacity_requirement: i32,
  pub speed_penalty: Option<i32>,
}
