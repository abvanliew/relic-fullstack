use std::collections::HashSet;

use super::internal::*;
use crate::rule::prelude::*;
use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub type RulesSnippet = Vec<Snippet>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Snippet {
  pub text: Option<String>,
  pub roll: Option<Roll>,
  pub term: Option<Term>,
}

impl Snippet {
  pub fn get_keyword_ids(&self) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    if let Some(term) = &self.term {
      if let Some(keyword_id) = term.keyword_id {
        ids.insert(keyword_id);
      }
    }
    return ids;
  }
}

#[component]
pub fn RulesSpippetDetail(snippets: RulesSnippet, display: TermDisplay) -> Element {
  rsx!(
    div {
      class: "inline",
      for snippet in snippets {
        SnippetDetail { snippet, display }
      }
    }
  )
}

#[component]
pub fn SnippetDetail(snippet: Snippet, display: TermDisplay) -> Element {
  let id = match snippet.term {
    Some(ref term) => match term.keyword_id {
      Some(id) => Some(id.to_string()),
      _ => None,
    },
    None => None,
  };
  rsx!(
    if let Some( text ) = snippet.text {
      TextSnippet { text }
    }
    if let Some( term ) = snippet.term {
      TermSnippet { term, id, display }
    }
    if let Some( roll ) = snippet.roll {
      RollSnippet { roll }
    }
  )
}

#[component]
pub fn TextSnippet(text: String) -> Element {
  return rsx!( span { "{text} " } );
}
