use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{character::prelude::DamageClass, rules::prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
  pub title: String,
  pub class: WeaponClass,
  pub damage_dice: DiceGroup,
  pub damage_class: DamageClass,
  pub range: Option<i32>,
  pub block: Option<i32>,
  pub keywords: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum WeaponClass {
  #[default]
  Melee,
  Thrown,
  Ranged,
}

impl fmt::Display for WeaponClass {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        WeaponClass::Melee => "Melee",
        WeaponClass::Thrown => "Thrown",
        WeaponClass::Ranged => "Ranged",
      }
    )
  }
}

#[component]
pub fn WeaponEntry(weapon: ReadSignal<Weapon>) -> Element {
  let weapon_signal = weapon.read().clone();
  let title = weapon_signal.title;
  let class = weapon_signal.class;
  let opt_keywords = weapon_signal.keywords;
  let damage_dice = weapon_signal.damage_dice;
  let damage_class = weapon_signal.damage_class;
  let opt_range = weapon_signal.range;
  let opt_block = weapon_signal.block;
  rsx!(
    div {
      class: "column no-gap",
      div {
        class: "row",
        span { class: "highlight", "{title}" }
        span { " {class}" }
      }
      div {
        DiceGroupEntry { group: damage_dice }
        span { " {damage_class}" }
      }
      if let Some( range ) = opt_range {
        div { "Range {range}" }
      }
      if let Some( block ) = opt_block {
        div { "Block {block} Physical" }
      }
      if let Some( keywords ) = opt_keywords {
        div { class: "italics", "{keywords}" }
      }
    }
  )
}
