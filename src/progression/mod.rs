pub mod component;

mod fixed;
mod track;
mod training;

pub mod prelude {
  // pub use super::track::{character_growth_track, training_growth_track};
  // pub use super::training::{CharacterBonus, TrainingGrowth};
  pub use super::component::ranks::RankDisplay;
  pub use super::fixed::{BASE_DEFENSE, BASE_RESIST, MAX_LEVEL, MIN_LEVEL};
}
