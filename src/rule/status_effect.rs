use serde::{Deserialize, Serialize};
use dioxus::prelude::*;

use crate::rule::snippet::SnippetSetDetails;

use super::snippet::Snippet;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct StatusEffect {
  pub title: Option<String>,
  pub keywords: Option<Vec<String>>,
  pub rules: Option<Vec<Vec<Snippet>>>,
}

#[component]
pub fn StatusEffectSnippet( effect: StatusEffect ) -> Element {
  rsx!(
    div {
      class: "card-small",
      if let Some( title ) = effect.title {
        div { "{title}" }
      }
      if let Some( sets ) = effect.rules {
        ul {
          for rules in sets {
            li {
              SnippetSetDetails { rules }
            }
          }
        }
      }
    }
  )
}