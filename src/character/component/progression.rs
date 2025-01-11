use std::{cmp, collections::HashSet};
use dioxus::prelude::*;

use crate::character::{attribute::{AttributeSignal, AttributeType::{self, Capability, Defense}}, training::{self, main_training_growth, TrainingClass, TrainingGrowth, TrainingPanel, TrainingSignal}};
use crate::skill::{component::SkillTile, Skill};
use crate::path::components::PathTile;

use crate::path::Path;
use crate::character::progression::CharacterGrowth;
use crate::character::prelude::Capability::{Manipulation,Physique,Spirit,Warfare};
use crate::character::prelude::Defense::{Dodge,Fortitude,Insight,Resolve,Tenacity};

#[derive(Debug, Clone)]
struct PathContext {
  pub ids: Signal<HashSet<String>>
}

#[derive(Debug, Clone)]
struct SkillContext {
  pub ids: Signal<HashSet<String>>
}

#[component]
pub fn CharacterProgression( paths: Vec<Path> ) -> Element {
  // let mut path_id = use_signal( || "".to_string() );
  let full_features = vec![
    FeatureTest { title: "HP +1".into(), summary: None }
  ];
  let half_features = vec![
    FeatureTest { title: "Expertise +1 rank".into(), summary: None }
  ];
  let mut level = use_signal( || 1 );
  let mut attrib_signal = AttributeSignal::use_context_provider();
  let training_signal = TrainingSignal::use_context_provider();
  let stats = CharacterGrowth::default().stats( level() );
  use_effect( move || attrib_signal.cap( stats.max_ranks ) );
  let path_ids_selected: Signal<HashSet<String>> = use_signal( || HashSet::new() );
  use_context_provider( || PathContext { ids: path_ids_selected } );
  let skill_ids_selected: Signal<HashSet<String>> = use_signal( || HashSet::new() );
  use_context_provider( || SkillContext { ids: skill_ids_selected } );
  let mut skills_full: Signal<Vec<Skill>> = use_signal( || Vec::new() );
  let path_copy = paths.clone();
  use_effect( move || {
    let mut path_skills: Vec<Skill> = Vec::new();
    for id in path_ids_selected() {
      let Some( path ) = find_path( path_copy.clone(), id.clone() ) else { continue; };
      path_skills.extend( path.skills_spells_full() )
    }
    path_skills.dedup_by_key(|p| p.id.to_string() );
    skills_full.set( path_skills );
  } );
  let path_count: i32 = path_ids_selected().len().try_into().unwrap_or( i32::MAX );
  let full_count: i32 = skill_ids_selected().len().try_into().unwrap_or( i32::MAX );
  let remaining = stats.attributes - attrib_signal.sum();
  let path_min = stats.iniatite.path_min;
  let path_max = stats.iniatite.path_max;
  let full_min = stats.iniatite.features - path_max + path_min;
  let full_max = stats.iniatite.features;
  let half_min = stats.iniatite.half_features;
  let half_max = half_min + 2 * ( full_max );
  let path_enabled = path_count < path_max;
  rsx! {
    div {
      class: "grid dim-sidebar",
      div {
        class: "uv-main",
        div {
          class: "grid dim-keywords",
          div { class: "uv-title", "Level" }
          div { class: "uv-after-title", select {
            onchange: move |event| { level.set( event.value().parse().unwrap_or( 1 ) ); },
            for lvl in 1..=12 { option { value: lvl, label: lvl, selected: level() == lvl, } }
          } }
        }
        div {
          class: "tiles",
          div {
            class: "uv-full small-text dotted-underline spacer",
            "Paths: {path_min} - {path_max} [{path_count}]"
          }
          for path in paths {
            PathSelector { path, enabled: path_enabled }
          }
          div {
            class: "uv-full small-text dotted-underline spacer",
            "Full Features: {full_min} - {full_max} [{full_count}]"
          }
          for skill in skills_full() {
            SkillSelector { skill }
          }
          for feature in full_features {
            FeatureSelector { feature }
          }
          div {
            class: "uv-full small-text dotted-underline spacer",
            "Half Features: {half_min} - {half_max}"
          }
          for feature in half_features {
            FeatureSelector { feature }
          }
        }
        TrainingRanks { level: level() }
      }
      div {
        class: "uv-sidebar",
        div {
          class: "grid dim-keywords",
          AttributeDistribution { remaining, min: 0, max: stats.max_ranks, total: stats.attributes }
          div { class: "uv-full spacer" }
          div { class: "uv-title highlight","HP" }
          div { class: "uv-after-title", "{stats.hp}" }
        }
      }
    }
  }
}

fn find_path( paths: Vec<Path>, id: String ) -> Option<Path> {
  for path in paths {
    if path.id.to_string().eq( &id ) { return Some( path.clone() ) }
  }
  return None;
}

#[component]
pub fn TrainingRanks( level: i32 ) -> Element {
  let growth = main_training_growth();
  let training = use_context::<TrainingSignal>();
  let sum = training.sum();
  let ranks_shown = 6 *( ( level + 5 ) / 6 );
  rsx!(
    div {
      class: "grid dim-training",
      div { class: "uv-full dotted-underline small-text spacer", "Training Ranks: {level} [{sum}]" }
      div { class: "uv-expert", "Expert" }
      div { class: "uv-adept", "Adept" }
      div { class: "uv-endurance", "Endurance" }
      div { class: "uv-resonance", "Resonance" }
      div { class: "uv-innate", "Innate" }
      div { class: "uv-magic", "Magic" }
      for i in 1..=ranks_shown {
        TrainingSelector {
          class: TrainingClass::Expert,
          rank: i.into(),
          growth: growth.clone(),
          level,
          available: true,
        }
        TrainingSelector {
          class: TrainingClass::Adept,
          rank: i.into(),
          growth: growth.clone(),
          level,
          available: true,
        }
        TrainingSelector {
          class: TrainingClass::Endurance,
          rank: i.into(),
          growth: growth.clone(),
          level,
          available: true,
        }
        TrainingSelector {
          class: TrainingClass::Resonance,
          rank: i.into(),
          growth: growth.clone(),
          level,
          available: false,
        }
        TrainingSelector {
          class: TrainingClass::Innate,
          rank: i.into(),
          growth: growth.clone(),
          level,
          available: false,
        }
        TrainingSelector {
          class: TrainingClass::Magic,
          rank: i.into(),
          growth: growth.clone(),
          level,
          available: false,
        }
      }
      if ranks_shown < 18 {
        div { class: "uv-expert", "..." }
        div { class: "uv-adept", "..." }
        div { class: "uv-endurance", "..." }
        div { class: "uv-resonance", "..." }
        div { class: "uv-innate", "..." }
        div { class: "uv-magic", "..." }
      }
    }
  )
}

#[component]
pub fn TrainingSelector( class: TrainingClass, rank: i32, growth: TrainingGrowth, level: i32, available: bool ) -> Element {
  let mut training = use_context::<TrainingSignal>();
  let selected_rank = training.get( &class );
  let min: i32 = 0;
  let max: i32 = level - training.sum() + selected_rank;
  let top = selected_rank == rank;
  let uv = match class {
    TrainingClass::Expert => "uv-expert",
    TrainingClass::Adept => "uv-adept",
    TrainingClass::Endurance => "uv-endurance",
    TrainingClass::Resonance => "uv-resonance",
    TrainingClass::Innate => "uv-innate",
    TrainingClass::Magic => "uv-magic",
  };
  let selected = rank <= selected_rank;
  let enabled = available && rank <= max;
  rsx!(
    div {
      class: match ( selected, enabled ) {
        ( true, true ) => format!( "{uv} selected padded" ),
        ( true, false ) => format!( "{uv} selected disabled padded" ),
        ( false, true ) => format!( "{uv} unselected padded" ),
        ( false, false ) => format!( "{uv} unselected disabled padded" ),
      },
      onclick: move |_| {
        let mut new_value = match ( selected, enabled, top ) {
          ( true, true, true ) => rank - 1,
          ( _, true, _ ) => rank,
          ( true, false, _ ) => max,
          _ => selected_rank,
        };
        if new_value < min { new_value = min; }
        training.set( &class, new_value )
      },
      TrainingPanel { growth, class: class.clone(), rank }
    }
  )
}

#[component]
pub fn AttributeDistribution( remaining: i32, min: i32, max: i32, total: i32 ) -> Element {
  rsx!(
    div { class: "uv-title", "Remaining" }
    div { class: "uv-after-title", "{remaining}" }
    div { class: "uv-title highlight spacer", "Capabilities" }
    AttributeSelector { attribute: Capability(Physique), min, max, total }
    AttributeSelector { attribute: Capability(Warfare), min, max, total }
    AttributeSelector { attribute: Capability(Spirit), min, max, total }
    AttributeSelector { attribute: Capability(Manipulation), min, max, total }
    div { class: "uv-title highlight spacer", "Defenses" }
    AttributeSelector { attribute: Defense(Tenacity), min, max, total }
    AttributeSelector { attribute: Defense(Fortitude), min, max, total }
    AttributeSelector { attribute: Defense(Resolve), min, max, total }
    AttributeSelector { attribute: Defense(Insight), min, max, total }
    AttributeSelector { attribute: Defense(Dodge), min, max, total }
  )
}

#[component]
pub fn AttributeSelector( attribute: AttributeType, min: i32, max: i32, total: i32 ) -> Element {
  let attributes = use_context::<AttributeSignal>();
  let mut signal = attributes.get( &attribute );
  let cur_max = cmp::min( max, total - attributes.sum() + signal() );
  rsx!(
    div { class: "uv-title", "{attribute}" }
    div {
      class: "uv-after-title",
      input {
        type: "number",
        min: min,
        max: cur_max,
        value: signal(),
        onchange: move |event| {
          let mut value = event.value().parse().unwrap_or( min );
          if value > cur_max { value = cur_max; }
          if value < min { value = min; }
          signal.set( value );
        }
      }
    }
  )
}

#[component]
pub fn PathSelector( path: Path, enabled: bool ) -> Element {
  let paths = use_context::<PathContext>();
  let id = path.id.to_string();
  let mut signal = paths.ids;
  let mut selected = use_signal( || false );
  rsx!(
    div {
      class: match ( selected(), enabled ) {
        ( true, true ) => "tile selected",
        ( true, false ) => "tile selected disabled",
        ( false, true ) => "tile unselected",
        ( false, false ) => "tile hidden",
      },
      onclick: move |_| {
        if !enabled && !selected() { return; }
        let mut updated_paths = signal().clone();
        match selected() {
          true => updated_paths.remove( &id ),
          false => updated_paths.insert( id.to_owned() ),
        };
        signal.set( updated_paths );
        selected.set( !selected() );
      },
      PathTile { path }
    }
  )
}

#[component]
pub fn SkillSelector( skill: Skill ) -> Element {
  let skills = use_context::<SkillContext>();
  let id = skill.id.to_string();
  let mut skill_list = skills.ids;
  let mut selected = use_signal( || false );
  rsx!(
    div {
      class: match selected() {
        true => "tile selected",
        false => "tile unselected",
        // ( true, true ) => "tile selected",
        // ( true, false ) => "tile selected disabled",
        // ( false, true ) => "tile unselected",
        // ( false, false ) => "tile hidden",
      },
      onclick: move |_| {
        let mut value = skill_list().clone();
        match selected() {
          true => value.remove( &id ),
          false => value.insert( id.to_owned() ),
        };
        skill_list.set( value );
        selected.set( !selected() );
      },
      SkillTile { skill }
    }
  )
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureTest {
  pub title: String,
  pub summary: Option<String>,
}

#[component]
pub fn FeatureSelector( feature: FeatureTest ) -> Element {
  let min = 0;
  let max = 5;
  let mut count: Signal<i32> = use_signal( || min );
  rsx!(
    div {
      class: if count() > 0 { "tile selected" } else { "tile unselected" },
      onclick: move |_| { if count() < max { count.set( count() + 1 ); } },
      div { "{feature.title}" }
      div {
        input {
          type: "number",
          min: min,
          max: max,
          value: count(),
          onclick: move |event| { event.stop_propagation(); },
          onchange: move |event| {
            let mut value = event.value().parse().unwrap_or( min );
            if value > max { value = max; }
            if value < min { value = min; }
            count.set( value );
          }
        }
      }
      div {
        onclick: move |event| {
          event.stop_propagation();
          count.set( min );
        },
        "Clear"
      }
    }
  )
}



// fn selection_stats( selections: &Vec<Selection> ) -> SelectionStats {
//   let mut stats = SelectionStats::default();
//   for selection in selections {
//     match ( &selection.class, selection.id ) {
//       ( Some( SelectionClass::Path ), Some( _ ) ) => stats.paths += 1,
//       ( Some( SelectionClass::FullFeature ), Some( _ ) ) => stats.features += 1,
//       ( Some( SelectionClass::HalfFeature ), Some( _ ) ) => stats.half_features += 1,
//       _ => ()
//     }
//   }
//   return stats;
// }

// #[derive(Default)]
// struct SelectionStats {
//   pub paths: i32,
//   pub features: i32,
//   pub half_features: i32,
// }