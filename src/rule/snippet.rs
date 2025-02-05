use serde::{ Deserialize, Serialize };
use crate::rule::prelude::*;
use crate::rule::roll::{OutcomesSnippet, RollSnippet};
use crate::rule::status_effect::StatusEffectSnippet;
use crate::rule::term::TermSnippet;
use dioxus::prelude::*;

use super::status_effect::StatusEffect;
use super::term::RuleTerm;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct Snippet {
  pub text: Option<String>,
  pub roll: Option<Roll>,
  pub outcomes: Option<Vec<RollOutcome>>,
  pub effect: Option<StatusEffect>,
  pub term: Option<RuleTerm>,
}

#[component]
pub fn SnippetSetDetails( rules: Vec<Snippet> ) -> Element {
  rsx!(
    for rule in rules {
      SnippetDetails { rule }
    }
  )
}

#[component]
pub fn SnippetDetails( rule: Snippet ) -> Element {
  rsx!(
    if let Some( text ) = rule.text {
      TextSnippet { text }
    }
    if let Some( term ) = rule.term {
      TermSnippet { term }
    }
    if let Some( roll ) = rule.roll {
      RollSnippet { roll }
    }
    if let Some( outcomes ) = rule.outcomes {
      OutcomesSnippet { outcomes }
    }
    if let Some( effect ) = rule.effect {
      StatusEffectSnippet { effect }
    }
  )
}

#[component]
pub fn TextSnippet( text: String ) -> Element {
  rsx!( span { "{text}" } )
}
