use crate::server::prelude::ServerRequestSignals;
use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Term {
  pub keyword_id: Option<ObjectId>,
  pub title: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub enum TermDisplay {
  #[default]
  TitleOnly,
  Block,
  Hover,
  Row,
}

#[derive(PartialEq, Props, Clone)]
pub struct TermSnippetProps {
  #[props(default)]
  id: Option<String>,
  #[props(default)]
  term: Option<Term>,
  #[props(default)]
  display: TermDisplay,
}

#[component]
pub fn TermSnippet(props: TermSnippetProps) -> Element {
  let opt_keyword = match props.id {
    Some(id) => {
      let signal = use_context::<ServerRequestSignals>();
      let keywords_response = signal.get_keywords();
      let Some(keyword_map) = keywords_response else {
        return rsx!( div { "Loading" } );
      };
      keyword_map.get(&id).cloned()
    }
    _ => None,
  };
  let opt_title = match props.term {
    Some(term) => term.title,
    None => None,
  };
  let (title, blurb_text) = match (opt_keyword, opt_title) {
    (Some(keyword), _) => (
      keyword.title.clone(),
      keyword.blurb.clone().unwrap_or("".into()),
    ),
    (_, Some(title)) => (title, "".into()),
    _ => ("Undefined".into(), "".into()),
  };
  match &props.display {
    TermDisplay::Hover => rsx! {
      div {
        class: "term",
        div {
          class: "float-anchor",
          div { class: "floating-panel term-panel", "{blurb_text}" }
        }
        span { class: "highlight", " {title}" }
      }
    },
    TermDisplay::Block => rsx! {
      div {
        class: "term-panel column",
        div { class: "highlight", "{title}" }
        div { "{blurb_text}" }
      }
    },
    TermDisplay::Row => rsx! {
      div { class: "uv-title highlight", "{title}" }
      div { class: "uv-details", "{blurb_text}" }
    },
    _ => rsx! { span { class: "highlight", " {title}" } },
  }
}
