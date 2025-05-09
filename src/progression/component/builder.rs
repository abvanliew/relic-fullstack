use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
// use std::cmp::max;

use dioxus::prelude::*;

use crate::{path::Path, progression::fixed::MIN_LEVEL};
// use crate::progression::component::effects::{level_change_effect, path_change_effect};
// use crate::progression::component::path::PathSelections;
// use crate::progression::component::TrainingRanks;
// use crate::progression::fixed::{MAX_LEVEL, MIN_LEVEL};
// use crate::progression::growth::LevelStats;
// use crate::progression::prelude::TrainingClass;
// use crate::progression::qualifier::{FlowPoolQualifier, PathQualifier};
use crate::progression::track::TrackContext;
// use crate::rule::prelude::Tier;
use super::level::LevelSelector;
// use super::qualifiers::BuildQualifiers;
// use super::TrainingSignal;

#[component]
pub fn CharacterProgression( paths: Vec<Path> ) -> Element {
  TrackContext::use_context_provider();
  let build_context = BuildContext::use_context_provider();
  let paths = build_context.paths;
  let path_options = build_context.path_options;
  let path_count = paths.read().len();
  rsx! {
    div {
      "{path_options:?}"
    }
    LevelSelector {}
    div {
      "Path Count: {path_count}"
    }
    div {
      class: "row-wrap",
      HashButton { name: "A" }
      HashButton { name: "B" }
      HashButton { name: "C" }
      HashButton { name: "D" }
      HashButton { name: "E" }
      HashButton { name: "F" }
    }
    for path in paths().iter() {
      PathOptionDropdown {
        path_id: path,
        option_id: "Bonus",
        selection_text: "Pick a bonus",
        options: vec![
          ( "HP".to_string(), "Health +1".to_string() ),
          ( "RANK".to_string(), "Rank +1".to_string() ),
          ( "POOL".to_string(), "Pool +1".to_string() ),
        ]
      }
    }
    div {
      "Training"
      div {
        class: "row",
        div {
          class: "column",
          div { "Adept" }
          for i in 0..6 {
            div { class: "tile unselected", "Rank {i}" }
          }
        }
        div {
          class: "column",
          div { "Endurance" }
          for i in 0..6 {
            div { class: "tile unselected", "Rank {i}" }
          }
        }
        div {
          class: "column",
          div { "Expert" }
          for i in 0..6 {
            div { class: "tile unselected", "Rank {i}" }
          }
        }
        div {
          class: "column",
          div { "Innate" }
          for i in 0..6 {
            div { class: "tile unselected", "Rank {i}" }
          }
        }
        div {
          class: "column",
          div { "Resonance" }
          for i in 0..6 {
            div { class: "tile unselected", "Rank {i}" }
          }
        }
        div {
          class: "column",
          div { "Magic" }
          for i in 0..6 {
            div { class: "tile unselected", "Rank {i}" }
          }
        }
      }
    }
  }
}

#[derive(PartialEq, Props, Clone)]
struct HashButtonProps {
  name: String,
}

pub enum SelectionState {
  Unselected,
  Selected,
  Disabled,
  Invalid,
}

#[component]
pub fn HashButton( props: HashButtonProps ) -> Element {
  let mut build = use_context::<BuildContext>();
  let behavoir = build.path_behavoir( &props.name );
  let class: String = match behavoir {
    SelectionState::Unselected => "tile unselected",
    SelectionState::Selected => "tile selected",
    SelectionState::Disabled => "tile unselected hidden",
    SelectionState::Invalid => "tile selected disabled"
  }.into();
  rsx! {
    div {
      class: class,
      onclick: move |_| {
        build.path_toggle( &props.name );
      },
      "{props.name}"
    }
  }
}

#[derive(PartialEq, Props, Clone)]
struct PathOptionDropdownProps {
  pub path_id: String,
  pub option_id: String,
  pub selection_text: String,
  pub options: Vec<(String,String)>,
}

impl PathOptionDropdownProps {
  pub fn valid_option( &self, option_value: &String ) -> bool {
    for ( key, _ ) in self.options.iter() {
      if key.cmp( option_value ) == Ordering::Equal { return true; }
    }
    return false;
  }
}

#[component]
pub fn PathOptionDropdown( props: PathOptionDropdownProps ) -> Element {
  let mut build = use_context::<BuildContext>();
  
  let option = match build.get_path_option( &props.path_id, &props.option_id ) {
    Some( option ) => {
      match props.valid_option( &option ) {
        true => Some( option ),
        false => None,
      }
    },
    None => None,
  };
  rsx! {
    div {
      div { "Option: {props.path_id}-{props.option_id}" }
      div {
        select {
          onchange: move |event| {
            let new_value = event.value();
            let valid = props.valid_option( &new_value );
            if valid {
              build.set_path_option( &props.path_id, &props.option_id, new_value );
            }
          },
          option { disabled: true, selected: option.clone().is_none(), "{props.selection_text}" }
          for ( value, label ) in props.options.iter() {
            option {
              value: "{value}",
              label: "{label}",
              selected: option.is_some() && option.clone().unwrap().cmp( value ) == Ordering::Equal,
            }
          }
        }
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct BuildContext {
  pub level: Signal<usize>,
  pub paths: Signal<HashSet<String>>,
  pub path_options: Signal<HashMap<(String,String),String>>,
  pub training: Signal<TrainingSet>,
}

impl BuildContext {
  pub fn use_context_provider()-> Self {
    let level = use_signal( || MIN_LEVEL );
    let paths: Signal<HashSet<String>> = use_signal( || HashSet::new() );
    let path_options:Signal<HashMap<(String,String),String>> = use_signal( || HashMap::new() );
    let training:Signal<TrainingSet> = use_signal( || TrainingSet::default() );

    use_context_provider( || Self {
      level,
      paths, path_options,
      training,
    } )
  }

  pub fn path_behavoir( &self, name: &String ) -> SelectionState {
    let track = use_context::<TrackContext>();
    let level = self.level;
    let stats = track.character.stats( level() );

    match (
      self.paths.read().contains( name ),
      self.paths.read().len() < stats.iniatite.path_max,
      self.paths.read().len() > stats.iniatite.path_max,
    ) {
        ( true, _, false ) => SelectionState::Selected,
        ( false, true, _ ) => SelectionState::Unselected,
        ( false, false, _ ) => SelectionState::Disabled,
        ( true, _, true ) => SelectionState::Invalid,
    }
  }

  pub fn path_toggle( &mut self, name: &String ) {
    let mut current = self.paths.read().clone();
    match self.path_behavoir( name ) {
      SelectionState::Selected | SelectionState::Invalid => current.remove( name ),
      SelectionState::Unselected => current.insert( name.clone() ),
      _ => false,
    };
    self.paths.set( current );
  }

  pub fn get_path_option( &self, path_id: &String, option_id: &String ) -> Option<String> {
    let path_options = self.path_options;
    return match path_options().get( &( path_id.clone(), option_id.clone() ) ) {
      Some( value ) => Some( value.clone() ),
      None => None,
    }
  }

  pub fn set_path_option( &mut self, path_id: &String, option_id: &String, value: String ) {
    let mut path_options = self.path_options;
    let mut binding = path_options();
    let entry = binding.entry( ( path_id.clone(), option_id.clone() ) ).or_default();
    *entry = value.clone();
    path_options.set( binding );
  }
}



#[derive(Debug, Clone, Default)]
pub struct TrainingSet {
  pub adept: i32,
  pub endurance: i32,
  pub expert: i32,
  pub innate: i32,
  pub resonance: i32,
  pub magic: i32,
}