use serde::{Deserialize, Serialize};
use super::internal::*;

impl Keyword {
  pub fn title_as(&self, tense: &Option<Tense>) -> String {
    return match ( &self.tenses, tense) {
      (Some(tenses),Some(tense)) => match tenses.get(tense) {
        None => self.title.clone(),
        Some(title) => title,
      },
      _ => self.title.clone(),
    };
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Eq)]
pub enum Tense {
  Past,
  #[default]
  Present,
  Future,
  Singular,
  Plural,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Tenses {
  pub past: Option<String>,
  pub present: Option<String>,
  pub future: Option<String>,
  pub singular: Option<String>,
  pub plural: Option<String>,
}

impl Tenses {
  pub fn get(&self, tense: &Tense) -> Option<String> {
    match tense {
      Tense::Past => self.past.clone(),
      Tense::Present => self.present.clone(),
      Tense::Future => self.future.clone(),
      Tense::Singular => self.singular.clone(),
      Tense::Plural => self.plural.clone(),
    }
  }
}
