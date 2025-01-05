use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::equipment::armor::Armor;

const BASE_RESIST: i32 = 3;

#[component]
pub fn ResistanceDodge( resistances: Resistances, armor: Option<Armor>, dodge: i32 ) -> Element {
  let current_dodge = 1;
  let uncapped: Option<i32> = None;
  rsx!(
    div { "Dodge" }
    div {
      "{current_dodge+10}"
      if let Some( original ) = uncapped {
        " ({original+10})"
      }
    }
    Resistance { resistances }
  )
}

#[component]
pub fn Resistance( resistances: Resistances ) -> Element {
  let mut display_physical = use_signal(|| false);
  let mut display_elemental = use_signal(|| false);
  let mut display_esoteric = use_signal(|| false);
  rsx!(
    div { "Resistances" }
    div {
      onclick: move |_| { display_physical.with_mut(|x| *x = !*x); },
      "Physical"
    }
    div {
      "{resistances.physical.unwrap_or(BASE_RESIST)}"
    }
    SubResistance { title: "Bashing", value: resistances.bashing, display: display_physical() }
    SubResistance { title: "Slashing", value: resistances.slashing, display: display_physical() }
    SubResistance { title: "Piercing", value: resistances.piercing, display: display_physical() }
    div { 
      onclick: move |_| { display_elemental.with_mut(|x| *x = !*x); },
      "Elemental"
    }
    div {
      "{resistances.elemental.unwrap_or(BASE_RESIST)}"
    }
    SubResistance { title: "Fire", value: resistances.fire, display: display_elemental() }
    SubResistance { title: "Cold", value: resistances.cold, display: display_elemental() }
    SubResistance { title: "Lighting", value: resistances.lighting, display: display_elemental() }
    SubResistance { title: "Thunder", value: resistances.thunder, display: display_elemental() }
    SubResistance { title: "Acid", value: resistances.acid, display: display_elemental() }
    div { 
      onclick: move |_| { display_esoteric.with_mut(|x| *x = !*x); },
      "Esoteric"
    }
    div {
      "{resistances.esoteric.unwrap_or(BASE_RESIST)}"
    }
    SubResistance { title: "Force", value: resistances.force, display: display_esoteric() }
    SubResistance { title: "Radiant", value: resistances.radiant, display: display_esoteric() }
    SubResistance { title: "Necrotic", value: resistances.necrotic, display: display_esoteric() }
    SubResistance { title: "Psionic", value: resistances.psionic, display: display_esoteric() }
  )
}

#[component]
fn SubResistance( title: String, value: Option<i32>, display: bool ) -> Element {
  rsx!(
    div {
      class: if !value.is_some() && !display { "hidden" },
      "{title}"
    }
    div {
      class: if !value.is_some() && !display { "hidden" },
      "{value.unwrap_or(BASE_RESIST)}"
    }
  )
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Resistances {
  physical: Option<i32>,
  bashing: Option<i32>,
  slashing: Option<i32>,
  piercing: Option<i32>,

  elemental: Option<i32>,
  fire: Option<i32>,
  cold: Option<i32>,
  lighting: Option<i32>,
  thunder: Option<i32>,
  acid: Option<i32>,

  esoteric: Option<i32>,
  force: Option<i32>,
  radiant: Option<i32>,
  necrotic: Option<i32>,
  psionic: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Damage {
  Bashing,
  Slashing,
  Piercing,
  Fire,
  Cold,
  Lighting,
  Thunder,
  Acid,
  Force,
  Radiant,
  Necrotic,
  Psionic,
}
