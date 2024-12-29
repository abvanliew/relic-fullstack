use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Resource {
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

impl fmt::Display for Resource {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      Resource::Anointment => "Anointment",
      Resource::Animalism => "Animalism",
      Resource::Sanguine => "Sanguine",
      Resource::Rage => "Rage",
      Resource::Mastery => "Mastery",
      Resource::Channel => "Channel",
      Resource::Ki => "Ki",
      Resource::Virtuoso => "Virtuoso",
      Resource::MinorMana => "Minor Mana",
      Resource::ModerateMana => "Moderate Mana",
      Resource::MajorMana => "Major Mana",
    } )
  }
}

impl Resource {
  pub fn drain( &self ) -> String {
    match self {
      Resource::Anointment => "d6",
      Resource::Animalism => "d8",
      Resource::Sanguine => "d10",
      Resource::Rage => "d12",
      Resource::Mastery => "d6",
      Resource::Channel => "d8",
      Resource::Ki => "d8",
      Resource::Virtuoso => "d10",
      Resource::MinorMana => "d10",
      Resource::ModerateMana => "d10 **1-3",
      Resource::MajorMana => "d10 **1-4",
    }.into()
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCost {
  resource: Resource,
  base_cost: Option<i32>,
  charge_cost: Option<i32>,
}

impl fmt::Display for ResourceCost {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    let mut components: Vec<String> = Vec::new();
    if let Some( base ) = self.base_cost {
      components.push(
        cost_format( base, self.resource.to_string(), self.resource.drain(), false )
      );
    }
    if let Some( charge ) = self.charge_cost {
      components.push(
        cost_format( charge, self.resource.to_string(), self.resource.drain(), true )
      );
    }
    write!( f, "{}", components.join( " plus " ) )
  }
}

fn cost_format( cost: i32, name: String, drain: String, per_charge: bool ) -> String {
  format!( "{} {} ({}{}){}", cost, name, cost, drain, if per_charge { " per charge" } else { "" } )
}
