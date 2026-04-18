use std::cmp::max;
use std::collections::HashSet;

use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::components::ConstitutionRow;
use crate::character::expertise::ExpertiseComponent;
use crate::character::flow::FlowResourcesBlock;
// use crate::character::prelude::AttributeRow;
use crate::common::{HorizontalBar, StaggeredGrid};
use crate::equipment::armor::{Armor, ArmorEntry};
use crate::equipment::weapon::{Weapon, WeaponEntry};
use crate::path::components::{PathChipsLoader};
// use crate::keyword::prelude::*;
// use crate::path::prelude::*;
// use crate::rules::components::Modifier;
// use crate::skill::component::*;
// use crate::skill::prelude::*;
use crate::Route;
use crate::rules::prelude::*;
use crate::skill::component::SkillCardElementsLoader;

use super::aspects::{BodyStats, TrainingRanks};
// use super::attribute::*;
use super::expertise::ExpertiseEntry;
use super::flow::FlowStat;
use super::resistance::ResistanceDetails;
use super::resistance::Resistances;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSheet {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub name: String,
  pub level: i32,
  pub attributes: AttributeRanks,
  pub training: TrainingRanks,
  pub body: BodyStats,
  pub paths: HashSet<ObjectId>,
  pub skills: Vec<ObjectId>,
  pub flows: Option<Vec<FlowStat>>,
  pub armor: Option<Armor>,
  pub weapons: Option<Vec<Weapon>>,
  pub resistances: Option<Resistances>,
  pub expertise: Option<Vec<ExpertiseEntry>>,
}

#[component]
pub fn SheetTable(
  sheets: Vec<CharacterSheet>,
  #[props(default)] named_url: bool,
) -> Element {
  rsx!(
    for sheet in sheets {
      SheetDetails {
        sheet,
        named_url,
      }
    }
  )
}

fn adjust_for_armor( 
  armor: &Option<Armor>,
  mut attributes: AttributeRanks, 
  resistances: Option<Resistances>, 
  speed: i32,
) -> (
  AttributeRanks, Resistances, i32, i32
) {
  let mut base_resistances = resistances.unwrap_or_default();
  let Some( equiped_armor ) = armor else {
    attributes.update_dodge_with_bulk(0, 0);
    return ( attributes, base_resistances, speed, 3); 
  };
  base_resistances.update_physical_resistance(equiped_armor.physical_resistance);
  attributes.update_dodge_with_bulk(
    equiped_armor.tenacity_requirement, 
    equiped_armor.speed_penalty.unwrap_or_default()
  );
  let speed_penalty = equiped_armor.speed_penalty.unwrap_or_default();
  let speed = max(speed - speed_penalty, 1);
  let dash = max(3 - speed_penalty, 1);
  return ( attributes, base_resistances, speed, dash); 
}

#[component]
pub fn SheetDetails(
  sheet: CharacterSheet,
  #[props(default)] named_url: bool,
) -> Element {
  let id = sheet.id.to_string();
  let name = sheet.name;
  let (attributes, resistances, speed, dash) = adjust_for_armor(
    &sheet.armor,
    sheet.attributes.clone(),
    sheet.resistances.clone(),
    sheet.body.speed,
  );

  let (armor_opt, armor_title) = match &sheet.armor {
    Some( equiped_armor ) => (Some(equiped_armor.clone()), Some(equiped_armor.title.clone())),
    None => (None, None),
  };
  let body = sheet.body;
  let hp = body.hp;
  let path_ids = sheet.paths;
  let skill_ids = sheet.skills;
  let opt_weapons = sheet.weapons;
  let opt_flows = sheet.flows;
  let training = sheet.training;
  rsx! {
    div {
      class: "sheet grid dim-attributes",
      if named_url {
        Link { to: Route::SingleCharacterSheetPage { id }, class: "uv-left-half heavier", "{name}" }
      } else {
        div { class: "uv-left-half heavier", "{name}" }
      }
      div {
        class: "row uv-right-half content-right row-wrap align-center",
        "Level {sheet.level}"
        PathChipsLoader { path_ids }
        " Training: {training}"
      }
      HorizontalBar {}
      div {
        class: "uv-capabilites column underhang",
        div { class: "subheading", "Capabilities" }
        CapabilityBlock { attributes: attributes.clone() }
      }
      div {
        class: "uv-defenses column underhang",
        div { class: "subheading", "Defenses" }
        DefenseBlock { attributes: attributes.clone() }
      }
      div {
        class: "uv-resistances column underhang",
        div { class: "subheading", "Resistances" }
        if let Some( armor_title ) = armor_title {
          div {
            span { class: "highlight", "Armor:" }
            span { " {armor_title}" }
          }
        }
        ResistanceDetails { resistances }
      }
      div {
        class: "uv-expertise column underhang",
        div { class: "subheading", "Expertise" }
        if let Some( expertise ) = sheet.expertise {
          for entry in expertise {
            ExpertiseComponent { entry }
          }
        }
      }
      div {
        class: "uv-resources column underhang",
        div { class: "subheading", "Body" }
        div { "Speed {speed} ( Dash {dash} )" }
        div { "Health {hp}" }
        div { class: "hp-box" }
        ConstitutionRow { constitution: 4 }
      }
      div {
        class: "uv-mid column underhang",
        div { class: "subheading", "Equipment" }
        if let Some( weapons ) = opt_weapons {
          for weapon in weapons {
            WeaponEntry { weapon }
          }
        }
        if let Some( armor ) = armor_opt {
          ArmorEntry { armor }
        }
      }
      if let Some( flows ) = opt_flows {
        div {
          class: "uv-resources column underhang",
          div { class: "subheading", "Resources" }
          FlowResourcesBlock { flows }
        }
      }
    }
    StaggeredGrid {
      SkillCardElementsLoader { skill_ids }
    }
  }
}

// fn calc_dodge_speed_resistances(
//   tenacity: i32,
//   speed: i32,
//   resistances: Resistances,
//   armor: &Option<Armor>,
// ) -> (i32, i32, i32, bool, Resistances) {
//   if armor.is_none() || armor.as_ref().unwrap().tenacity_requirement > tenacity {
//     return (tenacity, speed, DASH_SPEED, false, resistances);
//   }
//   let worn_armor = armor.as_ref().unwrap();
//   let net_tenacity = tenacity - worn_armor.tenacity_requirement;
//   let speed_penalty = worn_armor.speed_penalty.unwrap_or(0);
//   let (dodge, net_speed, net_dash) = match net_tenacity.cmp(&speed_penalty) {
//     std::cmp::Ordering::Less => (
//       0,
//       speed - speed_penalty + net_tenacity,
//       DASH_SPEED - speed_penalty + net_tenacity,
//     ),
//     std::cmp::Ordering::Equal => (0, speed, DASH_SPEED),
//     std::cmp::Ordering::Greater => (net_tenacity - speed_penalty, speed, DASH_SPEED),
//   };
//   let armored_resistances = resistances.with_armor(&armor);
//   return (dodge, net_speed, net_dash, true, armored_resistances);
// }
