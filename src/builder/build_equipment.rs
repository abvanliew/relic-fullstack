use bson::oid::ObjectId;
use dioxus::prelude::*;

use crate::builder::common::{Interactable, SectionBar};
use crate::common::{CollapsibleHeader, StaggeredCell, StaggeredGrid};
use crate::equipment::base::{Equipment, EquipmentCard};
use crate::server::prelude::EquipmentCache;

use super::CharacterBuild;

#[derive(Debug, Clone, Default)]
pub struct ActiveEquipment {
  armor_id: Option<ObjectId>,
}

impl ActiveEquipment {
  pub fn extend(&mut self, other: &Self) {
    if other.armor_id.is_none() { return; }
    self.armor_id = other.armor_id.clone();
  }

  pub fn select_armor(&mut self, armor_id: ObjectId) {
    self.armor_id = match self.armor_id {
      Some(current_id) => {
        if current_id.eq(&armor_id) {
          None
        } else {
          Some(armor_id)
        }
      },
      None => Some(armor_id),
    }
  }

  pub fn is_worn_armor(&self, other_id: &ObjectId) -> bool {
    return match self.armor_id {
      Some(id) => id.eq(other_id),
      None => false,
    };
  }
}

impl CharacterBuild {
  pub fn select_armor(&mut self, armor_id: ObjectId) {
    let level_selection = self.get_current_mut();
    level_selection.equipment.select_armor(armor_id);
  }

  pub fn get_selectable_equipment(&self) -> Vec<(Interactable, Equipment)> {
    let EquipmentCache(ref equipment_cache) = use_context();
    let level_selection = self.current_selection();
    let (_physique, fortitude) = level_selection.get_effective_physique_fortitude();
    let equipment_list = equipment_cache.into_vec();
    let (_weapons, armors) = Equipment::parition(equipment_list);
    let armor_selection = armors
      .into_iter()
      .filter(|armor| !armor.is_special_material())
      .map(|equipment| {
        let worn = level_selection.equipment.is_worn_armor(&equipment.id());
        (
          match &equipment {
            Equipment::Armor(armor) => match (armor.usable(fortitude), worn) {
              (_, true) => Interactable::Deselectable,
              (true, _) => Interactable::Selectable,
              (false, _) => Interactable::LockedOut,
            },
            _ => Interactable::LockedOut,
          },
          equipment,
        )
      })
      .collect::<Vec<_>>();
    return armor_selection;
  }
}

#[component]
pub fn EquipmentSelector(mut build_signal: Signal<CharacterBuild>) -> Element {
  let armors = build_signal().get_selectable_equipment();
  return rsx! {
    SectionBar {
      title: "Equipment",
      bar: rsx! {},
      CollapsibleHeader {
        class: "thin-border minimal-background heavier slightlight",
        header: rsx! { "Armor" },
        StaggeredGrid {
          class: "indent stg-medium flow-medium",
          for (interactable, equipment) in armors {
            ArmorSelector { build_signal, interactable, equipment }
          }
        }
      }
    }
  };
}

#[component]
pub fn ArmorSelector(
  mut build_signal: Signal<CharacterBuild>, interactable: Interactable, equipment: Equipment,
) -> Element {
  let class = match &interactable {
    Interactable::Deselectable => "no-select selected",
    Interactable::LockedOut => "no-select disabled",
    _ => "no-select",
  };
  let id = equipment.id();
  return rsx! {
    StaggeredCell {
      EquipmentCard {
        class, equipment,
        onclick: move |_| {
          match interactable {
            Interactable::LockedIn | Interactable::LockedOut => { return; }
            _ => ()
          };
          let mut new_build = build_signal();
          new_build.select_armor(id.clone());
          build_signal.set(new_build);
        }
      }
    }
  };
}
