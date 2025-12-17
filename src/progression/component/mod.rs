mod builder;
mod growth;
mod level;
mod paths;

pub use builder::CharacterProgression;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionState {
  Unfinished,
  Finished,
  Invalid,
}
