pub mod component;

// mod build;
mod fixed;
mod growth;
mod qualifier;
mod track;
mod training;

pub mod prelude {
  pub use super::growth::{CharacterGrowth, LevelGrowth};
  // pub use super::track::{character_growth_track, training_growth_track};
  pub use super::training::{CharacterBonus, TrainingGrowth};
}
