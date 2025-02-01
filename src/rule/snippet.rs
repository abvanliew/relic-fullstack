use serde::{ Deserialize, Serialize };
use crate::rule::prelude::*;

use super::effect::StatusEffect;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct RuleSnippet {
  pub text: Option<String>,
  pub roll: Option<Roll>,
  pub outcomes: Option<Vec<RollOutcome>>,
  pub effect: Option<StatusEffect>,
}
