use dioxus::prelude::*;

use crate::progression::{fixed::{MAX_LEVEL, MIN_LEVEL}, track::TrackContext};

use super::{builder::BuildContext, level::LevelContext};

pub fn level_change_effect() {
  let level = use_context::<LevelContext>().level;
  let track = use_context::<TrackContext>();
  let mut build = use_context::<BuildContext>();
  use_effect(move || {
    let mut new_index = level() - MIN_LEVEL;
    if new_index >= MAX_LEVEL { new_index = MAX_LEVEL - 1; }
    build.level_index.set( new_index );
    build.level_stats.set( track.character.stats( level() ) );
    build.previous_paths.set( build.get_previous_paths() );
  } );
}
