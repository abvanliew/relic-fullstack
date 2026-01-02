use std::collections::{HashMap, HashSet};
use std::fmt;

use dioxus::prelude::*;

use super::growth::{CharacterGrowth, TrainingGrowthSignals};
use super::level::LevelSelector;
use super::paths::CharacterPaths;
use super::skills::{CharacterSkills, SkillSelections};

use crate::progression::track::LevelTrack;

use crate::modifiers::prelude::*;
use crate::server::prelude::*;
// use crate::skill::prelude::*;
use crate::path::prelude::*;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
  pub path_filter: PathFilter,
  pub selection_class: PathSelectionClass,
  pub required_weight: i32,
  pub overages: i32,
}

impl Default for Constraint {
  fn default() -> Self {
    Self {
      path_filter: PathFilter::All,
      selection_class: PathSelectionClass::MinorFeatures,
      required_weight: 0,
      overages: 0
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConstraintSet {
  pub required_weight: i32,
  pub selected_weight: i32,
  pub leeway: i32,
}

#[component]
pub fn CharacterProgression() -> Element {
  let current_tab: Signal<BuilderTab> = use_signal( || BuilderTab::Paths );
  
  let level_signal: Signal<i32> = use_signal( || 6 );
  let level = level_signal();
  let mut character_modifiers = LevelTrack::as_of(level);
  
  let selected_paths: Signal<HashSet<String>> = use_signal(|| HashSet::new());
  let extra_features_signal: Signal<i32> = use_signal(|| 0);
  let path_min = character_modifiers.get( &ModifierClass::PathMin );
  let path_max = character_modifiers.get( &ModifierClass::PathMax );
  
  let growth_signals = TrainingGrowthSignals::default();
  
  let path_cache = use_context::<PathCache>();
  let path_options = path_cache.get_sorted_paths(false);
  let PathCache(path_map_cache) = path_cache;
  let SkillCache(skill_map_cache) = use_context::<SkillCache>();
  let growth_ranks_maximum = character_modifiers.get(&ModifierClass::GrowthRanks);
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
  let mut constraints = Vec::<Constraint>::new();
  let mut maximum_weights = 2 * extra_features_signal();
  let mut skill_constraints = HashMap::<String, u64>::new();
  
  for path_id in all_path_ids {
    let Some( path ) = path_map_cache.from_id( &path_id ) else { continue; };
    if let Some( path_selections ) = path.selections {
      for ( class, requirement ) in path_selections {
        let constraint_requirement = class.weight_multiplier() * requirement;
        maximum_weights += constraint_requirement;
        constraints.push( Constraint { 
          path_filter: PathFilter::Single( path.id.to_string() ), 
          selection_class: class,
          required_weight: constraint_requirement,
          ..Constraint::default()
        } );
      }
    }
    let Some( path_skill_ids ) = path.skill_ids else { continue; };
    for skill_id in path_skill_ids {
      let Some( skill ) = skill_map_cache.from_object_id( &skill_id ) else { continue; };
      skill_selection.add_skill( &skill );
      if skill.feature_weight() != 0 { continue; }
      let Some( ref skill_modifiers ) = skill.modifiers else { continue; };
      character_modifiers.append( skill_modifiers );
    }
  }

  let mut constraint_combos: HashMap<u64, ConstraintSet> = HashMap::new();
  let mut selected_weights = 0;

  let selected_skills = skill_selection.to_vec();

  for skill in &selected_skills {
    let mut skill_mask: u64 = 0;
    let mut total_required_weight = 0;
    for ( index, constraint ) in constraints.iter().enumerate() {
      if !skill.is_match( &constraint.path_filter, &constraint.selection_class ) { continue; }
      skill_mask += 1 << index;
      total_required_weight += constraint.required_weight;
    }
    let ranks = *skill_selection.rank_signal.entry( skill.id.to_string() ).or_insert(use_signal(|| 0));
    let weight = ranks() * skill.feature_weight();
    selected_weights += weight;
    if skill_mask == 0 { continue; }
    let constraint_set = constraint_combos.entry( skill_mask ).or_default();
    constraint_set.required_weight = total_required_weight;
    constraint_set.selected_weight += weight;
    skill_constraints.insert( skill.id.to_string(), skill_mask );
    if ranks() == 0 { continue; }
    let Some( ref skill_modifiers ) = skill.modifiers else { continue; };
    skill_modifiers.multiple( ranks() );
    character_modifiers.append( &skill_modifiers );
  }
  
  maximum_weights += 2 * character_modifiers.get( &ModifierClass::Feature );
  let minor_features_requirements = character_modifiers.get( &ModifierClass::MinorFeature );
  if minor_features_requirements > 0 {
    constraints.push( Constraint { required_weight: minor_features_requirements, ..Constraint::default() } );
    maximum_weights += minor_features_requirements;
  }

  for ( index, constraint ) in constraints.iter_mut().enumerate() {
    let constraint_mask: u64 = 1 << index;
    let mut constrained_weight: i32 = 0;
    for ( combo_mask, combo_constraint ) in constraint_combos.iter() {
      if constraint_mask & combo_mask == 0 { continue; }
      constrained_weight += ( 
        combo_constraint.selected_weight 
        - combo_constraint.required_weight 
        + constraint.required_weight )
        .max( 0 );
    }
    constraint.overages = constrained_weight;
  }
  
  let remaining_weight = maximum_weights - selected_weights;
  skill_selection.remaining_weight = remaining_weight;

  for ( combo_mask, combo_constraint ) in constraint_combos.iter_mut() {
    let mut total_overages = 0;
    for ( index, constraint ) in constraints.iter().enumerate() {
      let constraint_mask: u64 = 1 << index;
      if constraint_mask & combo_mask == 0 { continue; }
      total_overages += constraint.overages;
    }
    combo_constraint.leeway = ( combo_constraint.required_weight - total_overages - combo_constraint.selected_weight )
    .min( remaining_weight );
  }
  
  for ( skill_id, mask ) in skill_constraints {
    let Some( constraint_set ) = constraint_combos.get( &mask ) else { continue; };
    skill_selection.leeway.insert( skill_id, constraint_set.leeway );
  }
  
  let has_innate = character_modifiers.contains_key( &ModifierClass::InnateFlow );
  let has_resonance = character_modifiers.contains_key( &ModifierClass::ResonanceFlow );
  let has_magic = character_modifiers.contains_key( &ModifierClass::MagicFlow );
  

  let (x, y) =  skill_selection.to_split_vec();
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
    // div { "{selected_skills:?}" }
    div { "{constraints:?}" }
    div { "{constraint_combos:?}" }
    div { "{skill_selection.rank_signal:?}" }
    for k in x {
      div { "{k.title:?}" }
    }
    for k in y {
      div { "{k.title:?}" }
    }
    match current_tab() {
      BuilderTab::Paths =>rsx! {
        CharacterPaths {
          path_options, path_min, path_max, extra_features_signal, selected_paths
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
