pub mod component;

mod fixed;
mod progress_sheet;
mod track;
mod training;

pub mod prelude {
  pub use super::track::LevelTrack;
  // pub use super::training::{CharacterBonus, TrainingGrowth};
  pub use super::component::ranks::RankDisplay;
  pub use super::fixed::{BASE_DEFENSE, BASE_RESIST};
  pub use super::progress_sheet::{TrainingTables, LevelTable};
}
