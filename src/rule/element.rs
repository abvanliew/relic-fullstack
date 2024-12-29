use serde::{ Deserialize, Serialize };
use crate::rule::prelude::*;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct RuleElement {
  pub text: Option<String>,
  pub roll: Option<Roll>,
  pub outcomes: Option<Vec<RollOutcome>>,
}