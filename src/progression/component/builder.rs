use std::collections::{HashMap, HashSet};
use std::fmt;

use dioxus::prelude::*;

use super::growth::{CharacterGrowth, TrainingGrowthSignals};
use super::level::LevelSelector;
use super::paths::CharacterPaths;
use super::skills::{CharacterSkills, SkillSelections};

use crate::progression::component::SelectionState;
use crate::progression::track::LevelTrack;

use crate::modifiers::prelude::*;
use crate::server::prelude::*;
use crate::skill::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BuilderTab {
  Paths,
  Growth,
  Skills,
}

impl fmt::Display for BuilderTab {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BuilderTab::Paths => "Paths",
        BuilderTab::Growth => "Growth",
        BuilderTab::Skills => "Skills",
      }
    )
  }
}

#[component]
pub fn CharacterProgression() -> Element {
  let current_tab: Signal<BuilderTab> = use_signal(|| BuilderTab::Paths);

  let level_signal: Signal<i32> = use_signal(|| 6);
  let level = level_signal();
  let level_track_modifiers = LevelTrack::as_of(level);

  let selected_paths: Signal<HashSet<String>> = use_signal(|| HashSet::new());
  let path_feature_count_signal: Signal<i32> = use_signal(|| 0);
  let path_min = level_track_modifiers.get(&ModifierClass::PathMin);
  let path_max = level_track_modifiers.get(&ModifierClass::PathMax);

  let growth_signals = TrainingGrowthSignals::default();

  let path_cache = use_context::<PathCache>();
  let path_options = path_cache.get_sorted_paths(false);
  let PathCache(path_map_cache) = path_cache;
  let SkillCache(skill_map_cache) = use_context::<SkillCache>();
  let growth_ranks_maximum = level_track_modifiers.get(&ModifierClass::GrowthRanks);
  let growth_ranks_selected = (growth_signals.adept)()
    + (growth_signals.endurance)()
    + (growth_signals.expert)()
    + (growth_signals.innate)()
    + (growth_signals.resonance)()
    + (growth_signals.magic)();
  let growth_ranks_remaining = growth_ranks_maximum - growth_ranks_selected;



  let mut all_path_ids = path_map_cache.into_vec().iter()
  .filter_map( |path|
    match path.inherient {
      Some( true ) => Some( path.id.to_string() ),
      _ => None
    }
  )
  .collect::<Vec<String>>();
  all_path_ids.extend( selected_paths().clone() );

  let mut skill_selection = SkillSelections::default();
  let mut modifiers = ModifierSet::default();
  let mut path_modifiers: HashMap<String, ModifierSet> = HashMap::new();
  for path_id in all_path_ids {
    let Some(path) = path_map_cache.from_id(&path_id) else {
      continue;
    };
    let Some(path_skill_ids) = path.skill_ids else {
      continue;
    };
    for skill_id in path_skill_ids {
      let Some( skill ) = skill_map_cache.from_object_id( &skill_id ) else {
        continue;
      };
      skill_selection.add_skill( &skill );
      match skill.training_cost {
        TrainingCost::Inherient | TrainingCost::Keystone => {
          let Some(ref skill_modifiers) = skill.modifiers else {
            continue;
          };
          modifiers.append( skill_modifiers );
        },
        _ => ()
      }
    }
    let path_mods = modifiers.split_path_modifiers();
    if path_mods.len() == 0 { continue; }
    path_modifiers.insert(path_id, path_mods);
  }

  let has_innate = modifiers.contains_key(&ModifierClass::InnateFlow);
  let has_resonance = modifiers.contains_key(&ModifierClass::ResonanceFlow);
  let has_magic = modifiers.contains_key(&ModifierClass::MagicFlow);

  // let growth_selection_state = match (
  //   !has_innate && (growth_signals.innate)() > 0,
  //   !has_resonance && (growth_signals.resonance)() > 0,
  //   !has_magic && (growth_signals.magic)() > 0,
  //   growth_ranks_remaining < 0,
  //   growth_ranks_remaining == 0,
  // ) {
  //   ( true, _, _, _, _ ) |
  //   ( _, true, _, _, _ ) |
  //   ( _, _, true, _, _ ) |
  //   ( _, _, _, true, _ ) => SelectionState::Invalid,
  //   ( _, _, _, _, true ) => SelectionState::Finished,
  //   _ => SelectionState::Unfinished
  // };


  rsx! {
    div {
      class: "row",
      LevelSelector { level_signal }
      TabSelector { tab: BuilderTab::Paths, current_tab }
      TabSelector { tab: BuilderTab::Growth, current_tab }
      TabSelector { tab: BuilderTab::Skills, current_tab }
    }
    match current_tab() {
      BuilderTab::Paths =>rsx! {
        CharacterPaths {
          path_options, path_min, path_max, path_feature_count_signal, selected_paths
        }
      },
      BuilderTab::Growth =>rsx! {
        CharacterGrowth {
          growth_ranks_remaining, has_innate, has_resonance, has_magic, level, growth_signals
        }
      },
      BuilderTab::Skills =>rsx! {
        CharacterSkills { skill_selection }
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
