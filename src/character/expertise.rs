use crate::rules::components::Modifier;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::components::AttributeRow;

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
      class: "solid-underline full lh"
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
