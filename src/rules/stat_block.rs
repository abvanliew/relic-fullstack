use std::fmt::Display;
use std::cmp::Ordering;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::prelude::*;
use crate::progression::prelude::*;
use crate::rules::components::Modifier;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StatBlock {
  pub attributes: Option<AttributeRanks>,
  pub resistances: Option<Resistances>,
  pub ap: Option<i32>,
  pub hp: Option<i32>,
  pub hp_pool: Option<i32>,
  pub con: Option<i32>,
  pub speed: Option<i32>,
  pub dash: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, Eq)]
pub struct AttributeRanks {
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

fn unpack_to_tuple<T: Display>( value: &Option<i32>, id: T ) -> Option<(String, i32)> {
  match value {
    Some( rank ) => Some((id.to_string(), *rank)),
    None => None,
  }
}

impl AttributeRanks {
  pub fn update_dodge_with_bulk(&mut self, bulk: i32, slow: i32) -> (bool, i32) {
    let tenacity = self.tenacity.unwrap_or(0);
    let net_tenacity = tenacity - bulk;
    let equiped = net_tenacity >= 0;
    let (dodge, net_slow) = match net_tenacity.cmp(&slow) {
      Ordering::Less => (0, slow + net_tenacity),
      Ordering::Equal => (0, 0),
      Ordering::Greater => (net_tenacity - slow, 0),
    };
    self.dodge = Some(dodge);
    return (equiped, net_slow);
  }

  pub fn list_capabilities( &self ) -> Vec<(String,i32)> {
    let capabilities: Vec<Option<(String,i32)>>  = vec![
      unpack_to_tuple( &self.physique, Capability::Physique ),
      unpack_to_tuple( &self.warfare, Capability::Warfare ),
      unpack_to_tuple( &self.spirit, Capability::Spirit ),
      unpack_to_tuple( &self.manipulation, Capability::Manipulation ),
    ];
    capabilities.into_iter().filter_map(|tuple|tuple).collect()
  }

  pub fn list_defenses( &self ) -> Vec<(String,i32)> {
    let capabilities: Vec<Option<(String,i32)>>  = vec![
      unpack_to_tuple( &self.tenacity, Defense::Tenacity ),
      unpack_to_tuple( &self.fortitude, Defense::Fortitude ),
      unpack_to_tuple( &self.resolve, Defense::Resolve ),
      unpack_to_tuple( &self.insight, Defense::Insight ),
      unpack_to_tuple( &self.dodge, Defense::Dodge ),
    ];
    capabilities.into_iter().filter_map(|tuple|tuple).collect()
  }
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
        CapabilityBlock { attributes: attributes.clone() }
      }
      div {
        class: "column",
        div { class: "highlight underline", "Defenses" }
        DefenseBlock { attributes: attributes.clone() }
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

#[component]
pub fn CapabilityBlock(
  attributes: AttributeRanks,
) -> Element {
  let capabilities = attributes.list_capabilities();
  return rsx! {
    for (capability, rank) in capabilities {
      AttributeRow {
        name: capability,
        name_class: Some( "".into() ),
        Modifier { value: rank }
      }
    }
  }
}

#[component]
pub fn DefenseBlock(
  attributes: AttributeRanks,
) -> Element {
  let defenses = attributes.list_defenses();
  return rsx! {
    for (defense, rank) in defenses {
      AttributeRow {
        name: defense,
        name_class: Some( "".into() ),
        "{rank + BASE_DEFENSE}"
      }
    }
  }
}
