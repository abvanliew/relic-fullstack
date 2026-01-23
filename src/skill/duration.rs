use serde::{Deserialize, Serialize};
use std::fmt;

use super::cost::ResourceCost;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
  pub class: DurationClass,
  pub length: Option<i32>,
  pub expendable: Option<bool>,
  pub upkeep: Option<bool>,
  pub upkeep_cost: Option<ResourceCost>,
  pub custom: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum DurationClass {
  Custom,
  NextTurnStart,
  NextTurnEnd,
  WhileReserved,
  Minutes,
  Hours,
  Days,
  Weeks,
  Months,
}

impl Duration {
  pub fn base(&self) -> String {
    let length = match self.length {
      Some(amount) => amount,
      None => 1,
    };
    let mut base: String = match (&self.class, length != 1, &self.upkeep_cost) {
      (DurationClass::Custom, _, _) => {
        format!("{}", self.custom.clone().unwrap_or("".into()))
      },
      (DurationClass::NextTurnStart, _, _) => "Until the start of the next round".into(),
      (DurationClass::NextTurnEnd, _, _) => "Until the end of the next round".into(),
      (DurationClass::WhileReserved, _, Some(cost)) => {
        format!("While {} is reserved", cost.simple())
      },
      (DurationClass::Minutes, true, _) => format!("{length} Minutes"),
      (DurationClass::Minutes, false, _) => format!("1 Minute"),
      (DurationClass::Hours, true, _) => format!("{length} Hours"),
      (DurationClass::Hours, false, _) => format!("1 Hour"),
      (DurationClass::Days, true, _) => format!("{length} Days"),
      (DurationClass::Days, false, _) => format!("1 Day"),
      (DurationClass::Weeks, true, _) => format!("{length} Weeks"),
      (DurationClass::Weeks, false, _) => format!("1 Week"),
      (DurationClass::Months, true, _) => format!("{length} Months"),
      (DurationClass::Months, false, _) => format!("1 Month"),
      (_, _, _) => format!("Undefined"),
    };
    if self.expendable.is_some() && self.expendable.unwrap() {
      base = format!("{base} or until expended")
    }
    return base;
  }

  pub fn upkeep(&self) -> Option<String> {
    match (
      &self.upkeep,
      &self.upkeep_cost,
      self.class != DurationClass::WhileReserved,
    ) {
      (_, Some(cost), true) => Some(cost.to_string()),
      (Some(_), _, true) => Some("At cost".into()),
      _ => None,
    }
  }
}

impl fmt::Display for Duration {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let base = self.base();
    match &self.upkeep() {
      Some(upkeep) => write!(f, "{base} {upkeep}"),
      None => write!(f, "{base}"),
    }
  }
}
