use std::collections::HashSet;

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use dioxus::prelude::*;

use crate::rule::snippet::SnippetSetDetails;

use super::snippet::Snippet;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct StatusEffect {
  pub title: Option<String>,
  pub rules: Option<Vec<Snippet>>,
}

impl StatusEffect {
  pub fn get_keyword_ids( &self ) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    if let Some( rules ) = &self.rules {
      for rule in rules {
        ids.extend( rule.get_keyword_ids() );
      }
    }
    return ids;
  }
}

#[component]
pub fn StatusEffectSnippet( effect: StatusEffect ) -> Element {
  println!( "Status Effect" );
  rsx!(
    div {
      class: "card-small",
      if let Some( title ) = effect.title {
        div { "{title}" }
      }
      if let Some( rules ) = effect.rules {
        SnippetSetDetails { rules }
      }
    }
  )
}
