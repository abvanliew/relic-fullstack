use serde::{Deserialize, Serialize};

use crate::rule::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
  pub style: Style,
  pub damage_dice: DiceGroup,
  pub damage_type: String,
  pub range: Option<i32>,
  pub block: Option<i32>,
  pub keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Style {
  Melee,
  Thrown,
  Ranged,
}