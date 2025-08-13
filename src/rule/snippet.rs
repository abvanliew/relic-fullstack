use std::collections::HashSet;

use serde::{ Deserialize, Serialize };
use crate::rule::prelude::*;
use super::internal::*;
use dioxus::prelude::*;
use bson::oid::ObjectId;

pub type RulesSnippet = Vec<Snippet>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Snippet {
  pub text: Option<String>,
  pub roll: Option<Roll>,
  pub term: Option<Term>,
}

impl Snippet {
  pub fn get_keyword_ids( &self ) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    if let Some( term ) = &self.term {
      if let Some( keyword_id ) = term.keyword_id {
        ids.insert( keyword_id );
      }
    }
    return ids;
  }
}

#[component]
pub fn RulesSpippetDetail( snippets: RulesSnippet ) -> Element {
  rsx!(
    div {
      class: "inline",
      for snippet in snippets {
        SnippetDetail { snippet }
      }
    }
  )
}

#[component]
pub fn SnippetDetail( snippet: Snippet ) -> Element {
  rsx!(
    if let Some( text ) = snippet.text {
      TextSnippet { text }
    }
    if let Some( term ) = snippet.term {
      TermSnippet { term, hover: true }
    }
    if let Some( roll ) = snippet.roll {
      RollSnippet { roll }
    }
  )
}

#[component]
pub fn TextSnippet( text: String ) -> Element {
  return rsx!( div { "{text}" } )
}
