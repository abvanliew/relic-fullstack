use std::collections::HashSet;

use dioxus::prelude::*;

use crate::path::Path;
use crate::progression::component::effects::{level_change_effect, path_change_effect};
use crate::progression::component::path::PathSelections;
use crate::progression::component::TrainingRanks;
use crate::progression::fixed::{MAX_LEVEL, MIN_LEVEL};
use crate::progression::growth::LevelStats;
use crate::progression::prelude::TrainingClass;
use crate::progression::qualifier::{FlowPoolQualifier, PathQualifier};
use crate::progression::track::TrackContext;
use crate::rule::prelude::Tier;
use super::level::{LevelSelector, LevelContext};

#[component]
pub fn CharacterProgression( paths: Vec<Path> ) -> Element {
  LevelContext::use_context_provider();
  TrackContext::use_context_provider();
  let build = BuildContext::use_context_provider();
  let stats = build.level_stats;
  level_change_effect();
  path_change_effect( paths.clone() );
  rsx! {
    LevelSelector {}
    div { "{stats:?}" }
    PathSelections { paths: paths.clone(), tier: Tier::Initiate }
    TrainingRanks {}
  }
}

#[derive(Debug, Clone)]
pub struct BuildContext {
  pub level_index: Signal<usize>,
  pub previous_paths: Signal<HashSet<String>>,
  pub level_stats: Signal<LevelStats>,
  pub path_qualifier: PathQualifier,
  pub paths: Vec<Signal<HashSet<String>>>,
  pub all_paths: Signal<HashSet<String>>,
  pub innate_qualifiers: Signal<Option<FlowPoolQualifier>>,
  pub resonance_qualifiers: Signal<Option<FlowPoolQualifier>>,
  pub magic_qualifiers: Signal<Option<FlowPoolQualifier>>,
}

impl BuildContext {
  pub fn use_context_provider()-> Self {
    let character = use_context::<TrackContext>().character;
    let level_index: Signal<usize> = use_signal( || 0 );
    let mut paths: Vec<Signal<HashSet<String>>> = Vec::new();
    for _ in 0..=(MAX_LEVEL - MIN_LEVEL) {
      paths.push( use_signal( || HashSet::new() ) );
    }
    let previous_paths = use_signal( || HashSet::new() );
    let all_paths = use_signal( || HashSet::new() );
    let level_stats = use_signal( || character.stats( MIN_LEVEL ) );
    let path_qualifier = PathQualifier::new_signal();

    let innate_qualifiers: Signal<Option<FlowPoolQualifier>> = use_signal( || None );
    let resonance_qualifiers: Signal<Option<FlowPoolQualifier>> = use_signal( || None );
    let magic_qualifiers: Signal<Option<FlowPoolQualifier>> = use_signal( || None );

    use_context_provider( || Self {
      level_index,
      path_qualifier, paths, previous_paths, all_paths,
      level_stats,
      innate_qualifiers, resonance_qualifiers, magic_qualifiers,
    } )
  }

  pub fn current_paths( &self ) -> Signal<HashSet<String>> { return self.paths[(self.level_index)()] }

  pub fn get_previous_paths( &self ) -> HashSet<String> {
    if (self.level_index)() == 0 { return HashSet::new(); }
    let mut paths: HashSet<String> =  HashSet::new();
    for i in 0..(self.level_index)() {
      paths.extend( (self.paths[i])().clone() );
    }
    return paths;
  }

  pub fn paths( &self ) -> HashSet<String> {
    let mut paths = (self.current_paths())().clone();
    paths.extend( (self.previous_paths)() );
    return paths;
  }

  fn paths_len( &self ) -> usize { return self.paths().len(); }

  pub fn at_path_max( &self ) -> bool {
    let path_max = ((self.path_qualifier.paths)() + (self.path_qualifier.paths_optional)()) as usize;
    return self.paths_len() >= path_max;
  }

  pub fn has_innate( &self ) -> bool { return (self.innate_qualifiers)().is_some() }
  pub fn has_resonance( &self ) -> bool { return (self.resonance_qualifiers)().is_some() }
  pub fn has_magic( &self ) -> bool { return (self.magic_qualifiers)().is_some() }

  pub fn has_training( &self, class: &TrainingClass ) -> bool {
    return match class {
      TrainingClass::Innate => self.has_innate(),
      TrainingClass::Resonance => self.has_resonance(),
      TrainingClass::Magic => self.has_magic(),
      _ => true,
    };
  }
}
