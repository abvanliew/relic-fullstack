use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Action {
  type_id: ObjectId,
  #[serde(rename = "type")]
  pub action_type: Option<String>,
  mana_cost: Option<i32>,
  pub duration: Option<Duration>,
  pub target: Option<String>,
  pub body: Option<String>,
}

impl Action {
  pub fn display_type( &self ) -> String {
    return match &self.action_type {
      Some( value ) => value.clone(),
      None => "Unknown".into(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
  #[serde(rename = "type")]
  pub duration_type: DurationType,
  pub duration_length: Option<i32>,
  pub expendable: Option<bool>,
}

impl Duration {
  pub fn display( &self ) -> String {
    let length = match self.duration_length {
      Some( amount ) => amount,
      None => 1,
    };
    let base: String = match ( &self.duration_type, length != 1 ) {
      ( DurationType::NextTurnStart, _ ) => "Until the start of your next turn".into(),
      ( DurationType::NextTurnEnd, _ ) => "Until the end of your next turn".into(),
      ( DurationType::WhileReserved, _ ) => "While resources are reserved".into(),
      ( DurationType::Minutes, true ) => format!( "{length} Minutes" ),
      ( DurationType::Minutes, false ) => format!( "1 Minute" ),
      ( DurationType::Hours, true ) => format!( "{length} Hours" ),
      ( DurationType::Hours, false ) => format!( "1 Hour" ),
      ( DurationType::Days, true ) => format!( "{length} Days" ),
      ( DurationType::Days, false ) => format!( "1 Day" ),
    };
    if self.expendable.is_some() && self.expendable.unwrap() {
      return format!( "{base} or until expended" )
    }
    return base;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DurationType {
  NextTurnStart,
  NextTurnEnd,
  WhileReserved,
  Minutes,
  Hours,
  Days,
}
