use dioxus::prelude::*;

use super::CharacterBuild;
use super::build_common::{SectionBar};
use super::build_features::FeatureGroup;

use crate::builder::build_attributes::AttributeSelector;
use crate::builder::build_paths::PathGroup;
use crate::builder::build_growth::GrowthGroup;
use crate::progression::fixed::{MAX_LEVEL, MIN_LEVEL};
use crate::progression::prelude::{LevelTable};

#[component]
pub fn CharacterBuildPanel() -> Element {
  let build_signal = use_signal(|| CharacterBuild::default());
  // let build_debug = build_signal().clone();
  return rsx! {
    div{ class: "column gap-large",
      // div { "{build_debug:#?}" }
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
  let mut name = use_signal(String::new);
  rsx! {
    SectionBar {
      title: "Character",
      bar: rsx! {
      },
      div {
        class: "row compact-badge indent-small padding tertiary",
        span { class: "highlight bumper-small", "Name" }
        input {
          class: "input full",
          value: "{name}",
          oninput: move |event| {
            event.stop_propagation();
            name.set(event.value());
          }
        }
      }
      div {
        class: "compact-badge indent-small padding tertiary",
        span { class: "highlight bumper-small", "Level" }
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
      LevelTable { highlight_level: level }
    }
  }
}

#[component]
pub fn EquipmentSelector(mut build_signal: Signal<CharacterBuild>) -> Element {
  return rsx! {
    SectionBar {
      title: "Equipment",
      bar: rsx! {},

    }
  };
}
