use std::fmt::{Display, Formatter, Result};

use crate::rules::components::Modifier;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::components::AttributeRow;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StandardExpertise {
  Arcana,
  Engineering,
  History,
  Medicine,
  Nature,
  Politics,
  Religion,
  Survival,
}

impl Display for StandardExpertise {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self)
  }
}

impl StandardExpertise {
  pub fn iter<'a>() -> impl Iterator<Item = &'a Self> {
    return [
      Self::Arcana,
      Self::Engineering,
      Self::History,
      Self::Medicine,
      Self::Nature,
      Self::Politics,
      Self::Religion,
      Self::Survival,
    ].iter();
  }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertiseEntry {
  pub title: Option<String>,
  pub ranks: Option<i32>,
  pub empty: Option<bool>,
}

#[component]
pub fn ExpertiseComponent(entry: ExpertiseEntry) -> Element {
  return match (entry.title, entry.ranks, entry.empty) {
    (_, _, Some(true)) => rsx!(div {
      class: "underline-border full lh"
    }),
    (Some(title), Some(rank), _) => rsx! {
      AttributeRow {
        name: title,
        name_class: "highlight",
        Modifier { value: rank }
      }
    },
    _ => rsx!("Undefined"),
  };
}
