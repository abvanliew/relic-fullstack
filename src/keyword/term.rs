use super::internal::*;
use crate::rules::prelude::*;
use crate::server::prelude::ServerRequestSignals;
use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Term {
  pub keyword_id: Option<ObjectId>,
  pub title: Option<String>,
  pub tense: Option<Tense>,
}

impl Term {
  pub fn title(title: String) -> Self {
    Self {
      title: Some(title),
      ..Default::default()
    }
  }

  pub fn keyword(keyword_id: ObjectId) -> Self {
    Self {
      keyword_id: Some(keyword_id),
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub enum TermDisplay {
  #[default]
  Title,
  Block,
  Card,
}

#[derive(PartialEq, Props, Clone)]
pub(crate) struct TermSnippetProps {
  term: Term,
  #[props(default)]
  display: TermDisplay,
}

#[component]
pub(crate) fn TermSnippet(props: TermSnippetProps) -> Element {
  let keyword_opt = match &props.term.keyword_id {
    Some(id) => {
      let signal = use_context::<ServerRequestSignals>();
      let keywords_response = signal.get_keywords();
      let Some(keyword_map) = keywords_response else {
        return rsx!( div { "Loading" } );
      };
      keyword_map.get(&id.to_string()).cloned()
    }
    _ => None,
  };
  let title = match (&keyword_opt, &props.term.title) {
    (Some(keyword), _) => keyword.title_as(&props.term.tense),
    (_, Some(title)) => title.clone(),
    _ => "undefined".into(),
  };
  let blocks_opt = match &keyword_opt {
    Some(keyword) => match (&keyword.rules, &keyword.blurb) {
      (Some(block), _) => Some(snippets_to_rules_blocks(block.clone())),
      (_, Some(blurb)) => Some(blurb_to_rules_blocks(blurb.clone())),
      _ => None,
    },
    _ => None,
  };
  return match &props.display {
    TermDisplay::Title => rsx! { span { class: "highlight", " {title}" } },
    TermDisplay::Block => {
      rsx! {
        div { class: "uv-full highlight", "{title}" }
        if let Some( blocks ) = blocks_opt {
          div { class: "uv-full indent",
            RulesBlockSet { blocks }
          }
        }
      }
    }
    TermDisplay::Card => {
      let class = match &keyword_opt {
        Some(keyword) => match &keyword.class {
          Some(class) => class.to_string(),
          None => "".into(),
        },
        None => "".into(),
      };
      rsx! {
        div {
          class: "card-thin grid dim-keywords",
          div { class: "uv-title-property highlight", "{title}" }
          div { class: "uv-property italics", "{class}" }
          if let Some( blocks ) = blocks_opt {
            div { class: "uv-full indent",
              RulesBlockSet { blocks }
            }
          }
        }
      }
    }
  };
}
