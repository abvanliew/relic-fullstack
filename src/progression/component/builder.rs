use std::collections::HashSet;
// use std::cmp::max;

use dioxus::prelude::*;

use crate::path::Path;
// use crate::progression::component::effects::{level_change_effect, path_change_effect};
use crate::progression::component::path::PathSelections;
// use crate::progression::component::TrainingRanks;
// use crate::progression::fixed::{MAX_LEVEL, MIN_LEVEL};
// use crate::progression::growth::LevelStats;
// use crate::progression::prelude::TrainingClass;
// use crate::progression::qualifier::{FlowPoolQualifier, PathQualifier};
use crate::progression::track::TrackContext;
use crate::rule::prelude::Tier;
use super::{level::{LevelContext, LevelSelector}, qualifiers::BuildQualifiers};
// use super::TrainingSignal;

#[component]
pub fn CharacterProgression( paths: Vec<Path> ) -> Element {
  TrackContext::use_context_provider();
  LevelContext::use_context_provider();
  let stats = BuildQualifiers::use_memo_stats();
  BuildContext::use_context_provider();

  // let stats = build.level_stats;
  // level_change_effect();
  // path_change_effect( paths.clone() );
  rsx! {
    LevelSelector {}
    div { "{stats:?}" }
    PathSelections { paths: paths.clone(), tier: Tier::Initiate }
    // TrainingRanks {}
  }
}

#[derive(Debug, Clone)]
pub struct BuildContext {
  pub paths: Signal<HashSet<String>>,

  // pub path_qualifier: PathQualifier,
  // pub all_paths: Signal<HashSet<String>>,

  // pub innate_qualifiers: Signal<Option<FlowPoolQualifier>>,
  // pub resonance_qualifiers: Signal<Option<FlowPoolQualifier>>,
  // pub magic_qualifiers: Signal<Option<FlowPoolQualifier>>,
  // pub previous_training: TrainingSignal,
  // pub trainings: Vec<TrainingSignal>,
}

impl BuildContext {
  pub fn use_context_provider()-> Self {
    let paths: Signal<HashSet<String>> = use_signal( || HashSet::new() );

    use_context_provider( || Self {
      paths,
    } )
  }

  // pub fn paths( &self ) -> HashSet<String> {
  //   let mut paths = self.paths().clone();
  //   paths.extend( (self.previous_paths)() );
  //   return paths;
  // }

  // pub fn path_count( &self ) -> usize { return self.paths().len(); }

  // pub fn has_innate( &self ) -> bool { return (self.innate_qualifiers)().is_some() }
  // pub fn has_resonance( &self ) -> bool { return (self.resonance_qualifiers)().is_some() }
  // pub fn has_magic( &self ) -> bool { return (self.magic_qualifiers)().is_some() }

  // pub fn has_training( &self, class: &TrainingClass ) -> bool {
  //   return match class {
  //     TrainingClass::Innate => self.has_innate(),
  //     TrainingClass::Resonance => self.has_resonance(),
  //     TrainingClass::Magic => self.has_magic(),
  //     _ => true,
  //   };
  // }
}
