use std::cmp::min;
use std::collections::{HashSet};
use std::fmt;

use dioxus::prelude::*;

use super::level::LevelSelector;
use crate::path::Path;
use crate::progression::training::TrainingClass;
use crate::server::prelude::*;
use super::paths::CharacterPaths;

#[derive(Debug, Clone, PartialEq)]
pub enum BuilderTab {
  Paths,
  Growth,
}

impl fmt::Display for BuilderTab {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!( f, "{}",
      match self {
        BuilderTab::Paths => "Paths",
        BuilderTab::Growth => "Growth",
      }
    )
  }
}

#[component]
pub fn CharacterProgression() -> Element {
  let path_cache = use_context::<PathCache>();
  let paths = path_cache.get_sorted_paths(false);

  let level_signal: Signal<u32> = use_signal(|| 1);
  let current_tab: Signal<BuilderTab> = use_signal(|| BuilderTab::Paths);
  let path_feature_count_signal: Signal<u32> = use_signal(|| 0);
  let selected_paths: Signal<HashSet<String>> = use_signal(|| HashSet::new());
  let path_min = 1;
  let path_max = min(4, level_signal() + 1);
  rsx! {
    div {
      class: "row",
      LevelSelector { level_signal }
      TabSelector { tab: BuilderTab::Paths, current_tab }
      TabSelector { tab: BuilderTab::Growth, current_tab }
    }
    match current_tab() {
      BuilderTab::Paths =>rsx! {
        CharacterPaths { paths, path_min, path_max, path_feature_count_signal, selected_paths }
      },
      BuilderTab::Growth =>rsx! {
        // CharacterGrowth {}
      },
    }
  }
}

#[component]
pub fn TabSelector( tab: BuilderTab, current_tab: Signal<BuilderTab> ) -> Element {
  let selected = current_tab().eq(&tab);
  return rsx! {
    div {
      onclick: move |_| { current_tab.set(tab.clone()); },
      class: if selected { "highlight" } else { "" },
      "{tab}"
    }
  }
}

#[component]
pub fn CharacterGrowth() -> Element {
  let level:u32 = 1;
  rsx! {
    GrowthSelector { training: TrainingClass::Adept, level }
    GrowthSelector { training: TrainingClass::Endurance, level }
    GrowthSelector { training: TrainingClass::Expert, level }
    GrowthSelector { training: TrainingClass::Innate, level }
    GrowthSelector { training: TrainingClass::Resonance, level }
    GrowthSelector { training: TrainingClass::Magic, level }
  }
}

#[component]
pub fn GrowthSelector(
  training: TrainingClass,
  level: u32,
) -> Element {
  rsx! {
  }
}


#[derive(PartialEq, Props, Clone)]
struct SkillSelectorProps {
  id: String,
  path_id: String,
}

#[derive(PartialEq, Props, Clone)]
struct HashButtonProps {
  name: String,
  id: String,
  class: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionState {
  Unfinished,
  Finished,
  Invalid,
}
