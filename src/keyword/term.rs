use super::internal::*;
use crate::rules::prelude::*;
use crate::server::prelude::*;
use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Term {
  pub keyword_id: Option<ObjectId>,
  pub title: Option<String>,
  pub tense: Option<Tense>,
  pub italics: Option<bool>,
}

impl Term {
  pub fn to_title(&self) -> String {
    return match (self.keyword_id, &self.title) {
      (Some(keyword_id), _) => {
        let KeywordCache( ref keyword_cache ) = use_context();
        let keyword_result = keyword_cache.from_object_id( &keyword_id );
        match keyword_result {
          Some(keyword) => keyword.title_as(&self.tense),
          None => "undefined".into(),
        }
      }
      (_, Some(title)) => title.clone(),
      _ => "undefined".into(),
    };
  }
}

#[component]
pub(crate) fn TermSnippet(term: Term) -> Element {
  let title = term.to_title();
  let conditional_class = match term.italics.unwrap_or_default() {
    true => "italics",
    false => "highlight",
  };
  return rsx! { span { class: "{conditional_class}", " {title}" } };
}

#[component]
pub(crate) fn KeywordCards(
  keywords: Vec<Keyword>,
) -> Element {
  return rsx! {
    for keyword in keywords {
      KeywordCard { keyword }
    }
  };
}

#[component]
pub(crate) fn KeywordCard(keyword: Keyword) -> Element {
  let title = keyword.title.clone();
  let blocks = keyword.blocks();
  let class = keyword.class_title();
  return rsx! {
    div {
      class: "card thin-border grid dim-keywords",
      div { class: "uv-title-property highlight", "{title}" }
      div { class: "uv-property italics", "{class}" }
      div { class: "uv-full indent",
        RulesBlockSet { blocks }
      }
    }
  };
}

#[component]
pub(crate) fn KeywordBlocks(
  keywords: Vec<Keyword>,
) -> Element {
  return rsx! {
    for keyword in keywords {
      KeywordBlock { keyword }
    }
  };
}

#[component]
pub(crate) fn KeywordBlock(keyword: Keyword) -> Element {
  let title = keyword.title.clone();
  let blocks = keyword.blocks();
  return rsx! {
    PropertyDetail { 
      title,
      block: true,
      RulesBlockSet { blocks }
    }
  };
}
