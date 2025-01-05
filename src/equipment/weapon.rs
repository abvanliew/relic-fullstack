use serde::{Deserialize, Serialize};

use crate::rule::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Weapon {
  pub style: Style,
  pub damage: DiceGroup,
  pub range: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Style {
  Melee,
  Thrown,
  Ranged,
}