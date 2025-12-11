mod classifiers;
mod filter;
mod tense;
mod term;

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::rules::prelude::*;
use internal::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub title: String,
  pub class: Option<KeywordClass>,
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

pub mod prelude {
  pub(crate) use super::classifiers::{KeywordClass, KeywordClassified};
  pub(crate) use super::term::{TermSnippet, KeywordCardList};
  pub use super::term::{Term, TermDisplay};
  pub use super::Keyword;
  pub use super::filter::rules_specific;
}

mod internal {
  pub(super) use super::Keyword;
  pub(super) use super::classifiers::KeywordClass;
  pub(super) use super::tense::{Tense, Tenses};
}
