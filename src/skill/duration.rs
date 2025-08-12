use std::fmt;
use serde::{Deserialize, Serialize};

use crate::character::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
  pub class: DurationClass,
  pub length: Option<i32>,
  pub expendable: Option<bool>,
  pub upkeep: Option<bool>,
  pub upkeep_cost: Option<ResourceCost>,
  pub custom: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DurationClass {
  NextTurnStart,
  NextTurnEnd,
  WhileReserved,
  Minutes,
  Hours,
  Days,
  Custom,
}

impl fmt::Display for Duration {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    let length = match self.length {
      Some( amount ) => amount,
      None => 1,
    };
    let mut base: String = match ( &self.class, length != 1, &self.upkeep_cost ) {
      ( DurationClass::Custom, _, _ ) => format!( "{}", self.custom.clone().unwrap_or( "".into() ) ),
      ( DurationClass::NextTurnStart, _, _ ) => "Until the start of the next round".into(),
      ( DurationClass::NextTurnEnd, _, _ ) => "Until the end of the next round".into(),
      ( DurationClass::WhileReserved, _, Some( cost ) ) => format!( "While {} is reserved", cost.simple() ),
      ( DurationClass::Minutes, true, _ ) => format!( "{length} Minutes" ),
      ( DurationClass::Minutes, false, _ ) => format!( "1 Minute" ),
      ( DurationClass::Hours, true, _ ) => format!( "{length} Hours" ),
      ( DurationClass::Hours, false, _ ) => format!( "1 Hour" ),
      ( DurationClass::Days, true, _ ) => format!( "{length} Days" ),
      ( DurationClass::Days, false, _ ) => format!( "1 Day" ),
      ( _, _ , _ ) => format!( "Undefined" ),
    };
    if self.expendable.is_some() && self.expendable.unwrap() {
      base = format!( "{base} or until expended" )
    }
    match ( &self.upkeep, &self.upkeep_cost, self.class != DurationClass::WhileReserved ) {
      ( _, Some( cost ), true ) => { base = format!( "{base}, upkeep: {cost}" ) },
      ( Some( _ ), _, true ) => { base = format!( "{base}, upkeep at cost" ) },
      _ => ()
    }
    return write!( f, "{base}" )
  }
}
