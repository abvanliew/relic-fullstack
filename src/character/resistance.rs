use std::{cmp::max, fmt};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{character::sheet::AttributeRow, equipment::armor::Armor};

const BASE_RESIST: i32 = 3;

#[component]
pub fn ResistanceDetails( resistances: Resistances ) -> Element {
  let mut display_physical = use_signal(|| false);
  let mut display_elemental = use_signal(|| false);
  let mut display_esoteric = use_signal(|| false);
  rsx!(
    div {
      class: "row full",
      onclick: move |_| { display_physical.with_mut(|x| *x = !*x); },
      AttributeRow {
        name: "Physical", element: rsx!( "{resistances.get_category( &DamageCategory::Physical )}" )
      }
    }
    SubResistance { details: resistances.show_damage( &Damage::Bashing ), display: display_physical() }
    SubResistance { details: resistances.show_damage( &Damage::Slashing ), display: display_physical() }
    SubResistance { details: resistances.show_damage( &Damage::Piercing ), display: display_physical() }
    div {
      class: "row full",
      onclick: move |_| { display_elemental.with_mut(|x| *x = !*x); },
      AttributeRow {
        name: "Elemental", element: rsx!( "{resistances.get_category( &DamageCategory::Elemental )}" )
      }
    }
    SubResistance { details: resistances.show_damage( &Damage::Fire ), display: display_elemental() }
    SubResistance { details: resistances.show_damage( &Damage::Cold ), display: display_elemental() }
    SubResistance { details: resistances.show_damage( &Damage::Lighting ), display: display_elemental() }
    SubResistance { details: resistances.show_damage( &Damage::Thunder ), display: display_elemental() }
    SubResistance { details: resistances.show_damage( &Damage::Acid ), display: display_elemental() }
    div {
      class: "row full",
      onclick: move |_| { display_esoteric.with_mut(|x| *x = !*x); },
      AttributeRow {
        name: "Esoteric", element: rsx!( "{resistances.get_category( &DamageCategory::Esoteric )}" )
      }
    }
    SubResistance { details: resistances.show_damage( &Damage::Force ), display: display_esoteric() }
    SubResistance { details: resistances.show_damage( &Damage::Radiant ), display: display_esoteric() }
    SubResistance { details: resistances.show_damage( &Damage::Necrotic ), display: display_esoteric() }
    SubResistance { details: resistances.show_damage( &Damage::Psionic ), display: display_esoteric() }
  )
}

#[component]
fn SubResistance( details: ( String, i32, bool ), display: bool ) -> Element {
  let ( name, value, show ) = details;
  rsx!(
    div {
      class: if show || display { "row full" } else { "hidden" },
      AttributeRow { name: "{name}", element: rsx!( "{value}" ) }
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

impl Resistances {
  pub fn with_armor( &self, armor: &Option<Armor> ) -> Self {
    return Self {
      physical: match ( self.physical, armor ) {
        ( Some( resist_value ), Some( worn_armor ) ) =>
          Some( max( resist_value, worn_armor.physical_resistance + BASE_RESIST ) ),
        ( Some( resist_value ), None ) => Some( resist_value ),
        ( None, Some( worn_armor ) ) => Some( worn_armor.physical_resistance + BASE_RESIST ),
        ( None, None ) => None,
      },
      ..self.clone()
    }
  }

  fn category_ref( &self, category: &DamageCategory ) -> &Option<i32> {
    match category {
      DamageCategory::Physical => &self.physical,
      DamageCategory::Elemental => &self.elemental,
      DamageCategory::Esoteric => &self.esoteric,
    }
  }

  fn damage_ref( &self, damage: &Damage ) -> &Option<i32> {
    match damage {
      Damage::Bashing => &self.bashing,
      Damage::Slashing => &self.slashing,
      Damage::Piercing => &self.piercing,
      Damage::Fire => &self.fire,
      Damage::Cold => &self.cold,
      Damage::Lighting => &self.lighting,
      Damage::Thunder => &self.thunder,
      Damage::Acid => &self.acid,
      Damage::Force => &self.force,
      Damage::Radiant => &self.radiant,
      Damage::Necrotic => &self.necrotic,
      Damage::Psionic => &self.psionic,
    }
  }

  pub fn get_category( &self, category: &DamageCategory ) -> i32 {
    return self.category_ref( category ).unwrap_or( BASE_RESIST )
  }

  pub fn show_damage( &self, damage: &Damage ) -> ( String, i32, bool ) {
    return match self.damage_ref( damage ) {
      Some( resist) => ( damage.to_string(), *resist, true ),
      None => ( damage.to_string(), self.get_category( &damage.category() ), false ),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DamageCategory {
  Physical,
  Elemental,
  Esoteric,
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

impl fmt::Display for Damage {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
        Damage::Acid => "Acid",
        Damage::Bashing => "Bashing",
        Damage::Slashing => "Slashing",
        Damage::Piercing => "Piercing",
        Damage::Fire => "Fire",
        Damage::Cold => "Cold",
        Damage::Lighting => "Lighting",
        Damage::Thunder => "Thunder",
        Damage::Force => "Force",
        Damage::Radiant => "Radiant",
        Damage::Necrotic => "Necrotic",
        Damage::Psionic => "Psionic",
    } )
  }
}

impl Damage {
  fn category( &self ) -> DamageCategory {
    match self {
      // Physical
      Damage::Bashing => DamageCategory::Physical,
      Damage::Slashing => DamageCategory::Physical,
      Damage::Piercing => DamageCategory::Physical,
  
      // Elemental
      Damage::Fire => DamageCategory::Elemental,
      Damage::Cold => DamageCategory::Elemental,
      Damage::Lighting => DamageCategory::Elemental,
      Damage::Thunder => DamageCategory::Elemental,
      Damage::Acid => DamageCategory::Elemental,
  
      // Esoteric
      Damage::Force => DamageCategory::Esoteric,
      Damage::Radiant => DamageCategory::Esoteric,
      Damage::Necrotic => DamageCategory::Esoteric,
      Damage::Psionic => DamageCategory::Esoteric,
    }
  }
}
