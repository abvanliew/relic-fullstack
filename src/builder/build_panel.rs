use dioxus::prelude::*;

use super::build_features::FeatureGroup;
use super::common::SectionBar;
use super::CharacterBuild;

use crate::builder::build_attributes::AttributeSelector;
use crate::builder::build_development::GrowthGroup;
use crate::builder::build_equipment::EquipmentSelector;
use crate::builder::build_paths::PathGroup;
use crate::progression::fixed::{MAX_LEVEL, MIN_LEVEL};
use crate::progression::prelude::LevelTable;

#[component]
pub fn CharacterBuildPanel() -> Element {
  let build_signal = use_signal(|| CharacterBuild::default());
  let (a, b) = build_signal().get_selection_pair();
  return rsx! {
    div { "{a:#?}"}
    div { "{b:#?}"}
    div{ class: "column gap-large",
      CharacterGroup { build_signal }
      PathGroup { build_signal }
      GrowthGroup { build_signal }
      FeatureGroup { build_signal }
      AttributeSelector { build_signal }
      EquipmentSelector { build_signal }
    }
  };
}

#[component]
pub fn CharacterGroup(mut build_signal: Signal<CharacterBuild>) -> Element {
  let level = build_signal().get_level();
  let name = build_signal().get_character_name();
  let name_display = if name.trim().is_empty() {
    "---".into()
  } else {
    name.clone()
  };
  rsx! {
    SectionBar {
      title: "Character",
      bar: rsx! {
        "{name_display} Lvl {level}"
      },
      div {
        class: "row-wrap",
        div {
          class: "middle thin-border fit-width minimal-background",
          span { class: "highlight bumper", "Level" }
          select {
            class: "big-text",
            autocomplete: "off",
            onclick: move |event| { event.stop_propagation(); },
            onchange: move |event| {
              let mut new_build = build_signal().clone();
              let new_level = event.value().parse::<i32>().ok().unwrap_or(MIN_LEVEL).max(MIN_LEVEL).min(MAX_LEVEL);
              if new_level != level {
                new_build.set_level(new_level);
                build_signal.set(new_build);
              }
            },
            for lvl in MIN_LEVEL..=MAX_LEVEL {
              option { value: lvl, label: lvl, selected: level == lvl, }
            }
          }
        }
        div {
          class: "middle thin-border fit-width minimal-background",
          span { class: "highlight bumper", "Name" }
          input {
            class: "input fixed-title-width-large left",
            value: "{name}",
            oninput: move |event| {
              event.stop_propagation();
              let mut new_build = build_signal();
              new_build.set_character_name(event.value());
              build_signal.set(new_build);
            }
          }
        }
      }
      LevelTable { highlight_level: level }
    }
  }
}

