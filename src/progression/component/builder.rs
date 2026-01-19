use std::collections::{HashMap, HashSet};
use std::fmt;

use dioxus::prelude::*;

use super::growth::{CharacterGrowth, TrainingGrowthSignals};
use super::level::LevelSelector;
use super::paths::CharacterPaths;
use super::skills::{CharacterSkills, SkillSelections};
use super::ranks::{CharacterRanks, RankSelections};

use crate::progression::component::ranks::StaticRanks;
use crate::progression::track::{LevelTrack, GrowthTrack};
use crate::progression::training::TrainingClass;

use crate::modifiers::prelude::*;
use crate::server::prelude::*;
use crate::path::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BuilderTab {
  Paths,
  Skills,
  Attributes,
}

impl fmt::Display for BuilderTab {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BuilderTab::Paths => "Paths",
        BuilderTab::Skills => "Skills",
        BuilderTab::Attributes => "Attributes",
      }
    )
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConstraintSet {
  pub required_weight: i32,
  pub selected_weight: i32,
  pub overage_total: i32,
  pub leeway: i32,
  pub filters: Vec<SelectionFilter>,
}

#[component]
pub fn CharacterProgression() -> Element {
  let current_tab: Signal<BuilderTab> = use_signal( || BuilderTab::Attributes );
  
  let level_signal: Signal<i32> = use_signal( || 6 );
  let level = level_signal();
  let mut character_modifiers = LevelTrack::as_of( level );
  
  let selected_paths: Signal<HashSet<String>> = use_signal( || HashSet::new() );
  let extra_features_signal: Signal<i32> = use_signal( || 0 );
  let path_min = character_modifiers.get( &ModifierClass::InitiatePathMin );
  let path_max = character_modifiers.get( &ModifierClass::InitiatePathMax );
  
  let growth_signals = TrainingGrowthSignals::default();
  
  let path_cache = use_context::<PathCache>();
  let path_options = path_cache.get_sorted_paths(false);
  let PathCache( ref path_map_cache ) = path_cache;
  let SkillCache( ref skill_map_cache ) = use_context::<SkillCache>();
  let growth_ranks_maximum = character_modifiers.get( &ModifierClass::GrowthRanks );
  let growth_ranks_selected = (growth_signals.adept)()
    + (growth_signals.endurance)()
    + (growth_signals.expert)()
    + (growth_signals.innate)()
    + (growth_signals.resonance)()
    + (growth_signals.magic)();
  let growth_ranks_remaining = growth_ranks_maximum - growth_ranks_selected;
  character_modifiers.append( &GrowthTrack::class_at(
    &TrainingClass::Adept, (growth_signals.adept)()
  ) );
  character_modifiers.append( &GrowthTrack::class_at(
    &TrainingClass::Endurance, (growth_signals.endurance)()
  ) );
  character_modifiers.append( &GrowthTrack::class_at(
    &TrainingClass::Expert, (growth_signals.expert)()
  ) );
  character_modifiers.append( &GrowthTrack::class_at(
    &TrainingClass::Innate, (growth_signals.innate)()
  ) );
  character_modifiers.append( &GrowthTrack::class_at(
    &TrainingClass::Resonance, (growth_signals.resonance)()
  ) );
  character_modifiers.append( &GrowthTrack::class_at(
    &TrainingClass::Magic, (growth_signals.magic)()
  ) );
  
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
  let mut constraint_combos = HashMap::<u64, ConstraintSet>::new();
  let mut skill_constraints = HashMap::<String, u64>::new();
  let mut weight_budget = 0;
  let mut selected_weights = 0;
  
  for path_id in all_path_ids {
    let Some( path ) = path_map_cache.from_id( &path_id ) else { continue; };
    let ( mut path_constraints, additional_budget ) = path.selection_constraints();
    weight_budget += additional_budget;
    constraints.append( &mut path_constraints );
    let Some( path_skill_ids ) = path.skill_ids else { continue; };
    for skill_id in path_skill_ids {
      let Some( skill ) = skill_map_cache.from_object_id( &skill_id ) else { continue; };
      skill_selection.add_skill( &skill );
      if skill.weight() != 0 { continue; }
      let Some( ref skill_modifiers ) = skill.modifiers else { continue; };
      character_modifiers.append( skill_modifiers );
    }
  }
  
  let selectable_skills = skill_selection.to_vec();
  let rank_map = (skill_selection.rank_signal)();
  let feature_count = extra_features_signal() + character_modifiers.get( &ModifierClass::Feature );
  if feature_count > 0 {
    let feature_weight = 2 * feature_count;
    constraints.push( Constraint::feature( feature_weight ) );
    weight_budget += feature_weight;
  }
  let minor_feature_weights = character_modifiers.get( &ModifierClass::MinorFeature );
  if minor_feature_weights > 0 {
    constraints.push( Constraint::minor_feature( minor_feature_weights ) );
    weight_budget += minor_feature_weights;
  }
  
  for ( index, constraint ) in constraints.iter().enumerate() {
    let mask = 1 << index;
    constraint_combos.insert(
      mask,
      ConstraintSet {
        required_weight: constraint.required_weight,
        filters: vec![ constraint.filter.clone() ],
        ..Default::default()
      }
    );
  }

  for skill in &selectable_skills {
    let mut skill_mask: u64 = 0;
    let mut required_weight = 0;
    let mut filters = Vec::<SelectionFilter>::new();
    for ( index, constraint ) in constraints.iter().enumerate() {
      if !skill.is_match( &constraint.filter ) { continue; }
      skill_mask += 1 << index;
      required_weight += constraint.required_weight;
      filters.push( constraint.filter.clone() );
    }
    let ranks = *rank_map.get( &skill.id.to_string() ).unwrap_or( &0 );
    let weight = ranks * skill.weight();
    selected_weights += weight;
    if skill_mask != 0 {
      let constraint_set = constraint_combos.entry( skill_mask ).or_insert(
        ConstraintSet {
          required_weight,
          filters,
          ..Default::default()
        }
      );
      constraint_set.selected_weight += weight;
      skill_constraints.insert( skill.id.to_string(), skill_mask );
    }
    if ranks > 0 {
      let Some( ref skill_modifiers ) = skill.modifiers else { continue; };
      let ranked_modifier = skill_modifiers.multiple( ranks );
      character_modifiers.append( &ranked_modifier );
    }
  }

  for ( index, constraint ) in constraints.iter_mut().enumerate() {
    let constraint_mask: u64 = 1 << index;
    let mut constrained_weight: i32 = 0;
    for ( combo_mask, combo_constraint ) in constraint_combos.iter_mut() {
      if constraint_mask & combo_mask == 0 { continue; }
      let constraint_net = ( 
        combo_constraint.selected_weight 
        - combo_constraint.required_weight 
        + constraint.required_weight
      )
      .max( 0 );
      constrained_weight += constraint_net;
      combo_constraint.overage_total += constraint_net;
    }
    constraint.overages = constrained_weight;
  }
  
  let remaining_weight = weight_budget - selected_weights;
  skill_selection.remaining_weight = remaining_weight;
  
  for ( combo_mask, combo_constraint ) in constraint_combos.iter_mut() {
    let mut total_constraint_overages = 0;
    for ( index, constraint ) in constraints.iter().enumerate() {
      let constraint_mask: u64 = 1 << index;
      if constraint_mask & combo_mask == 0 { continue; }
      total_constraint_overages += constraint.overages;
    }
    combo_constraint.leeway = ( 
      combo_constraint.required_weight 
      - combo_constraint.selected_weight
      - total_constraint_overages 
      + combo_constraint.overage_total
    )
    .min( remaining_weight );
  }
  
  for ( skill_id, mask ) in skill_constraints {
    let Some( constraint_set ) = constraint_combos.get( &mask ) else { continue; };
    skill_selection.leeway.insert( skill_id, constraint_set.leeway );
  }
  
  let mut core_constraints: Vec<String> = Vec::new();
  for ( index, constraint ) in constraints.iter().enumerate() {
    let mask = 1 << index;
    let Some( combo_constraint ) = constraint_combos.get( &mask ) else { continue; };
    let filter = &constraint.filter;
    let filter_name = match &filter.path_filter {
      PathFilter::All => "any Path".into(),
      PathFilter::Single( path_id ) => {
        match path_map_cache.from_id( &path_id ) {
          Some( path ) => path.title,
          None => "undefined path".into(),
        }
      },
    };
    let max_rank = constraint.required_weight / filter.skill_filter.weight();
    let ranks = max_rank - combo_constraint.leeway / filter.skill_filter.weight();
    let skill_name = filter.skill_filter.to_string();
    core_constraints.push( format!( "{}/{} {} from {}", ranks, max_rank, skill_name, filter_name ) );
  }
  
  let innate_flow = character_modifiers.get( &ModifierClass::InnateFlow );
  let has_innate = innate_flow > 0;
  let has_resonance = character_modifiers.contains_key( &ModifierClass::ResonanceFlow );
  let has_magic = character_modifiers.contains_key( &ModifierClass::MagicFlow );

  let rank_selections = RankSelections::default();
  let min_rank = 0;
  let max_rank = character_modifiers.get( &ModifierClass::RankMax );
  let attribute_ranks = character_modifiers.get( &ModifierClass::AttributeRank );
  let capability_max_ranks = character_modifiers.get( &ModifierClass::CapabilityMaxRank );
  let defense_max_ranks = character_modifiers.get( &ModifierClass::DefenseMaxRank );
  let mut innate_pools: Vec<(ModifierClass,i32)> = Vec::new();
  for pool_class in vec![
    &ModifierClass::AnointmentPool,
    &ModifierClass::AnimalismPool,
    &ModifierClass::SanguinePool,
    &ModifierClass::RagePool,
  ] {
    let pool = character_modifiers.get( pool_class );
    if pool > 0 {
      innate_pools.push( ( pool_class.clone(), pool ) );
    }
  }
  let innate_ranks = character_modifiers.get( &ModifierClass::InnatePool );
  let innate_all_ranks = character_modifiers.get( &ModifierClass::InnatePoolAll );
  let static_ranks = StaticRanks {
    hp: character_modifiers.get( &ModifierClass::HP ),
    constituion: character_modifiers.get( &ModifierClass::Constituion ),
    speed: character_modifiers.get( &ModifierClass::WalkingSpeed ),
    dash: character_modifiers.get( &ModifierClass::DashSpeed ),
  };
  rsx! {
    div {
      class: "row",
      LevelSelector { level_signal }
      TabSelector { tab: BuilderTab::Paths, current_tab }
      TabSelector { tab: BuilderTab::Skills, current_tab }
      TabSelector { tab: BuilderTab::Attributes, current_tab }
    }
    match current_tab() {
      BuilderTab::Paths => rsx! {
        CharacterPaths {
          path_options, path_min, path_max, extra_features_signal, selected_paths
        }
      },
      BuilderTab::Skills => rsx! {
        CharacterSkills { skill_selection, core_constraints }
      },
      BuilderTab::Attributes => rsx! {
        CharacterGrowth {
          growth_ranks_remaining, has_innate, has_resonance, has_magic, level, growth_signals
        }
        CharacterRanks {
          rank_selections, min_rank, max_rank, attribute_ranks,
          capability_max_ranks,
          defense_max_ranks,
          innate_flow,
          innate_ranks,
          innate_all_ranks,
          innate_pools,
          static_ranks,
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
