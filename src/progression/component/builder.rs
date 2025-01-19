use std::collections::HashSet;

use dioxus::prelude::*;

use crate::path::Path;
use crate::progression::component::effects::level_change_effect;
use crate::progression::component::path::PathSelections;
use crate::progression::fixed::{MAX_LEVEL, MIN_LEVEL};
use crate::progression::growth::LevelStats;
use crate::progression::track::TrackContext;
use super::level::{LevelSelector, LevelContext};

#[component]
pub fn CharacterProgression( paths: Vec<Path> ) -> Element {
  LevelContext::use_context_provider();
  TrackContext::use_context_provider();
  let stats = BuildContext::use_context_provider().level_stats;
  level_change_effect();
  rsx!(
    LevelSelector {}
    div { "{stats:?}" }
    PathSelections { paths: paths.clone() }
  )
}

#[derive(Debug, Clone)]
pub struct BuildContext {
  pub level_index: Signal<usize>,
  pub previous_paths: Signal<HashSet<String>>,
  pub level_stats: Signal<LevelStats>,
  paths: Vec<Signal<HashSet<String>>>,
  // training: TrainingSignal,
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
    let level_stats = use_signal( || character.stats( MIN_LEVEL ) );
    use_context_provider( || Self { level_index, paths, previous_paths, level_stats } )
  }

  pub fn get_previous_paths( &self ) -> HashSet<String> {
    if (self.level_index)() == 0 { return HashSet::new(); }
    let mut paths: HashSet<String> =  HashSet::new();
    for i in 0..(self.level_index)() {
      paths.extend( (self.paths[i])().clone() );
    }
    return paths;
  }

  pub fn current_paths( &self ) -> Signal<HashSet<String>> { return self.paths[(self.level_index)()] }
}
