use serde::{ Deserialize, Serialize };

use super::snippet::RuleSnippet;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct StatusEffect {
  pub title: Option<String>,
  pub keywords: Option<Vec<String>>,
  pub rules: Option<Vec<RuleSnippet>>,
}
