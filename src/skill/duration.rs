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
    let mut base: String = match ( &self.class, length != 1 ) {
      ( DurationClass::NextTurnStart, _ ) => "Until the start of your next turn".into(),
      ( DurationClass::NextTurnEnd, _ ) => "Until the end of your next turn".into(),
      ( DurationClass::WhileReserved, _ ) => "While resources are reserved".into(),
      ( DurationClass::Minutes, true ) => format!( "{length} Minutes" ),
      ( DurationClass::Minutes, false ) => format!( "1 Minute" ),
      ( DurationClass::Hours, true ) => format!( "{length} Hours" ),
      ( DurationClass::Hours, false ) => format!( "1 Hour" ),
      ( DurationClass::Days, true ) => format!( "{length} Days" ),
      ( DurationClass::Days, false ) => format!( "1 Day" ),
      ( DurationClass::Custom, _ ) => format!( "{}", self.custom.clone().unwrap_or( "".into() ) ),
    };
    if self.expendable.is_some() && self.expendable.unwrap() {
      base = format!( "{base} or until expended" )
    }
    match ( &self.upkeep, &self.upkeep_cost ) {
      ( _, Some( cost ) ) => { base = format!( "{base}, upkeep: {cost}" ) },
      ( Some( _ ), _ ) => { base = format!( "{base}, upkeep at cost" ) },
      _ => ()
    }
    return write!( f, "{base}." )
  }
}
