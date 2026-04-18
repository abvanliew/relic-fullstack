use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Armor {
  pub title: String,
  pub physical_resistance: i32,
  pub tenacity_requirement: i32,
  pub speed_penalty: Option<i32>,
}


#[component]
pub fn ArmorEntry(
  armor: Armor,
) -> Element {
  let title = armor.title;
  let physical_resistance = armor.physical_resistance;
  let tenacity_requirement = if armor.tenacity_requirement <= 0 {None} else {Some(armor.tenacity_requirement)};
  let speed_penalty = armor.speed_penalty;
  rsx! {
    div {
      class: "column",
      div {
        class: "row",
        span { class: "underline highlight", "{title}" }
        span { class: "slightlight", " Armor" }
        " {physical_resistance}, " 
        if let Some( tenacity_requirement ) = tenacity_requirement {
          span { class: "italics", " Bulk {tenacity_requirement}" }
        }
      }
      div {
        class: "row",
        if let Some( speed_penalty ) = speed_penalty {
          span { class: "italics", " Speed Penalty {speed_penalty}" }
        }
      }
    }
  }
}
