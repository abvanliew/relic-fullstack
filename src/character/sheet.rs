use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

use crate::character::resistance::ResistanceDetails;
use crate::equipment::armor::Armor;
use crate::equipment::weapon::Weapon;
use crate::path::Path;
use crate::rule::components::Modifier;
use crate::skill::prelude::{SkillDescription, SkillTable};
use crate::skill::Skill;

use super::attribute::*;
use super::aspects::{BodyStats, FlowStat, TrainingRanks};
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
  pub paths: Vec<ObjectId>,
  pub skills: Vec<ObjectId>,
  pub flows: Option<Vec<FlowStat>>,
  pub armor: Option<Armor>,
  pub weapons: Option<Vec<Weapon>>,
  pub resistances: Option<Resistances>,
}

#[component]
pub fn SheetTable( sheets: Vec<CharacterSheet>, skills: Vec<Skill>, paths: Vec<Path> ) -> Element {
  rsx!(
    for sheet in sheets {
      SheetDetails { sheet, skills: skills.clone(), paths: paths.clone() }
    }
  )
}

#[component]
pub fn SheetDetails( sheet: CharacterSheet, skills: Vec<Skill>, paths: Vec<Path> ) -> Element {
  let mut path_names: Vec<String> = Vec::new();
  for path in &sheet.paths {
    let results: Vec<Path> = paths.clone().into_iter().filter(|p| p.id == *path ).collect();
    if results.len() != 1 { continue; }
    path_names.push( results[0].title.clone() );
  }
  let joined_paths = path_names.join(", ");
  let attributes = sheet.attributes.clone();
  let resistances = sheet.resistances.clone().unwrap_or_default();
  let armor: Option<Armor> = sheet.armor.clone();
  let mut selected_skills: Vec<Skill> = Vec::new();
  selected_skills.resize( sheet.skills.len(), Skill::default() );
  let mut match_count: usize = 0;
  'outer: for i in 0..skills.len() {
    for c in 0..sheet.skills.len() {
      if skills[i].id != sheet.skills[c] { continue; }
      selected_skills[c] = skills[i].clone();
      match_count += 1;
      if match_count == sheet.skills.len() { break 'outer; }
      break;
    }
  }
  rsx!(
    // "{sheet:?}"
    // "{selected_skills:?}"
    div {
      class: "column",
      // icon and player details
      div {
        class: "row",
        div { "Name: {sheet.name}" }
        div { "Level: {sheet.level}" }
        if path_names.len() > 0 {
          div { "Paths: {joined_paths}"}
        }
        div { "Training: {sheet.training}" }
      }
      div {
        class: "row",
        AttributeBlock { attributes, resistances, armor }
      }
      div {
        class: "row-wrap",
        for skill in selected_skills {
          SkillDescription { skill }
        }
      }
    }
  )
}

#[component]
pub fn DodgeValue( armor: Option<Armor>, dodge: i32 ) -> Element {
  let ( current_dodge, uncapped ) = match armor {
    Some( worn_armor) => match worn_armor.max_dodge {
      Some( max_dodge ) => if dodge <= max_dodge {
        ( dodge, None ) } else { ( max_dodge, Some( dodge ) )
      },
      None => ( dodge, None ),
    },
    None => ( dodge, None ),
  };
  rsx!(
    "{current_dodge + 10}"
    if let Some( original ) = uncapped {
      " ({original + 10})"
    }
  )
}

#[component]
pub fn AttributeRow( name: String, element: Element ) -> Element {
  rsx!(
    div {
      class: "row",
      div { "{name}" }
      div { { element } }
    }
  )
}

#[component]
pub fn AttributeBlock( attributes: AttributeRanks, resistances: Resistances, armor: Option<Armor> ) -> Element {
  let armored_resistances = resistances.with_armor( &armor );
  rsx!(
    div {
      class: "column",
      div { "Capabilites" }
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
      class: "column",
      div { "Defenses" }
      AttributeRow {
        name: "Tenacity",
        element: rsx!( "{attributes.tenacity + 10}" ),
      }
      AttributeRow {
        name: "Fortitude",
        element: rsx!( "{attributes.fortitude + 10}" ),
      }
      AttributeRow {
        name: "Resolve",
        element: rsx!( "{attributes.resolve + 10}" ),
      }
      AttributeRow {
        name: "Insight",
        element: rsx!( "{attributes.insight + 10}" ),
      }
      AttributeRow {
        name: "Dodge",
        element: rsx!( DodgeValue { armor, dodge: attributes.dodge } ),
      }
    }
    div {
      class: "column",
      div { "Armor" }
      ResistanceDetails { resistances: armored_resistances }
    }
    div {
      class: "column",
      div { "Speed 6" }
      div { "Health & Con" }
      div { "Expertise" }
    }
  )
}
