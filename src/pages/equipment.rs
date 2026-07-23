use dioxus::prelude::*;

use crate::equipment::base::{Equipment, EquipmentCard, EquipmentRow};
use crate::rules::prelude::WeaponExplainer;
use crate::server::prelude::*;
use crate::equipment::enchantment::EnchantmentDetails;
use crate::common::{StaggeredCell, StaggeredGrid};

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
  EquipmentCache::use_context_provider();
  let EquipmentCache(ref equipment_cache) = use_context();
  if let Some(status) = equipment_cache.status_element() {
    return status;
  }
  let mut equipment_list = equipment_cache.into_vec();
  equipment_list.sort();
  let (weapons, armors_all): (Vec<Equipment>, Vec<Equipment>) = equipment_list.clone().into_iter().partition(
    |equipment| matches!(equipment, Equipment::Weapon(_))
  );
  let armors = armors_all.into_iter().filter(
    |equipment| ! equipment.is_special_material()
  ).collect::<Vec<Equipment>>();
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
        class: "staggered-medium break-before",
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
pub fn QuickTerm(
  title: String,
  children: Element,
) -> Element {
  rsx! {
    div { span { class: "highlight bumper", "{title}" } "- " {children} }
  }
}