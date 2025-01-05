use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Armor {
  pub physical_resistance: i32,
  pub max_dodge: Option<i32>,
  pub speed_penalty: Option<i32>,
}