use serde::{ Deserialize, Serialize };
use dioxus::prelude::*;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct RuleTerm {
  pub title: String,
  pub blurb: String,
  pub keyword_id: Option<String>,
}

#[component]
pub fn TermSnippet( term: RuleTerm ) -> Element {
  rsx!(
    span {
      class: "term",
      div {
        class: "float-anchor",
        div {
          class: "floating-panel",
          "{term.blurb}"
        }
      }
      span {
        class: "bold",
        "{term.title}"
      }
    }
  )
}
