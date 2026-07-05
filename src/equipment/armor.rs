use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Armor {
  pub title: String,
  pub physical_resistance: i32,
  pub fortitude_requirement: i32,
  pub bulk: Option<i32>,
  pub drag: Option<i32>,
}

#[component]
pub fn ArmorEntry(armor: Armor) -> Element {
  let title = armor.title;
  let physical_resistance = armor.physical_resistance;
  let fortitude_requirement = armor.fortitude_requirement;
  rsx! {
    div {
      class: "card-snug column",
      div { class: "underline highlight", "{title}" }
      div { "Armor {physical_resistance}" }
      if fortitude_requirement > 0 {
        div { "Required Fortitude {fortitude_requirement}" }
      }
      if let Some( bulk ) = armor.bulk {
        div { "Bulk {bulk}" }
      }
      if let Some( drag ) = armor.drag {
        div { "Drag {drag}" }
      }
    }
  }
}
