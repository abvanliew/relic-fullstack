use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::prelude::DamageClass;
use crate::keyword::prelude::display_keywords;
use crate::rules::prelude::{DiceGroup, DiceGroupEntry};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "class")]
pub enum Equipment {
  Weapon(Weapon),
  Armor(Armor),
}

impl Equipment {
  pub fn is_special_material(&self) -> bool {
    match self {
      Equipment::Weapon(_) => false,
      Equipment::Armor(armor) => armor.special_material,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
  #[serde(rename = "_id")]
  id: ObjectId,
  title: String,

  damage_class: DamageClass,
  damage_dice: DiceGroup,

  weight_class: WeightClass,
  weapon_class: WeaponClass,
  #[serde(default)]
  keywords: Vec<ObjectId>,

  physique_req: Option<i32>,
  handed: Option<i32>,
  #[serde(default)]
  natural: bool,
  #[serde(default)]
  double: bool,
  #[serde(default)]
  reach: bool,
  block: Option<i32>,

  range: Option<i32>,
  range_max: Option<i32>,
  reload: Option<ReloadAction>,
}

impl Ord for Weapon {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.physique_req.cmp(&other.physique_req) {
      Ordering::Equal => {},
      ord => return ord,
    }
    match self.weapon_class.cmp(&other.weapon_class) {
      Ordering::Equal => {},
      ord => return ord,
    }
    match self.weight_class.cmp(&other.weight_class) {
      Ordering::Equal => {},
      ord => return ord,
    }
    match self.handed.cmp(&other.handed) {
      Ordering::Equal => {},
      ord => return ord,
    }
    self.title.cmp(&other.title)
  }
}

impl PartialOrd for Weapon {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeightClass {
  Light,
  Balanced,
  Heavy,
}

impl Display for WeightClass {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaponClass {
  Melee,
  Thrown,
  Ranged,
}

impl Display for WeaponClass {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ReloadAction {
  Free,
  Interaction,
  Action,
  ComplexAction,
}

impl Display for ReloadAction {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match &self {
      ReloadAction::Free => write!(f, "Free Action"),
      ReloadAction::Interaction => write!(f, "Interaction"),
      ReloadAction::Action => write!(f, "Action"),
      ReloadAction::ComplexAction => write!(f, "Complex Action"),
    }
  }
}

#[component]
pub fn EquipmentCard(equipment: Equipment) -> Element {
  return match equipment {
    Equipment::Weapon(weapon) => rsx! { WeaponEntry { weapon } },
    Equipment::Armor(armor) => rsx! { ArmorEntry { armor } },
  };
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum EquipmentDisplay {
  #[default]
  Card,
  Row,
}

#[component]
pub fn WeaponEntry(weapon: Weapon, #[props(default)] display: EquipmentDisplay) -> Element {
  let title = weapon.title;
  let weight_class = weapon.weight_class.to_string();
  let weapon_class = weapon.weapon_class.to_string();
  let handed = match weapon.handed {
    Some(1) => Some("One Handed".into()),
    Some(2) => Some("Two Handed".into()),
    _ => None,
  };
  let natural = if weapon.natural {
    Some("Natural".into())
  } else {
    None
  };
  let double = if weapon.double {
    Some("Double".into())
  } else {
    None
  };
  let reach = if weapon.reach {
    Some("Reach".into())
  } else {
    None
  };
  let dice_group = weapon.damage_dice;
  let damage_class = weapon.damage_class;
  let physique_req = weapon.physique_req;
  let physique_req_display = physique_req
    .clone()
    .map_or("-".into(), |req| req.to_string());
  let block = weapon.block.map(|block| format!("Block {block}"));
  let range = match (weapon.range, weapon.range_max) {
    (Some(range), Some(max)) => Some(format!("Range {range} ({max})")),
    (Some(range), _) => Some(format!("Range {range}")),
    _ => None,
  };
  let reload = weapon.reload.map(|reload| format!("Reload {reload}"));
  let range_reload = vec![range.clone(), reload.clone()]
    .into_iter()
    .flatten()
    .collect::<Vec<String>>();
  let range_reload_display = if range_reload.len() > 0 {
    Some(range_reload.join(", "))
  } else {
    None
  };
  let keywords = display_keywords(&weapon.keywords);
  let properties = vec![
    natural.clone(),
    double.clone(),
    reach.clone(),
    block.clone(),
    keywords.clone(),
  ]
  .into_iter()
  .flatten()
  .collect::<Vec<String>>();
  let properties_display = if properties.len() > 0 {
    Some(properties.join(", "))
  } else {
    None
  };
  let characteristics = vec![
    Some(weapon_class.clone()),
    Some(weight_class.clone()),
    handed.clone(),
    natural.clone(),
    double.clone(),
    reach.clone(),
    block.clone(),
    range.clone(),
    reload.clone(),
    keywords.clone(),
  ]
  .into_iter()
  .flatten()
  .collect::<Vec<String>>();
  let characteristics_display = characteristics.join(", ");
  return match &display {
    EquipmentDisplay::Card => rsx! {
      div {
        class: "card grid dim-keywords",
        div { class: "uv-full title", "{title}" }
        div { class: "uv-full",
          span { class: "highlight", "{weight_class} {weapon_class} Weapon" }
          if let Some( keywords ) = keywords {
            span { class: "italics", " - {keywords}" }
          }
        }
        div { class: "uv-full",
          DiceGroupEntry { dice_group } " {damage_class} Damage"
        }
        if let Some( physique_req ) = physique_req {
          div { class: "uv-full", "Physique Requirement {physique_req}" }
        }
        if let Some( range_reload ) = range_reload_display {
          div { class: "uv-full", "{range_reload}" }
        }
        if let Some(properties) = properties_display {
          div { class: "uv-full", "{properties}" }
        }
      }
    },
    EquipmentDisplay::Row => rsx! {
      div { "{title}" }
      div { DiceGroupEntry { dice_group } " {damage_class}" }
      div { class: "centered", "{physique_req_display}" }
      div { "{characteristics_display}" }
    },
  };
}

#[component]
pub fn EquipmentRow(equipment: Equipment) -> Element {
  return match equipment {
    Equipment::Weapon(weapon) => rsx! { WeaponEntry { weapon, display: EquipmentDisplay::Row } },
    Equipment::Armor(armor) => rsx! { ArmorEntry { armor, display: EquipmentDisplay::Row } },
  };
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Armor {
  #[serde(rename = "_id")]
  id: ObjectId,
  title: String,
  armor: i32,
  weight: ArmorWeight,
  fortitude_req: Option<i32>,
  bulk: Option<i32>,
  drag: Option<i32>,
  #[serde(default)]
  special_material: bool,
  #[serde(default)]
  elemental_resistance: bool,
}

impl Ord for Armor {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.special_material.cmp(&other.special_material) {
      Ordering::Equal => {},
      ord => return ord,
    }
    match self.armor.cmp(&other.armor) {
      Ordering::Equal => {},
      ord => return ord,
    }
    match self.fortitude_req.cmp(&other.fortitude_req) {
      Ordering::Equal => {},
      ord => return ord,
    }
    self.title.cmp(&other.title)
  }
}

impl PartialOrd for Armor {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArmorWeight {
  Light,
  Heavy,
}

impl Display for ArmorWeight {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self)
  }
}

#[component]
pub fn ArmorEntry(armor: Armor, #[props(default)] display: EquipmentDisplay) -> Element {
  let title = armor.title;
  let armor_resistance = armor.armor;
  let fortitude_req = armor.fortitude_req;
  let fortitude_req_display = fortitude_req
    .clone()
    .map_or("-".into(), |req| req.to_string());
  let weight = armor.weight;
  let bulk_display = armor.bulk.map(|bulk| format!("Bulk {bulk}"));
  let drag_display = armor.drag.map(|drag| format!("Drag {drag}"));
  let bulk_drag = vec![bulk_display.clone(), drag_display.clone()]
    .into_iter()
    .flatten()
    .collect::<Vec<String>>();
  let bulk_drag_display = if bulk_drag.len() > 0 {
    Some(bulk_drag.join(", "))
  } else {
    None
  };
  let elemental_resistance = if armor.elemental_resistance {
    Some("Elemental Resistance".to_string())
  } else {
    None
  };
  let properties = vec![
    Some(weight.to_string()),
    bulk_display.clone(),
    drag_display.clone(),
    elemental_resistance.clone(),
  ]
  .into_iter()
  .flatten()
  .collect::<Vec<String>>();
  let properties_display = properties.join(", ");
  return match &display {
    EquipmentDisplay::Card => rsx! {
      div {
        class: "card grid dim-keywords",
        div { class: "uv-full title", "{title}" }
        div { class: "uv-full",
          span { class: "highlight", "{weight} Armor" }
        }
        div { class: "uv-full",
          match elemental_resistance {
            Some(_) => rsx!{"{armor_resistance} Physical & Elemental Resistance"},
            _ => rsx!{"{armor_resistance} Physical Resistance"},
          }
        }
        if let Some( fortitude_req ) = fortitude_req {
          div { class: "uv-full", "Physique Requirement {fortitude_req}" }
        }
        if let Some( bulk_drag_display ) = bulk_drag_display {
          div { class: "uv-full", "{bulk_drag_display}" }
        }
      }
    },
    EquipmentDisplay::Row => rsx! {
      div { "{title}" }
      div { class: "centered", "{armor_resistance}" }
      div { class: "centered", "{fortitude_req_display}" }
      div { "{properties_display}" }
    },
  };
}
