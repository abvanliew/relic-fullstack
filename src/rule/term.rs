use serde::{ Deserialize, Serialize };
use dioxus::prelude::*;
use bson::oid::ObjectId;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
#[serde(rename_all = "camelCase")]
pub struct RuleTerm {
  pub title: String,
  pub blurb: String,
  pub keyword_id: Option<ObjectId>,
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
