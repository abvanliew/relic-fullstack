use std::collections::HashMap;

use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::expertise::ExpertiseComponent;
use crate::character::flow::FlowResourcesBlock;
// use crate::character::prelude::*;
// use crate::rule::prelude::*;
use crate::equipment::armor::Armor;
use crate::equipment::weapon::{Weapon, WeaponEntry};
use crate::path::Path;
use crate::rule::components::Modifier;
use crate::skill::prelude::*;
use crate::Route;

use super::aspects::{BodyStats, TrainingRanks};
use super::attribute::*;
use super::expertise::ExpertiseEntry;
use super::flow::FlowStat;
use super::resistance::ResistanceDetails;
use super::resistance::Resistances;

const DASH_SPEED: i32 = 3;
pub const BASE_DEFENSE: i32 = 11;

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
  pub paths: Vec<ObjectId>,
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
  skills: Vec<Skill>,
  paths: Vec<Path>,
  keywords: ReadOnlySignal<HashMap<String, Keyword>>,
  named_url: bool,
) -> Element {
  rsx!(for sheet in sheets {
    SheetDetails {
      sheet,
      skills: skills.clone(),
      paths: paths.clone(),
      keywords,
      named_url,
    }
  })
}

#[component]
pub fn SheetDetails(
  sheet: CharacterSheet,
  skills: Vec<Skill>,
  paths: Vec<Path>,
  keywords: ReadOnlySignal<HashMap<String, Keyword>>,
  named_url: bool,
) -> Element {
  let id = sheet.id.to_string();
  let name = sheet.name;
  let mut path_names: Vec<String> = Vec::new();
  for path in &sheet.paths {
    let results: Vec<Path> = paths
      .clone()
      .into_iter()
      .filter(|p| p.id == *path)
      .collect();
    if results.len() != 1 {
      continue;
    }
    path_names.push(results[0].title.clone());
  }
  let joined_paths = path_names.join(", ");
  let attributes = sheet.attributes.clone();
  let armor: Option<Armor> = sheet.armor.clone();
  let mut selected_skills: Vec<Skill> = Vec::new();
  selected_skills.resize(sheet.skills.len(), Skill::default());
  let mut match_count: usize = 0;
  'outer: for i in 0..skills.len() {
    for c in 0..sheet.skills.len() {
      if skills[i].id != sheet.skills[c] {
        continue;
      }
      selected_skills[c] = skills[i].clone();
      match_count += 1;
      if match_count == sheet.skills.len() {
        break 'outer;
      }
      break;
    }
  }
  let hp = sheet.body.hp;
  let (dodge, speed, dash, armor_equiped, armored_resistances) = calc_dodge_speed_resistances(
    attributes.tenacity,
    sheet.body.speed,
    sheet.resistances.clone().unwrap_or_default(),
    &sheet.armor,
  );
  let opt_weapons = sheet.weapons;
  let opt_flows = sheet.flows;
  rsx!(
    div {
      class: "column print-block sheet",
      div {
        class: "row",
        if named_url {
          Link { to: Route::SingleChracterSheet { id }, class: "heavier", "{name}" }
        } else {
          div { class: "heavier", "{name}" }
        }
      }
      div {
        class: "grid dim-thirds",
        div { "Level {sheet.level}" }
        if path_names.len() > 0 {
          div {
            class: "centered",
            span { class: "highlight", "Paths" }
            span { " {joined_paths}" }
          }
        }
        div {
          span { class: "highlight", "Training" }
          span { " {sheet.training}" }
        }
      }
      div {
        class: "grid dim-attributes gap-2xlarge",
        AttributeBlock { attributes, dodge }
        div {
          class: "uv-resistances column",
          div { class: "subheading", "Resistances" }
          if let Some( worn_armor ) = armor {
            div {
              span { class: "highlight", "Armor:" }
              span { class: if !armor_equiped { "disabled" }, " {worn_armor.title}" }
            }
          }
          ResistanceDetails { resistances: armored_resistances }
        }
        div {
          class: "uv-expertise column",
          div { class: "subheading", "Body" }
          div { "Speed {speed}" }
          div { "Dash {dash}" }
          ConstitutionRow { constitution: 4 }
          div { "Health {hp}" }
          div { class: "hp-box" }
        }
        div {
          class: "uv-capabilities column",
          div { class: "subheading", "Expertise" }
          if let Some( expertise ) = sheet.expertise {
            for entry in expertise {
              ExpertiseComponent { entry }
            }
          }
        }
        div {
          class: "uv-other row",
          if let Some( weapons ) = opt_weapons {
            div {
              class: "column-wrap",
              div { class: "subheading", "Weapons" }
              for weapon in weapons {
                WeaponEntry { weapon }
              }
            }
          }
          if let Some( flows ) = opt_flows {
            div {
              class: "column align-right",
              div { class: "subheading", "Resources" }
              FlowResourcesBlock { flows }
            }
          }
        }
      }
    }
    div {
      class: "column print-break",
      div {
        class: "row-wrap",
        for skill in selected_skills {
          SkillDescription { id: skill.id.to_string(), show_terms: true }
        }
      }
    }
  )
}

#[derive(PartialEq, Props, Clone)]
pub struct AttributeRowProps {
  name: String,
  element: Element,
  #[props(default)]
  name_class: Option<String>,
}

#[component]
pub fn AttributeRow(props: AttributeRowProps) -> Element {
  let name = props.name;
  let element = props.element;
  let name_class = props.name_class.unwrap_or("highlight".into());
  rsx!(
    div {
      class: "row full",
      div {
        class: name_class,
        "{name}"
      }
      div {
        class: "align-right",
        { element }
      }
    }
  )
}

fn calc_dodge_speed_resistances(
  tenacity: i32,
  speed: i32,
  resistances: Resistances,
  armor: &Option<Armor>,
) -> (i32, i32, i32, bool, Resistances) {
  if armor.is_none() || armor.as_ref().unwrap().tenacity_requirement > tenacity {
    return (tenacity, speed, DASH_SPEED, false, resistances);
  }
  let worn_armor = armor.as_ref().unwrap();
  let net_tenacity = tenacity - worn_armor.tenacity_requirement;
  let speed_penalty = worn_armor.speed_penalty.unwrap_or(0);
  let (dodge, net_speed, net_dash) = match net_tenacity.cmp(&speed_penalty) {
    std::cmp::Ordering::Less => (
      0,
      speed - speed_penalty + net_tenacity,
      DASH_SPEED - speed_penalty + net_tenacity,
    ),
    std::cmp::Ordering::Equal => (0, speed, DASH_SPEED),
    std::cmp::Ordering::Greater => (net_tenacity - speed_penalty, speed, DASH_SPEED),
  };
  let armored_resistances = resistances.with_armor(&armor);
  return (dodge, net_speed, net_dash, true, armored_resistances);
}

#[component]
pub fn AttributeBlock(attributes: AttributeRanks, dodge: i32) -> Element {
  rsx!(
    div {
      class: "uv-capabilities column",
      div { class: "subheading", "Capabilites" }
      AttributeRow {
        name: "Physique",
        element: rsx!( Modifier { value: attributes.physique } ),
      }
      AttributeRow {
        name: "Warfare",
        element: rsx!( Modifier { value: attributes.warfare } ),
      }
      AttributeRow {
        name: "Spirit",
        element: rsx!( Modifier { value: attributes.spirit } ),
      }
      AttributeRow {
        name: "Manipulation",
        element: rsx!( Modifier { value: attributes.manipulation } ),
      }
    }
    div {
      class: "uv-defenses column",
      div { class: "subheading", "Defenses" }
      AttributeRow {
        name: "Tenacity",
        element: rsx!( "{attributes.tenacity + BASE_DEFENSE}" ),
      }
      AttributeRow {
        name: "Fortitude",
        element: rsx!( "{attributes.fortitude + BASE_DEFENSE}" ),
      }
      AttributeRow {
        name: "Resolve",
        element: rsx!( "{attributes.resolve + BASE_DEFENSE}" ),
      }
      AttributeRow {
        name: "Insight",
        element: rsx!( "{attributes.insight + BASE_DEFENSE}" ),
      }
      AttributeRow {
        name: "Dodge",
        element: rsx!( "{dodge + BASE_DEFENSE}" ),
      }
    }
  )
}

#[component]
pub fn ConstitutionRow(constitution: i32) -> Element {
  rsx!(
    div { "Constituion {constitution}" }
    BoxRow { count: constitution }
  )
}

#[component]
pub fn BoxRow(count: i32) -> Element {
  rsx!(
    div {
      class: "box-row",
      if count > 0 {
        for _ in 0..count { div { class: "box" } }
      }
    }
  )
}
