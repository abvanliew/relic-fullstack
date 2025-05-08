use std::collections::HashSet;

use serde::{ Deserialize, Serialize };
use crate::rule::prelude::*;
use crate::rule::roll::{OutcomesSnippet, RollSnippet};
use crate::rule::status_effect::StatusEffectSnippet;
use crate::rule::term::TermSnippet;
use dioxus::prelude::*;
use bson::oid::ObjectId;

use super::status_effect::StatusEffect;
use super::term::RuleTerm;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct Snippet {
  pub text: Option<String>,
  pub roll: Option<Roll>,
  pub outcomes: Option<Vec<RollOutcome>>,
  pub effect: Option<StatusEffect>,
  pub term: Option<RuleTerm>,
  pub items: Option<Vec<Vec<Snippet>>>,
  pub p: Option<bool>,
}

impl Snippet {
  pub fn get_keyword_ids( &self ) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    if let Some( term ) = &self.term {
      if let Some( keyword_id ) = term.keyword_id {
        ids.insert( keyword_id );
      }
    }
    if let Some( outcomes ) = &self.outcomes {
      for outcome in outcomes {
        ids.extend( outcome.get_keyword_ids() );
      }
    }
    if let Some( items ) = &self.items {
      for item in items {
        for rule in item {
          ids.extend( rule.get_keyword_ids() );
        }
      }
    }
    if let Some( effect ) = &self.effect {
      ids.extend( effect.get_keyword_ids() );
    }
    return ids;
  }
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
    match rule.p {
      Some( true ) => rsx!( p {} ),
      _ => rsx!()
    }
    if let Some( text ) = rule.text {
      TextSnippet { text }
    }
    if let Some( term ) = rule.term {
      TermSnippet { term, hover: true }
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
    if let Some( items ) = rule.items {
      SnippetList { items }
    }
  )
}

#[component]
pub fn TextSnippet( text: String ) -> Element {
  return rsx!( span { "{text}" } );
}

#[component]
pub fn SnippetList( items: Vec<Vec<Snippet>> ) -> Element {
  rsx!(
    ul {
      for rules in items {
        li { SnippetSetDetails { rules } }
      }
    }
  )
}
