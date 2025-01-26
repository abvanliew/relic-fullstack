use std::collections::HashMap;

use dioxus::prelude::*;

use crate::character::prelude::{Flow, PoolModifier};
use crate::path::Path;
use crate::progression::fixed::{MAX_LEVEL, MIN_LEVEL};
use crate::progression::qualifier::FlowPoolQualifier;
use crate::progression::track::TrackContext;
use super::{builder::BuildContext, level::LevelContext};

pub fn level_change_effect() {
  let level = use_context::<LevelContext>().level;
  let track = use_context::<TrackContext>();
  let mut build = use_context::<BuildContext>();
  let previous_training = build.get_previous_trainings().clone();
  use_memo(move || {
    let mut new_index = level() - MIN_LEVEL;
    if new_index >= MAX_LEVEL { new_index = MAX_LEVEL - 1; }
    build.level_index.set( new_index );
    build.level_stats.set( track.character.stats( level() ) );
    build.previous_paths.set( build.get_previous_paths() );
    build.previous_training.set_from( previous_training );
    build.path_qualifier.set( track.character.get_path_qualifiers( level() ) );
  } );
}

pub fn path_change_effect( paths: Vec<Path> ) {
  let mut build = use_context::<BuildContext>();
  use_memo( move || {
    let updated_paths = build.paths();
    build.all_paths.set( updated_paths.clone() );
    let mut modifiers: Vec<PoolModifier> = Vec::new();
    for path in &paths {
      if !updated_paths.contains( &path.id.to_string() ) { continue; }
      modifiers.extend( path.resource_pool_modifiers() );
    }
    let mut resources: HashMap<Flow,FlowPoolQualifier> = HashMap::new();
    for modifier in modifiers {
      let flow = modifier.resource.flow();
      let entry = resources.entry( flow ).or_default();
      entry.flow += modifier.flow;
      let pool = entry.pools.entry( modifier.resource ).or_default();
      *pool += modifier.pool;
    }
    for flow in Flow::ordered() {
      let flow_resource = match resources.get( &flow ) {
        Some( value ) => Some( value.clone() ), _ => None
      };
      match &flow {
        Flow::Innate => build.innate_qualifiers.set( flow_resource ),
        Flow::Resonance => build.resonance_qualifiers.set( flow_resource ),
        Flow::Magic => build.magic_qualifiers.set( flow_resource ),
      }
    }
  } );
}
