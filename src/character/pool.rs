use std::fmt;
use serde::{Deserialize, Serialize};

use crate::rule::prelude::*;
use super::flow::Flow;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourcePool {
  Anointment,
  Animalism,
  Sanguine,
  Rage,
  Channel,
  Ki,
  Mastery,
  Virtuoso,
  MinorMana,
  ModerateMana,
  MajorMana,
}

impl fmt::Display for ResourcePool {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      ResourcePool::Anointment => "Anointment",
      ResourcePool::Animalism => "Animalism",
      ResourcePool::Sanguine => "Sanguine",
      ResourcePool::Rage => "Rage",
      ResourcePool::Channel => "Channel",
      ResourcePool::Ki => "Ki",
      ResourcePool::Mastery => "Mastery",
      ResourcePool::Virtuoso => "Virtuoso",
      ResourcePool::MinorMana => "Mana",
      ResourcePool::ModerateMana => "Mana",
      ResourcePool::MajorMana => "Mana",
    } )
  }
}

impl ResourcePool {
  pub fn ordered() -> [ResourcePool; 11] { [
    ResourcePool::Anointment, ResourcePool::Animalism, ResourcePool::Sanguine, ResourcePool::Rage,
    ResourcePool::Mastery, ResourcePool::Channel, ResourcePool::Ki, ResourcePool::Virtuoso,
    ResourcePool::MinorMana, ResourcePool::ModerateMana, ResourcePool::MajorMana,
  ] }

  pub fn drain( &self ) -> String {
    match self {
      ResourcePool::Anointment => "d6",
      ResourcePool::Animalism => "d8",
      ResourcePool::Sanguine => "d10",
      ResourcePool::Rage => "d12",
      ResourcePool::Mastery => "d6",
      ResourcePool::Channel => "d8",
      ResourcePool::Ki => "d8",
      ResourcePool::Virtuoso => "d10",
      ResourcePool::MinorMana => "d10",
      ResourcePool::ModerateMana => "d10 **1-3",
      ResourcePool::MajorMana => "d10 **1-4",
    }.into()
  }

  pub fn flow( &self ) -> Flow {
    match self {
      ResourcePool::Animalism => Flow::Innate,
      ResourcePool::Anointment => Flow::Innate,
      ResourcePool::Rage => Flow::Innate,
      ResourcePool::Sanguine => Flow::Innate,
      ResourcePool::Mastery => Flow::Resonance,
      ResourcePool::Channel => Flow::Resonance,
      ResourcePool::Ki => Flow::Resonance,
      ResourcePool::Virtuoso => Flow::Resonance,
      ResourcePool::MinorMana => Flow::Magic,
      ResourcePool::ModerateMana => Flow::Magic,
      ResourcePool::MajorMana => Flow::Magic,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCost {
  resource: ResourcePool,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PoolModifier {
  pub resource: ResourcePool,
  pub flow: Bonus<u8>,
  pub pool: Bonus<u8>,
}
