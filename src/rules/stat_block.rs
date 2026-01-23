use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::prelude::*;
use crate::progression::prelude::*;
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
      class: "uv-full grid dim-forths bumper gap-2xlarge",
      div {
        class: "column",
        div { class: "highlight underline", "Capabilites" }
        if let Some(physique) = attributes.physique {
          AttributeRow {
            name: "Physique",
            name_class: Some( "".into() ),
            Modifier { value: physique }
          }
        }
        if let Some(warfare) = attributes.warfare {
          AttributeRow {
            name: "Warfare",
            name_class: Some( "".into() ),
            Modifier { value: warfare }
          }
        }
        if let Some(spirit) = attributes.spirit {
          AttributeRow {
            name: "Spirit",
            name_class: Some("".into()),
            Modifier { value: spirit }
          }
        }
        if let Some(manipulation) = attributes.manipulation {
          AttributeRow {
            name: "Manipulation",
            name_class: Some("".into()),
            Modifier { value: manipulation }
          }
        }
      }
      div {
        class: "column",
        div { class: "highlight underline", "Defenses" }
        if let Some(tenacity) = attributes.tenacity {
          AttributeRow {
            name: "Tenacity",
            name_class: Some("".into()),
            "{tenacity + BASE_DEFENSE}"
          }
        }
        if let Some(fortitude) = attributes.fortitude {
          AttributeRow {
            name: "Fortitude",
            name_class: Some("".into()),
            "{fortitude + BASE_DEFENSE}"
          }
        }
        if let Some(resolve) = attributes.resolve {
          AttributeRow {
            name: "Resolve",
            name_class: Some("".into()),
            "{resolve + BASE_DEFENSE}"
          }
        }
        if let Some(insight) = attributes.insight {
          AttributeRow {
            name: "Insight",
            name_class: Some("".into()),
            "{insight + BASE_DEFENSE}"
          }
        }
        if let Some(dodge) = attributes.dodge {
          AttributeRow {
            name: "Dodge",
            name_class: "",
            "{dodge + BASE_DEFENSE}"
          }
        }
      }
      div {
        class: "column",
        div { class: "highlight underline", "Resistances" }
        if let Some( resistances ) = resistances_opt {
          ResistanceDetails { resistances }
        }
      }
      div {
        class: "column",
        div { class: "highlight underline", "Body" }
        if let Some(ap) = stats.ap {
          AttributeRow {
            name: "Action Point",
            name_class: "",
            "{ap}"
          }
        }
        if let Some(hp) = stats.hp {
          AttributeRow {
            name: "HP",
            name_class: "",
            "{hp}"
          }
        }
        if let Some(hp_pool) = stats.hp_pool {
          AttributeRow {
            name: "HP Pool",
            name_class: "",
            "{hp_pool}"
          }
        }
        if let Some(con) = stats.con {
          AttributeRow {
            name: "Constitution",
            name_class: "",
            "{con}"
          }
        }
        if let Some(speed) = stats.speed {
          AttributeRow {
            name: "Speed",
            name_class: "",
            "{speed}"
          }
        }
        if let Some(dash) = stats.dash {
          AttributeRow {
            name: "Dash",
            name_class: "",
            "{dash}"
          }
        }
      }
    }
  };
}
