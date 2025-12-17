use std::{cmp::max, fmt};

use crate::{character::sheet::AttributeRow, equipment::armor::Armor};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const BASE_RESIST: i32 = 0;

#[derive(PartialEq, Props, Clone)]
pub struct ResistanceDetailsProps {
  resistances: Resistances,
  #[props(default)]
  display: ResistanceDisplay,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ResistanceDisplay {
  #[default]
  Base,
}

#[component]
pub fn ResistanceDetails(props: ResistanceDetailsProps) -> Element {
  let resistances = props.resistances;
  let physical_total = resistances.get_category(&DamageCategory::Physical);
  let elemental_total = resistances.get_category(&DamageCategory::Elemental);
  let esoteric_total = resistances.get_category(&DamageCategory::Esoteric);
  rsx!(
    div {
      class: "row full",
      AttributeRow {
        name: "Physical", name_class: "",
        element: rsx!( "{physical_total}" ),
      }
    }
    SubResistance { details: resistances.show_damage( &DamageClass::Bashing ) }
    SubResistance { details: resistances.show_damage( &DamageClass::Slashing ) }
    SubResistance { details: resistances.show_damage( &DamageClass::Piercing ) }
    div {
      class: "row full",
      AttributeRow {
        name: "Elemental", name_class: "",
        element: rsx!( "{elemental_total}" ),
      }
    }
    SubResistance { details: resistances.show_damage( &DamageClass::Fire ) }
    SubResistance { details: resistances.show_damage( &DamageClass::Cold ) }
    SubResistance { details: resistances.show_damage( &DamageClass::Lighting ) }
    SubResistance { details: resistances.show_damage( &DamageClass::Thunder ) }
    SubResistance { details: resistances.show_damage( &DamageClass::Acid ) }
    div {
      class: "row full",
      AttributeRow {
        name: "Esoteric", name_class: "",
        element: rsx!( "{esoteric_total}" ),
      }
    }
    SubResistance { details: resistances.show_damage( &DamageClass::Force ) }
    SubResistance { details: resistances.show_damage( &DamageClass::Radiant ) }
    SubResistance { details: resistances.show_damage( &DamageClass::Necrotic ) }
    SubResistance { details: resistances.show_damage( &DamageClass::Psionic ) }
  )
}

#[component]
fn SubResistance(details: (String, i32, bool)) -> Element {
  let (name, value, show) = details;
  rsx!(
    div {
      class: if show { "row full" } else { "hidden" },
      AttributeRow { name: "{name}", name_class: "indent", element: rsx!( "{value}" ) }
    }
  )
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Eq)]
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
  pub fn with_armor(&self, armor: &Option<Armor>) -> Self {
    return Self {
      physical: match (self.physical, armor) {
        (Some(resist_value), Some(worn_armor)) => Some(max(
          resist_value,
          worn_armor.physical_resistance + BASE_RESIST,
        )),
        (Some(resist_value), None) => Some(resist_value),
        (None, Some(worn_armor)) => Some(worn_armor.physical_resistance + BASE_RESIST),
        (None, None) => None,
      },
      ..self.clone()
    };
  }

  fn category_ref(&self, category: &DamageCategory) -> &Option<i32> {
    match category {
      DamageCategory::Physical => &self.physical,
      DamageCategory::Elemental => &self.elemental,
      DamageCategory::Esoteric => &self.esoteric,
    }
  }

  fn damage_ref(&self, damage: &DamageClass) -> &Option<i32> {
    match damage {
      DamageClass::Bashing => &self.bashing,
      DamageClass::Slashing => &self.slashing,
      DamageClass::Piercing => &self.piercing,
      DamageClass::Fire => &self.fire,
      DamageClass::Cold => &self.cold,
      DamageClass::Lighting => &self.lighting,
      DamageClass::Thunder => &self.thunder,
      DamageClass::Acid => &self.acid,
      DamageClass::Force => &self.force,
      DamageClass::Radiant => &self.radiant,
      DamageClass::Necrotic => &self.necrotic,
      DamageClass::Psionic => &self.psionic,
    }
  }

  pub fn get_category(&self, category: &DamageCategory) -> i32 {
    return self.category_ref(category).unwrap_or(BASE_RESIST);
  }

  pub fn category_has_child_value(&self, category: &DamageCategory) -> bool {
    return match category {
      DamageCategory::Physical => {
        self.bashing.is_some() || self.slashing.is_some() || self.piercing.is_some()
      }
      DamageCategory::Elemental => {
        self.fire.is_some()
          || self.cold.is_some()
          || self.lighting.is_some()
          || self.thunder.is_some()
          || self.acid.is_some()
      }
      DamageCategory::Esoteric => {
        self.force.is_some()
          || self.radiant.is_some()
          || self.necrotic.is_some()
          || self.psionic.is_some()
      }
    };
  }

  pub fn show_damage(&self, damage: &DamageClass) -> (String, i32, bool) {
    return match self.damage_ref(damage) {
      Some(resist) => (damage.to_string(), *resist, true),
      None => (
        damage.to_string(),
        self.get_category(&damage.category()),
        false,
      ),
    };
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DamageCategory {
  Physical,
  Elemental,
  Esoteric,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum DamageClass {
  #[default]
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

impl fmt::Display for DamageClass {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        DamageClass::Acid => "Acid",
        DamageClass::Bashing => "Bashing",
        DamageClass::Slashing => "Slashing",
        DamageClass::Piercing => "Piercing",
        DamageClass::Fire => "Fire",
        DamageClass::Cold => "Cold",
        DamageClass::Lighting => "Lighting",
        DamageClass::Thunder => "Thunder",
        DamageClass::Force => "Force",
        DamageClass::Radiant => "Radiant",
        DamageClass::Necrotic => "Necrotic",
        DamageClass::Psionic => "Psionic",
      }
    )
  }
}

impl DamageClass {
  fn category(&self) -> DamageCategory {
    match self {
      // Physical
      DamageClass::Bashing => DamageCategory::Physical,
      DamageClass::Slashing => DamageCategory::Physical,
      DamageClass::Piercing => DamageCategory::Physical,

      // Elemental
      DamageClass::Fire => DamageCategory::Elemental,
      DamageClass::Cold => DamageCategory::Elemental,
      DamageClass::Lighting => DamageCategory::Elemental,
      DamageClass::Thunder => DamageCategory::Elemental,
      DamageClass::Acid => DamageCategory::Elemental,

      // Esoteric
      DamageClass::Force => DamageCategory::Esoteric,
      DamageClass::Radiant => DamageCategory::Esoteric,
      DamageClass::Necrotic => DamageCategory::Esoteric,
      DamageClass::Psionic => DamageCategory::Esoteric,
    }
  }
}
