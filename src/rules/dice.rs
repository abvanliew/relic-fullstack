use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Die {
  D3,
  D4,
  D6,
  D8,
  D10,
  D12,
}

impl fmt::Display for Die {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Die::D3 => "d3",
        Die::D4 => "d4",
        Die::D6 => "d6",
        Die::D8 => "d8",
        Die::D10 => "d10",
        Die::D12 => "d12",
      }
    )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DiceSet {
  pub amount: i32,
  pub die: Die,
}

impl fmt::Display for DiceSet {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}", self.amount, self.die)
  }
}

pub type DiceGroup = Vec<DiceSet>;

#[component]
pub fn DiceGroupEntry(group: ReadSignal<DiceGroup>) -> Element {
  let display = group
    .iter()
    .map(|d| d.to_string())
    .collect::<Vec<String>>()
    .join(" + ");
  return rsx!( span { "{display}" } );
}
