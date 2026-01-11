mod aspect;
mod classifiers;
mod filter;
mod tense;
mod term;

use std::cmp::Ordering;

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::rules::prelude::*;
use internal::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub title: String,
  pub class: KeywordClass,
  pub blurb: Option<String>,
  pub rules: Option<Vec<Snippet>>,
  pub tenses: Option<Tenses>,
}

impl Default for Keyword {
  fn default() -> Self {
    Self {
      id: Default::default(),
      title: "undefined".into(),
      class: Default::default(),
      blurb: Default::default(),
      rules: Default::default(),
      tenses: Default::default(),
    }
  }
}

impl PartialOrd for Keyword {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self.class.partial_cmp(&other.class) {
      Some(core::cmp::Ordering::Equal) => {}
      ord => return ord,
    }
    self.title.partial_cmp(&other.title)
  }
}

impl Ord for Keyword {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.class.cmp( &other.class ) {
      Ordering::Equal => {},
      ord => return ord,
    }
    self.title.cmp( &other.title )
  }
}

pub mod prelude {
  pub(crate) use super::classifiers::KeywordClassified;
  pub(crate) use super::filter::rules_specific;
  pub use super::term::Term;
  pub(crate) use super::term::{
    KeywordBlocks, KeywordCard, KeywordCards, TermSnippet,
  };
  pub use super::Keyword;
}

mod internal {
  pub(super) use super::classifiers::KeywordClass;
  pub(super) use super::tense::{Tense, Tenses};
  pub(super) use super::Keyword;
}
