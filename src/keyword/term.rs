use std::collections::HashSet;

use super::internal::*;
use crate::keyword::filter::rules_specific;
use crate::rules::prelude::*;
use crate::server::prelude::KeywordCache;
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

  pub fn to_title(&self) -> String {
    return match (self.keyword_id, &self.title) {
      (Some(keyword_id), _) => {
        let KeywordCache(keyword_cache) = use_context::<KeywordCache>();
        let keyword_result = keyword_cache.from_object_id(&keyword_id);
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
  return rsx! { span { class: "highlight", " {title}" } };
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum KeywordDisplay {
  #[default]
  Card,
  Block,
}

#[component]
pub(crate) fn KeywordSnippetsLoader(
  keyword_id_objects: HashSet<ObjectId>,
  #[props(default)] display: KeywordDisplay,
  #[props(default)] all_keywords: bool,
) -> Element {
  let KeywordCache(keyword_cache) = use_context::<KeywordCache>();
  let keywords_vec = Vec::from_iter(keyword_id_objects);
  let keywords = match all_keywords {
    true => keyword_cache.from_object_ids(&keywords_vec),
    false => rules_specific(keyword_cache.from_object_ids(&keywords_vec)),
  };
  return rsx! {
    KeywordSnippets { keywords, display }
  };
}

#[component]
pub(crate) fn KeywordSnippets(
  keywords: Vec<Keyword>,
  #[props(default)] display: KeywordDisplay,
) -> Element {
  return rsx! {
    for keyword in keywords {
      KeywordSnippet { keyword, display: display.clone() }
    }
  };
}

#[component]
pub(crate) fn KeywordSnippet(
  keyword: Keyword,
  #[props(default)] display: KeywordDisplay,
) -> Element {
  return match &display {
    KeywordDisplay::Card => rsx! { KeywordCard { keyword } },
    KeywordDisplay::Block => rsx! { KeywordBlock { keyword } },
  };
}

#[component]
pub(crate) fn KeywordCard(keyword: Keyword) -> Element {
  let title = keyword.title.clone();
  let blocks = keyword.blocks();
  let class = keyword.class_title();
  return rsx! {
    div {
      class: "card-thin grid dim-keywords",
      div { class: "uv-title-property highlight", "{title}" }
      div { class: "uv-property italics", "{class}" }
      div { class: "uv-full indent",
        RulesBlockSet { blocks }
      }
    }
  };
}

#[component]
pub(crate) fn KeywordBlock(keyword: Keyword) -> Element {
  let title = keyword.title.clone();
  let blocks = keyword.blocks();
  return rsx! {
    div { class: "uv-full highlight", "{title}" }
    div {
      class: "uv-full indent",
      RulesBlockSet { blocks }
    }
  };
}
