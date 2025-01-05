use dioxus::prelude::*;

use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

use crate::character::attribute::AttributeDetails;
use crate::character::resistance_dodge::ResistanceDodge;
use crate::equipment::{armor::Armor, weapon::Weapon};
use crate::skill::Skill;

use super::attribute::AttributeSet;
use super::resistance_dodge::Resistances;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Character {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub name: String,
  pub player: Option<String>,
  pub level: Option<i32>,
  pub attributes: AttributeSet,
  pub equiped_armor: Option<Armor>,
  pub resistances: Option<Resistances>,
  pub equiped_weapons: Option<Vec<Weapon>>,
  pub skills: Option<Vec<Skill>>,
}

#[component]
pub fn CharacterDetails( character: Character ) -> Element {
  let dodge = character.attributes.dodge.clone();
  rsx!(
    div { "Name" }
    div { "{character.name}" }
    AttributeDetails { attributes: character.attributes.to_owned() }
    ResistanceDodge { resistances: character.resistances.unwrap_or_default(), armor: character.equiped_armor.to_owned(), dodge }
  )
}
