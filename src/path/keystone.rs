use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::prelude::*;

use super::Bonus;

// impl Path {
//   pub fn resource_pool_modifiers( &self ) -> Vec<PoolModifier> {
//     self.keystones.clone().unwrap_or_default().into_iter().filter_map( |k| k.resource_pool ).collect()
//   }
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Keystone {
  pub title: String,
  pub summary: Option<String>,
  pub path_feature: Option<bool>,
  pub path_half_feature: Option<bool>,
  pub resource_pool: Option<PoolModifier>,
  pub one_of: Option<Vec<Keystone>>,
  pub spell_slot: Option<Bonus<i32>>,
  pub cantrip_slot: Option<Bonus<i32>>,
  pub spell_known: Option<Bonus<i32>>,
  pub cantrip_known: Option<Bonus<i32>>,
}

#[component]
pub fn KeystoneDescription(keystone: Keystone) -> Element {
  rsx!(
    div {
      class: "uv-full",
      "{keystone.clone():?}"
    }
    div {
      class: "highlight uv-title",
      "{keystone.title}"
    }
    div {
      class: "uv-after-title",
      if let Some( summary ) = keystone.summary {
        span { "{summary} " }
      }
      match ( keystone.path_feature, keystone.path_half_feature ) {
        ( Some( true ), Some( true ) ) => rsx!( span { "You gain a full and half feature from this path." } ),
        ( Some( true ), _ ) => rsx!( span { "You gain a feature from this path." } ),
        ( _, Some( true ) ) => rsx!( span { "You gain a half feature from this path." } ),
        _ => rsx!()
      }
    }
  )
}
