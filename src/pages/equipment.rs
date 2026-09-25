use dioxus::prelude::*;

use crate::common::{StaggeredCell, StaggeredGrid};
use crate::equipment::base::{Equipment, EquipmentCard, EquipmentRow};
use crate::equipment::enchantment::EnchantmentDetails;
use crate::rules::prelude::WeaponExplainer;
use crate::server::prelude::*;

#[component]
pub fn EnchantmentsPage() -> Element {
  EnchantmentCache::use_context_provider();
  let EnchantmentCache(ref enchantment_cache) = use_context();
  if let Some(status) = enchantment_cache.status_element() {
    return status;
  }
  let enchantments = enchantment_cache.into_vec();
  return rsx! {
    div {
      class: "column gap-large",
      StaggeredGrid {
        class: "stg-large",
        for enchantment in enchantments {
          StaggeredCell {
            EnchantmentDetails { enchantment }
          }
        }
      }
    }
  };
}

#[component]
pub fn EquipmentPage() -> Element {
  let EquipmentCache(ref equipment_cache) = use_context();
  if let Some(status) = equipment_cache.status_element() {
    return status;
  }
  let mut equipment_list = equipment_cache.into_vec();
  equipment_list.sort();
  let (weapons, armors_all): (Vec<Equipment>, Vec<Equipment>) = Equipment::parition(equipment_list.clone());
  let armors: Vec<Equipment> = armors_all.clone().into_iter().filter(|armor| !armor.is_special_material()).collect();
  return rsx! {
    div {
      class: "column gap-large",
      WeaponExplainer {}
      div {
        class: "grid dim-table padded-grid alt-background-4",
        div { class: "subheading sink", "Armor" }
        div { class: "subheading sink", "Resistance" }
        div { class: "subheading centered sink", "Fortitude Requirement" }
        div { class: "subheading sink", "Characteristics" }
        for equipment in armors {
          EquipmentRow { equipment }
        }
      }
      div {
        class: "grid dim-table padded-grid alt-background-4",
        div { class: "subheading sink", "Weapon" }
        div { class: "subheading sink", "Damage" }
        div { class: "subheading centered sink", "Physique Requirement" }
        div { class: "subheading sink", "Characteristics" }
        for equipment in weapons {
          EquipmentRow { equipment }
        }
      }
      StaggeredGrid {
        class: "stg-medium flow-medium break-before",
        for equipment in equipment_list {
          StaggeredCell {
            EquipmentCard { equipment }
          }
        }
      }
    }
  };
}

#[component]
pub fn QuickTerm(title: String, children: Element) -> Element {
  rsx! {
    div { span { class: "highlight bumper", "{title}" } "- " {children} }
  }
}
