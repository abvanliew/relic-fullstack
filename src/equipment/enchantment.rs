use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::HorizontalBar;
use crate::modifiers::prelude::*;
use crate::rules::prelude::*;
use crate::skill::component::ActionDetails;
use crate::skill::prelude::Action;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Enchantment {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub title: String,
  pub description: Option<String>,
  pub enchant_class: Option<EnchantClass>,
  pub enchants: Option<String>,
  pub rank: Option<i32>,
  pub rank_min: Option<i32>,
  pub base_cost: Option<f32>,
  pub ranked_cost: Option<f32>,
  #[serde(default)]
  pub attunement: HashSet<AttunementRune>,

  pub modifiers: Option<ModifierSet>,
  #[serde(default)]
  pub actions: Vec<Action>,
}

impl Default for Enchantment {
  fn default() -> Self {
    Self {
      id: ObjectId::new(),
      title: "Undefined".into(),
      description: None,
      enchant_class: None,
      enchants: None,
      actions: Default::default(),
      modifiers: None,
      attunement: Default::default(),
      base_cost: None,
      ranked_cost: None,
      rank: None,
      rank_min: None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
pub enum AttunementRune {
  // Survival
  Hune,
  // Passion
  Kal,
  // Wisdom
  Bok,
  // Heart
  Pir,
  // Voice
  Kin,
  // Sight
  Alu,
  // Astral
  Lat,
}

impl Display for AttunementRune {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub fn attunement_rune_order() -> Vec<AttunementRune> {
  vec![
    AttunementRune::Hune, 
    AttunementRune::Kal, 
    AttunementRune::Bok, 
    AttunementRune::Pir, 
    AttunementRune::Kin, 
    AttunementRune::Alu, 
    AttunementRune::Lat, 
  ]
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EnchantClass {
  Weapon,
  Armor,
  Accessory,
  Conumable,
}

impl Display for EnchantClass {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self)
  }
}

#[component]
pub fn EnchantmentDetails(enchantment: Enchantment) -> Element {
  let title = enchantment.title;
  let enchants = enchantment.enchants.unwrap_or("Enchantment".into());
  let description = enchantment.description;
  let actions = enchantment.actions;
  let rank_displays = vec![
    enchantment.rank.map(|value|format!("+{value}")),
    enchantment.rank_min.map(|value|format!("Minimum {value}")),
  ].into_iter().flatten().collect::<Vec<String>>();
  let rank_text_result = if rank_displays.len()>0 {Some(rank_displays.join(", "))} else {None};
  let base_cost = enchantment.base_cost;
  let ranked_cost = enchantment.ranked_cost;
  let attunement_runes: Option<String> = if enchantment.attunement.len() > 0 {
    let rune_text = attunement_rune_order()
    .iter()
    .filter_map(|rune| enchantment.attunement.get(rune))
    .map(|rune|rune.to_string())
    .collect::<Vec<String>>();
    Some( rune_text.join(", ") )
  } else {
    None
  };
  rsx! {
    div {
      class: "card grid dim-keywords",
      div { class: "uv-title-property title nowrap gap", "{title}" }
      div { class: "uv-property",
        div { class: "nowrap italics", "{enchants} Enchantment" }
      }
      if let Some( rank_text ) = rank_text_result {
        PropertyDetail {
          title: "Rank",
          "{rank_text}"
        }
      }
      if let Some( cost ) = base_cost {
        PropertyDetail {
          title: "Base Cost",
          "{cost}"
        }
      }
      if let Some( cost ) = ranked_cost {
        PropertyDetail {
          title: "Ranked Cost",
          "{cost}"
        }
      }
      if let Some( runes ) = attunement_runes {
        PropertyDetail {
          title: "Attune",
          "{runes}"
        }
      }
      if let Some( description ) = description {
        div {
          class: "uv-full",
          "{description}"
        }
      }
      if actions.len() > 0 {
        HorizontalBar {}
        for action in actions {
          ActionDetails { action }
        }
      }
    }
  }
}
