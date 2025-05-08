use serde::{ Deserialize, Serialize };
use dioxus::prelude::*;
use bson::oid::ObjectId;
use crate::server::prelude::GameLibrarySignal;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
#[serde(rename_all = "camelCase")]
pub struct RuleTerm {
  pub keyword_id: Option<ObjectId>,
  pub title: Option<String>,
}

#[component]
pub fn TermSnippet( term: RuleTerm, hover: bool ) -> Element {
  let signal = use_context::<GameLibrarySignal>();
  let keywords_response = signal.get_keyword();
  let Ok( keyword_map ) = keywords_response else { return rsx!( div { "Loading" } ) };
  let ( title, blurb ) = match ( term.keyword_id, term.title ) {
    ( Some( keyword_id ), _ ) => {
      let entry = keyword_map.get( &keyword_id.to_string() );
      match entry {
        Some( keyword ) => ( keyword.title.clone(), keyword.blurb.clone() ),
        _ => ( "Undefined".into(), None ),
      }
    },
    ( _, Some( title ) ) => ( title, None ),
    _ => ( "Undefined".into(), None ),
  };
  match hover {
    true => rsx!(
      span {
        class: "term",
        if let Some( blurb_text ) = blurb {
          div {
            class: "float-anchor",
            div {
              class: "floating-panel term-panel",
              "{blurb_text}"
            }
          }
        }
        span {
          class: "bold",
          "{title}"
        }
      }
    ),
    false => rsx!(
      if let Some( blurb_text ) = blurb {
        div {
          class: "term-panel column",
          div { class: "highlight", "{title}" }
          div { "{blurb_text}" }
        }
      }
    )
  }
  
}
