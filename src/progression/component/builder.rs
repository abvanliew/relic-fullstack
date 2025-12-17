use std::collections::HashSet;
use std::fmt;

use dioxus::prelude::*;

use super::growth::{CharacterGrowth, TrainingGrowthSignals};
use super::level::LevelSelector;
use super::paths::CharacterPaths;

use crate::modifiers::prelude::*;
use crate::progression::track::LevelTrack;
use crate::server::prelude::*;
use crate::skill::prelude::TrainingCost;

#[derive(Debug, Clone, PartialEq)]
pub enum BuilderTab {
  Paths,
  Growth,
}

impl fmt::Display for BuilderTab {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BuilderTab::Paths => "Paths",
        BuilderTab::Growth => "Growth",
      }
    )
  }
}

#[component]
pub fn CharacterProgression() -> Element {
  let current_tab: Signal<BuilderTab> = use_signal(|| BuilderTab::Growth);

  let level_signal: Signal<u32> = use_signal(|| 6);
  let level = level_signal();
  let level_track_modifiers = LevelTrack::as_of(level);

  let selected_paths: Signal<HashSet<String>> = use_signal(|| HashSet::new());
  let path_feature_count_signal: Signal<u32> = use_signal(|| 0);
  let path_min = level_track_modifiers.get(&ModifierClass::PathMin);
  let path_max = level_track_modifiers.get(&ModifierClass::PathMax);

  let growth_signals = TrainingGrowthSignals::default();

  let path_cache = use_context::<PathCache>();
  let paths = path_cache.get_sorted_paths(false);
  let PathCache(path_map_cache) = path_cache;
  let SkillCache(skill_map_cache) = use_context::<SkillCache>();
  let growth_ranks_maximum = level_track_modifiers.get(&ModifierClass::GrowthRanks);
  let growth_ranks_selected = (growth_signals.adept)()
    + (growth_signals.endurance)()
    + (growth_signals.expert)()
    + (growth_signals.innate)()
    + (growth_signals.resonance)()
    + (growth_signals.magic)();
  let growth_ranks_remaining = 
  if growth_ranks_selected > growth_ranks_maximum {
    0
  } else {
    growth_ranks_maximum - growth_ranks_selected
  };

  let mut modifiers = ModifierSet::default();
  for path_id in selected_paths() {
    let Some(path) = path_map_cache.from_id(&path_id) else {
      continue;
    };
    let Some(path_skill_ids) = path.skill_ids else {
      continue;
    };
    let skills = skill_map_cache
      .from_object_ids(&path_skill_ids)
      .into_iter()
      .filter(|skill| match skill.training_cost {
        TrainingCost::Inherient | TrainingCost::Keystone => true,
        _ => false,
      })
      .collect();
    let path_modifiers = modifiers_from_skills(&skills);
    modifiers.append(&path_modifiers);
  }
  let has_innate = modifiers.contains_key(&ModifierClass::InnateFlow);
  let has_resonance = modifiers.contains_key(&ModifierClass::ResonanceFlow);
  let has_magic = modifiers.contains_key(&ModifierClass::MagicFlow);
  rsx! {
    div {
      class: "row",
      LevelSelector { level_signal }
      TabSelector { tab: BuilderTab::Paths, current_tab }
      TabSelector { tab: BuilderTab::Growth, current_tab }
    }
    match current_tab() {
      BuilderTab::Paths =>rsx! {
        CharacterPaths {
          paths, path_min, path_max, path_feature_count_signal, selected_paths
        }
      },
      BuilderTab::Growth =>rsx! {
        CharacterGrowth {
          growth_ranks_remaining, has_innate, has_resonance, has_magic, level, growth_signals
        }
      },
    }
  }
}

#[component]
pub fn TabSelector(tab: BuilderTab, current_tab: Signal<BuilderTab>) -> Element {
  let selected = current_tab().eq(&tab);
  return rsx! {
    div {
      onclick: move |_| { current_tab.set(tab.clone()); },
      class: if selected { "highlight" } else { "" },
      "{tab}"
    }
  };
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
