use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum KeywordClass {
  Classifier,
  Term,
  Condition,
  Attribute,
  CoreRule,
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
