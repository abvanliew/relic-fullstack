use serde::{Deserialize, Serialize};
use std::fmt;

use crate::rule::prelude::*;
// use super::flow::Flow;

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
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        ResourcePool::Anointment => "Anointment",
        ResourcePool::Animalism => "Animalism",
        ResourcePool::Sanguine => "Sanguine",
        ResourcePool::Rage => "Rage",
        ResourcePool::Channel => "Channel",
        ResourcePool::Ki => "Ki",
        ResourcePool::Mastery => "Mastery",
        ResourcePool::Virtuoso => "Virtuoso",
        ResourcePool::MinorMana => "Minor Mana",
        ResourcePool::ModerateMana => "Moderate Mana",
        ResourcePool::MajorMana => "Major Mana",
      }
    )
  }
}

impl ResourcePool {
  pub fn with_drain(&self) -> String {
    return format!("{} ({})", self, self.drain());
  }

  // pub fn ordered() -> [ResourcePool; 11] { [
  //   ResourcePool::Anointment, ResourcePool::Animalism, ResourcePool::Sanguine, ResourcePool::Rage,
  //   ResourcePool::Mastery, ResourcePool::Channel, ResourcePool::Ki, ResourcePool::Virtuoso,
  //   ResourcePool::MinorMana, ResourcePool::ModerateMana, ResourcePool::MajorMana,
  // ] }

  pub fn drain(&self) -> String {
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
    }
    .into()
  }

  // pub fn flow( &self ) -> Flow {
  //   match self {
  //     ResourcePool::Animalism => Flow::Innate,
  //     ResourcePool::Anointment => Flow::Innate,
  //     ResourcePool::Rage => Flow::Innate,
  //     ResourcePool::Sanguine => Flow::Innate,
  //     ResourcePool::Mastery => Flow::Resonance,
  //     ResourcePool::Channel => Flow::Resonance,
  //     ResourcePool::Ki => Flow::Resonance,
  //     ResourcePool::Virtuoso => Flow::Resonance,
  //     ResourcePool::MinorMana => Flow::Magic,
  //     ResourcePool::ModerateMana => Flow::Magic,
  //     ResourcePool::MajorMana => Flow::Magic,
  //   }
  // }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCost {
  resource: ResourcePool,
  base_cost: Option<i32>,
  charge_cost: Option<i32>,
  optional: Option<bool>,
}

impl ResourceCost {
  pub fn format(&self, drain: bool) -> String {
    let mut components: Vec<String> = Vec::new();
    let drain_option = if drain {
      Some(self.resource.drain())
    } else {
      None
    };
    if let Some(base) = self.base_cost {
      components.push(cost_format(
        base,
        self.resource.to_string(),
        drain_option.clone(),
        false,
      ));
    }
    if let Some(charge) = self.charge_cost {
      components.push(cost_format(
        charge,
        self.resource.to_string(),
        drain_option,
        true,
      ));
    }
    return components.join(" plus ");
  }

  pub fn simple(&self) -> String {
    return self.format(false);
  }
}

impl fmt::Display for ResourceCost {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.format(true))
  }
}

fn cost_format(cost: i32, name: String, drain: Option<String>, per_charge: bool) -> String {
  let drain_details = match &drain {
    Some(text) => format!(" ({}{})", cost, text),
    _ => "".into(),
  };
  format!(
    "{} {}{}{}",
    cost,
    name,
    drain_details,
    if per_charge { " per charge" } else { "" }
  )
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PoolModifier {
  pub resource: ResourcePool,
  pub flow: Bonus<u32>,
  pub pool: Bonus<u32>,
}
