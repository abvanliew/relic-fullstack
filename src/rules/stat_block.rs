use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::prelude::*;
use crate::rules::components::Modifier;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StatBlock {
  pub attributes: Option<AttributeOptionRanks>,
  pub resistances: Option<Resistances>,
  pub ap: Option<i32>,
  pub hp: Option<i32>,
  pub hp_pool: Option<i32>,
  pub con: Option<i32>,
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
        if let Some(warfare) = attributes.warfare {
          AttributeRow {
            name: "Warfare",
            element: rsx!( Modifier { value: warfare } ),
          }
        }
        if let Some(spirit) = attributes.spirit {
          AttributeRow {
            name: "Spirit",
            element: rsx!( Modifier { value: spirit } ),
          }
        }
        if let Some(manipulation) = attributes.manipulation {
          AttributeRow {
            name: "Manipulation",
            element: rsx!( Modifier { value: manipulation } ),
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
        if let Some(resolve) = attributes.resolve {
          AttributeRow {
            name: "Resolve",
            element: rsx!( "{resolve + BASE_DEFENSE}" ),
          }
        }
        if let Some(insight) = attributes.insight {
          AttributeRow {
            name: "Insight",
            element: rsx!( "{insight + BASE_DEFENSE}" ),
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
        if let Some(ap) = stats.ap {
          AttributeRow {
            name: "Action Point",
            element: rsx!( "{ap}" ),
          }
        }
        if let Some(hp) = stats.hp {
          AttributeRow {
            name: "HP",
            element: rsx!( "{hp}" ),
          }
        }
        if let Some(hp_pool) = stats.hp_pool {
          AttributeRow {
            name: "HP Pool",
            element: rsx!( "{hp_pool}" ),
          }
        }
        if let Some(con) = stats.con {
          AttributeRow {
            name: "Constitution",
            element: rsx!( "{con}" ),
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
  };
}
