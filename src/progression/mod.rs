pub mod fixed;
mod progression_tables;
mod track;
mod training;

pub mod prelude {
  pub use super::fixed::{BASE_DEFENSE, BASE_RESIST};
  pub use super::progression_tables::{DevelopmentTable, DevelopmentTables, LevelTable};
  pub use super::track::{GrowthTrack, LevelTrack};
  pub use super::training::TrainingClass;
}
