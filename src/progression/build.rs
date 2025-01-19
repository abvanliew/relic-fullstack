use std::collections::HashSet;

use dioxus::prelude::*;

use super::fixed::{MAX_LEVEL, MIN_LEVEL};

#[derive(Debug, Clone, PartialEq)]
pub struct CharacterBuildSignal {
  pub levels: Vec<BuildSignal>
}

impl CharacterBuildSignal {
  pub fn retrieve( &mut self, level: u32 ) -> Option<BuildSignal> {
    if level > MAX_LEVEL || level < MIN_LEVEL { return None; }
    let level_as: usize = level.try_into().unwrap();
    if self.levels.len() < level_as {
      for _ in 0..( level_as - self.levels.len() ) {
        self.levels.push( BuildSignal::new_signal() );
      }
    }
    return Some( self.levels[level_as - 1].clone() );
  }

  pub fn use_context_provider()-> Self {
    let mut build = CharacterBuildSignal::default();
    build.retrieve( MIN_LEVEL );
    use_context_provider( || build )
  }

  pub fn get_previous_paths( &self, level: u32 ) -> HashSet<String> {
    if level > MAX_LEVEL || level <= MIN_LEVEL { return HashSet::new(); }
    let level_as: usize = level.try_into().unwrap();
    if level_as > self.levels.len() { return HashSet::new(); }
    let mut paths: HashSet<String> =  HashSet::new();
    for i in 0..level_as {
      paths.extend( (self.levels[i].paths)().clone() );
    }
    return paths;
  }
}


