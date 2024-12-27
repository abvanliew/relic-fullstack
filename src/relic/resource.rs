use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FlowResource {
  Anointment,
  Animalism,
  Sanguine,
  Rage,
  Mastery,
  Channel,
  Ki,
  Virtuoso,
  MinorMana,
  ModerateMana,
  MajorMana,
}

impl fmt::Display for FlowResource {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      FlowResource::Anointment => "Anointment",
      FlowResource::Animalism => "Animalism",
      FlowResource::Sanguine => "Sanguine",
      FlowResource::Rage => "Rage",
      FlowResource::Mastery => "Mastery",
      FlowResource::Channel => "Channel",
      FlowResource::Ki => "Ki",
      FlowResource::Virtuoso => "Virtuoso",
      FlowResource::MinorMana => "Minor Mana",
      FlowResource::ModerateMana => "Moderate Mana",
      FlowResource::MajorMana => "Major Mana",
    } )
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FlowResourceCost {
  resource: FlowResource,
  base_cost: Option<i32>,
  charge_cost: Option<i32>,
}
