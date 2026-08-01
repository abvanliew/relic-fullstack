use crate::server::prelude::KeywordCache;

use super::Keyword;
use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Default)]
pub enum KeywordClass {
  CoreRule,
  Attribute,
  Term,
  Condition,
  #[default]
  Classifier,
}

impl fmt::Display for KeywordClass {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        KeywordClass::Classifier => "Classifier",
        KeywordClass::Term => "Term",
        KeywordClass::Condition => "Condition",
        KeywordClass::Attribute => "Attribute",
        KeywordClass::CoreRule => "Core Rule",
      }
    )
  }
}

pub trait KeywordClassified {
  fn get_keyword_ids(&self) -> HashSet<ObjectId>;
}

pub fn terms_and_conditions(keywords: Vec<Keyword>) -> Vec<Keyword> {
  let mut filtered_keywords: Vec<Keyword> = keywords
    .into_iter()
    .filter(|keyword| match keyword.class {
      KeywordClass::Term | KeywordClass::Condition => true,
      _ => false,
    })
    .collect();
  filtered_keywords.sort();
  return filtered_keywords;
}

pub fn partitioned_terms_and_conditions(keywords: &Vec<Keyword>) -> (Vec<Keyword>, Vec<Keyword>) {
  let mut terms = Vec::new();
  let mut conditions = Vec::new();
  for keyword in keywords {
    match &keyword.class {
      KeywordClass::Term => terms.push(keyword.clone()),
      KeywordClass::Condition => conditions.push(keyword.clone()),
      _ => (),
    }
  }
  terms.sort();
  conditions.sort();
  return (terms, conditions);
}

pub fn display_keywords(keyword_ids: &Vec<ObjectId>) -> Option<String> {
  let KeywordCache(ref keyword_cache) = use_context();
  let keywords = keyword_cache
    .from_object_ids(&keyword_ids)
    .iter()
    .map(|keyword| keyword.title.clone())
    .collect::<Vec<String>>();
  return if keywords.len() > 0 {
    Some(keywords.join(", "))
  } else {
    None
  };
}
