use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Die {
  D4,
  D6,
  D8,
  D10,
  D12,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DiceSet {
  die: Die,
  amount: i32,
}

pub type DiceGroup = Vec<DiceSet>;
