use dioxus::prelude::*;
use crate::progression::{growth::LevelStats, track::TrackContext};

use super::LevelContext;

#[derive(Clone, Debug)]
pub struct BuildQualifiers;

impl BuildQualifiers {
  pub fn use_memo_stats() -> Memo<LevelStats> {
    let track = use_context::<TrackContext>();
    let level = LevelContext::use_context_provider().level;
    return use_memo( move || track.character.stats( level() ) )
  }
}
