mod builder;
mod growth;
mod level;
mod paths;
pub mod ranks;
mod skills;

pub use builder::CharacterProgression;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionState {
  Unfinished,
  Finished,
  Invalid,
}
