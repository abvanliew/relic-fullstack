use serde::{ Deserialize, Serialize };
use dioxus::prelude::*;
use bson::oid::ObjectId;
use crate::server::prelude::GameLibrarySignal;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
#[serde(rename_all = "camelCase")]
pub struct Term {
  pub keyword_id: Option<ObjectId>,
  pub title: Option<String>,
}

#[derive( Debug, Clone, Copy, PartialEq )]
pub enum TermDisplay {
  TitleOnly,
  Block,
  Hover,
}

#[component]
pub fn TermSnippet( term: Term, display: TermDisplay ) -> Element {
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
  match ( &display, &blurb ) {
    ( TermDisplay::Hover, Some( blurb_text ) ) => rsx! {
      div {
        class: "term",
        div {
          class: "float-anchor",
          div { class: "floating-panel term-panel", "{blurb_text}" }
        }
        span { class: "highlight", "{title} " }
      }
    },
    ( TermDisplay::Block, _ ) => rsx! {
      div {
        class: "term-panel column",
        div { class: "highlight", "{title}" }
        if let Some( blurb_text ) = blurb {
          div { "{blurb_text}" }
        }
      }
    },
    _ => rsx! { span { class: "highlight", "{title} " } },
  }
}
