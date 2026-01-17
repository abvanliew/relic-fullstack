use serde::{Deserialize, Serialize};
use std::fmt;

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
        ResourcePool::MinorMana => "Mana",
        ResourcePool::ModerateMana => "Mana",
        ResourcePool::MajorMana => "Mana",
      }
    )
  }
}

impl ResourcePool {
  pub fn with_drain(&self) -> String {
    return format!("{} ({})", self, self.drain());
  }

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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCost {
  resource: ResourcePool,
  base_cost: Option<i32>,
  charge_cost: Option<i32>,
  optional: Option<bool>,
  max_charges: Option<i32>,
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
        None,
      ));
    }
    if let Some(charge) = self.charge_cost {
      components.push(cost_format(
        charge,
        self.resource.to_string(),
        drain_option,
        true,
        self.max_charges,
      ));
    }
    return components.join(" + ");
  }

  pub fn simple(&self) -> String {
    return self.format(false);
  }

  pub fn minimum_resource_cost(&self) -> i32 {
    match (&self.base_cost, &self.charge_cost) {
      (Some(cost), _) => *cost,
      (_, Some(cost)) => *cost,
      _ => 0,
    }
  }
}

impl fmt::Display for ResourceCost {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.format(true))
  }
}

fn cost_format(
  cost: i32, 
  name: String, 
  drain: Option<String>, 
  per_charge: bool,
  max_charges: Option<i32>,
) -> String {
  let drain_details = match &drain {
    Some(text) => format!(" ({}{})", cost, text),
    _ => "".into(),
  };
  let charge_limit = match max_charges {
    Some( limit ) => format!( " up to {} charges", limit  ),
    None => "".into(),
  };
  format!(
    "{} {}{}{}{}",
    cost,
    name,
    drain_details,
    if per_charge { " per charge" } else { "" },
    charge_limit,
  )
}
