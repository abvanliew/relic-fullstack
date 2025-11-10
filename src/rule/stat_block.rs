use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::prelude::*;
use crate::rule::components::Modifier;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StatBlock {
  pub attributes: Option<AttributeOptionRanks>,
  pub resistances: Option<Resistances>,
  pub hp_bonus: Option<i32>,
  pub speed: Option<i32>,
  pub dash: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, Eq)]
pub struct AttributeOptionRanks {
  pub physique: Option<i32>,
  pub warfare: Option<i32>,
  pub spirit: Option<i32>,
  pub manipulation: Option<i32>,
  pub tenacity: Option<i32>,
  pub fortitude: Option<i32>,
  pub resolve: Option<i32>,
  pub insight: Option<i32>,
  pub dodge: Option<i32>,
}

#[component]
pub fn StatBlockSnippet(stats: StatBlock) -> Element {
  let attributes = stats.attributes.unwrap_or_default();
  let resistances_opt = stats.resistances;
  return rsx! {
    div {
      class: "uv-full grid dim-forths gap-2xlarge",
      div {
        class: "column",
        div { class: "subheading", "Capabilites" }
        if let Some(physique) = attributes.physique {
          AttributeRow {
            name: "Physique", 
            element: rsx!( Modifier { value: physique } ),
          }
        }
      }
      div {
        class: "column",
        div { class: "subheading", "Defenses" }
        if let Some(tenacity) = attributes.tenacity {
          AttributeRow {
            name: "Tenacity", 
            element: rsx!( "{tenacity + BASE_DEFENSE}" ),
          }
        }
        if let Some(fortitude) = attributes.fortitude {
          AttributeRow {
            name: "Fortitude", 
            element: rsx!( "{fortitude + BASE_DEFENSE}" ),
          }
        }
        if let Some(dodge) = attributes.dodge {
          AttributeRow {
            name: "Dodge", 
            element: rsx!( "{dodge + BASE_DEFENSE}" ),
          }
        }
      }
      div {
        class: "column",
        div { class: "subheading", "Resistances" }
        if let Some( resistances ) = resistances_opt {
          ResistanceDetails { resistances }
        }
      }
      div {
        class: "column",
        div { class: "subheading", "Body" }
        if let Some(hp_bonus) = stats.hp_bonus {
          AttributeRow {
            name: "HP Bonus", 
            element: rsx!( Modifier { value: hp_bonus } ),
          }
        }
        if let Some(speed) = stats.speed {
          AttributeRow {
            name: "Speed", 
            element: rsx!( "{speed}" ),
          }
        }
        if let Some(dash) = stats.dash {
          AttributeRow {
            name: "Dash", 
            element: rsx!( "{dash}" ),
          }
        }
      }
    }
  }
}
