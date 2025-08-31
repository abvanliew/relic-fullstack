use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;

use crate::asset::icon::{IMG_SELECTED, IMG_UNSELECTED};
use crate::server::prelude::GameLibrarySignal;
use crate::skill::prelude::*;
use crate::path::Path;
use crate::progression::fixed::MIN_LEVEL;
use crate::progression::track::TrackContext;
use super::level::LevelSelector;

#[component]
pub fn CharacterProgression( paths: Vec<Path> ) -> Element {
  let library = use_context::<GameLibrarySignal>();
  let res_paths = library.get_paths();
  let path_map: HashMap<String,Path>;
  match res_paths {
    Some( result) => {path_map = result},
    _ => { return rsx!{
      div { "Loading" }
    };},
  };
  TrackContext::use_context_provider();
  let build_context = BuildContext::use_context_provider();
  let paths = build_context.paths;
  let path_count = paths.len();
  rsx! {
    LevelSelector {}
    div { "Path Count: {path_count}" }
    div {
      class: "column",
      for ( id, path ) in path_map {
        PathSelector { id, path }
      }
    }
  }
}


#[derive(PartialEq, Props, Clone)]
struct PathSelectorProps {
  id: String,
  path: ReadOnlySignal<Path>,
}

#[component]
pub fn PathSelector(props:PathSelectorProps) -> Element {
  let mut build = use_context::<BuildContext>();
  let path = (props.path)();
  let id = props.id;
  let title = path.title;
  let summary = path.summary.unwrap_or( "".to_string() );
  let skill_ids = path.skill_ids.unwrap_or(Vec::new());
  let library = use_context::<GameLibrarySignal>();
  let res_skills = library.get_skills();
  let ( skills, loading ): (Vec<Skill>,bool) = match res_skills {
    Some( skill_map ) => {
      (
        skill_ids.iter()
        .map_while(|id| skill_map.get(&id.to_string()))
        .map(|skill|skill.clone())
        .collect(),
        false
      )
    },
    _ => (Vec::new(), true),
  };
  let mut display: Signal<bool> = use_signal(|| false);
  let behavoir = build.path_behavoir( &id );
  let (class, img_src) = match behavoir {
    SelectionState::Unselected => ("", IMG_UNSELECTED),
    SelectionState::Selected => ("", IMG_SELECTED),
    SelectionState::Disabled => ("hidden", IMG_UNSELECTED),
    SelectionState::Invalid => ("disabled", IMG_SELECTED),
  }.into();
  rsx! {
    div {
      class: "row path-card {class}",
      onclick: move |_| { display.set(!display()); },
      div {
        class: "path-checkbox-wrapper",
        onclick: move |evt: Event<MouseData>| {
          evt.stop_propagation();
          build.path_toggle(&id);
        },
        img { src: "{img_src}" }
      }
      div { class: "path-title", "{title}" }
      div { class: "path-description", "{summary}" }
    }
    div {
      class: if display() { "path-skill-panels {class}" } else { "hidden" },
      match loading {
        true => rsx! { div { "loading" } },
        _ => rsx! {
          for skill in skills {
            div {
              class: "path-skill-group",
              SkillDescription { id: skill.id.to_string(), show_terms: false }
            }
          }
        }
      }
    }
  }
}


#[derive(PartialEq, Props, Clone)]
struct SkillSelectorProps {
  id: String,
}


#[component]
pub fn SkillSelector(props:SkillSelectorProps) -> Element {
  let mut build = use_context::<BuildContext>();
  let id = props.id;
  let mut display: Signal<bool> = use_signal(|| false);
  let behavoir = build.skill_behavoir( &id );
  let (class, img_src) = match behavoir {
    SelectionState::Unselected => ("", IMG_UNSELECTED),
    SelectionState::Selected => ("", IMG_SELECTED),
    SelectionState::Disabled => ("hidden", IMG_UNSELECTED),
    SelectionState::Invalid => ("disabled", IMG_SELECTED),
  }.into();
  rsx! {
    div {
    }
  }
}

#[derive(PartialEq, Props, Clone)]
struct HashButtonProps {
  name: String,
  id: String,
  class: Option<String>,
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
pub struct HashSelectorContext {
  pub selection: Signal<HashSet<String>>,
}

impl HashSelectorContext {
  pub fn use_context_provider()-> Self {
    let selection: Signal<HashSet<String>> = use_signal( || HashSet::new() );
    use_context_provider( || Self { selection } )
  }

  pub fn len( &self ) -> usize {
    return self.selection.read().len();
  }

  pub fn selection_behavoir( &self, name: &String, max_len: usize ) -> SelectionState {
    let selection = self.selection.read();
    match (
      selection.contains( name ),
      selection.len() < max_len,
      selection.len() > max_len,
    ) {
        ( true, _, false ) => SelectionState::Selected,
        ( false, true, _ ) => SelectionState::Unselected,
        ( false, false, _ ) => SelectionState::Disabled,
        ( true, _, true ) => SelectionState::Invalid,
    }
  }

  pub fn toggle( &mut self, name: &String, max_len: usize ) {
    let mut current = self.selection.read().clone();
    match self.selection_behavoir( name, max_len ) {
      SelectionState::Selected | SelectionState::Invalid => current.remove( name ),
      SelectionState::Unselected => current.insert( name.clone() ),
      _ => false,
    };
    self.selection.set( current );
  }
}


#[derive(Debug, Clone)]
pub struct BuildContext {
  pub level: Signal<usize>,
  pub paths: HashSelectorContext,
  pub skills: HashSelectorContext,
  pub path_options: Signal<HashMap<(String,String),String>>,
  pub training: Signal<TrainingSet>,
}

impl BuildContext {
  pub fn use_context_provider()-> Self {
    let level = use_signal( || MIN_LEVEL );
    let paths = HashSelectorContext::use_context_provider();
    let skills = HashSelectorContext::use_context_provider();
    let path_options:Signal<HashMap<(String,String),String>> = use_signal( || HashMap::new() );
    let training:Signal<TrainingSet> = use_signal( || TrainingSet::default() );

    use_context_provider(|| Self {
      level,
      paths, skills, path_options,
      training,
    })
  }

  pub fn path_max( &self ) -> usize {
    let track = use_context::<TrackContext>();
    let level = self.level;
    let stats = track.character.stats( level() );
    return stats.iniatite.path_max;
  }

  pub fn path_behavoir( &self, name: &String ) -> SelectionState {
    return self.paths.selection_behavoir(name, self.path_max());
  }

  pub fn path_toggle( &mut self, name: &String ) {
    return self.paths.toggle(name, self.path_max());
  }

  pub fn skill_max( &self ) -> usize {
    let track = use_context::<TrackContext>();
    let level = self.level;
    let stats = track.character.stats( level() );
    return stats.iniatite.features;
  }

  pub fn skill_behavoir( &self, name: &String ) -> SelectionState {
    return self.skills.selection_behavoir(name, self.skill_max());
  }

  pub fn skill_toggle( &mut self, name: &String ) {
    return self.skills.toggle(name, self.skill_max());
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