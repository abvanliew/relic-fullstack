use dioxus::prelude::*;

use crate::rule::prelude::*;
use crate::progression::prelude::*;

#[derive(Debug, Clone)]
pub struct TrackContext {
  pub character: CharacterGrowth,
  pub training: TrainingGrowth,
}

impl TrackContext {
  pub fn use_context_provider()-> Self {
    let character = character_growth_track();
    let training = training_growth_track();
    use_context_provider( || Self{ character, training }  )
  }
}

pub fn character_growth_track() -> CharacterGrowth {
  CharacterGrowth {
    levels: vec![
      // Level 1
      LevelGrowth {
        tier: Some( Tier::Initiate ),
        hp: Some( 25 ),
        max_ranks: Some( 5 ),
        attributes: Some( 19 ),
        expertise: Some( 20 ),
        max_training: Some( 1 ),
        training: Some( 1 ),
        paths: Some( 1 ),
        path_features: Some( 1 ),
        ..Default::default()
      },
      // Level 2
      LevelGrowth {
        hp: Some( 1 ),
        attributes: Some( 1 ),
        expertise: Some( 1 ),
        max_training: Some( 1 ),
        training: Some( 2 ),
        path_features: Some( 1 ),
        features: Some( 1 ),
        ..Default::default()
      },
      // Level 3
      LevelGrowth {
        hp: Some( 1 ),
        attributes: Some( 2 ),
        expertise: Some( 2 ),
        max_training: Some( 1 ),
        training: Some( 1 ),
        path_features: Some( 1 ),
        half_features: Some( 1 ),
        ..Default::default()
      },
      // Level 4
      LevelGrowth {
        max_ranks: Some( 1 ),
        hp: Some( 1 ),
        attributes: Some( 1 ),
        expertise: Some( 1 ),
        features: Some( 1 ),
        max_training: Some( 1 ),
        training: Some( 2 ),
        ..Default::default()
      },
      // Level 5
      LevelGrowth {
        hp: Some( 1 ),
        attributes: Some( 2 ),
        expertise: Some( 2 ),
        features: Some( 1 ),
        max_training: Some( 1 ),
        training: Some( 1 ),
        half_features: Some( 1 ),
        ..Default::default()
      },
      // Level 6
      LevelGrowth {
        hp: Some( 1 ),
        attributes: Some( 2 ),
        expertise: Some( 1 ),
        max_training: Some( 1 ),
        training: Some( 2 ),
        features: Some( 1 ),
        ..Default::default()
      },
      // Level 7
      LevelGrowth {
        tier: Some( Tier::Journeyman ),
        hp: Some( 5 ),
        max_ranks: Some( 1 ),
        attributes: Some( 1 ),
        expertise: Some( 2 ),
        max_training: Some( 1 ),
        training: Some( 1 ),
        paths: Some( 1 ),
        features: Some( 1 ),
        ..Default::default()
      },
      // Level 8
      LevelGrowth {
        hp: Some( 1 ),
        attributes: Some( 1 ),
        expertise: Some( 1 ),
        max_training: Some( 1 ),
        training: Some( 2 ),
        path_features: Some( 1 ),
        half_features: Some( 1 ),
        ..Default::default()
      },
      // Level 9
      LevelGrowth {
        hp: Some( 1 ),
        attributes: Some( 2 ),
        expertise: Some( 2 ),
        max_training: Some( 1 ),
        training: Some( 1 ),
        path_features: Some( 1 ),
        ..Default::default()
      },
      // Level 10
      LevelGrowth {
        max_ranks: Some( 1 ),
        hp: Some( 1 ),
        attributes: Some( 1 ),
        expertise: Some( 1 ),
        max_training: Some( 1 ),
        training: Some( 2 ),
        features: Some( 1 ),
        ..Default::default()
      },
      // Level 11
      LevelGrowth {
        hp: Some( 1 ),
        attributes: Some( 2 ),
        expertise: Some( 2 ),
        features: Some( 1 ),
        max_training: Some( 1 ),
        training: Some( 1 ),
        half_features: Some( 1 ),
        ..Default::default()
      },
      // Level 12
      LevelGrowth {
        hp: Some( 1 ),
        attributes: Some( 2 ),
        expertise: Some( 1 ),
        max_training: Some( 1 ),
        training: Some( 2 ),
        features: Some( 1 ),
        ..Default::default()
      },
    ]
  }
}

pub fn training_growth_track() -> TrainingGrowth {
  return TrainingGrowth {
    expert: vec![
      CharacterBonus { rank: Some( 1 ), ..Default::default() },
      CharacterBonus { rank: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), max_rank: Some( 1 ), ..Default::default() },
      CharacterBonus { rank: Some( 1 ), ..Default::default() },
      CharacterBonus { rank: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), max_rank: Some( 1 ), ..Default::default() },
    ],
    adept: vec![
      CharacterBonus { hp: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), rank: Some( 1 ), ..Default::default() },
      CharacterBonus { max_rank: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), rank: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), rank: Some( 1 ), ..Default::default() },
      CharacterBonus { max_rank: Some( 1 ), ..Default::default() },
    ],
    endurance: vec![
      CharacterBonus { hp: Some( 2 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), rank: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 2 ), max_rank: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), rank: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 2 ), rank: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), con: Some( 1 ), max_rank: Some( 1 ), ..Default::default() },
    ],
    innate: vec![
      CharacterBonus { hp: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), pool: Some( 2 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), pool_all: Some( 1 ), flow: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), pool: Some( 2 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), pool_all: Some( 1 ), flow: Some( 1 ), ..Default::default() },
    ],
    resonance: vec![
      CharacterBonus { pool: Some( 1 ), ..Default::default() },
      CharacterBonus { pool: Some( 2 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), flow: Some( 1 ), pool_all: Some( 1 ), ..Default::default() },
      CharacterBonus { pool: Some( 1 ), ..Default::default() },
      CharacterBonus { pool: Some( 2 ), ..Default::default() },
      CharacterBonus { hp: Some( 1 ), flow: Some( 1 ), pool_all: Some( 1 ), ..Default::default() },
    ],
    magic: vec![
      CharacterBonus { flow: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      CharacterBonus { flow: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      CharacterBonus { flow: Some( 1 ), pool: Some( 1 ), pool_all: Some( 1 ), ..Default::default() },
      CharacterBonus { flow: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      CharacterBonus { flow: Some( 1 ), pool: Some( 1 ), ..Default::default() },
      CharacterBonus { flow: Some( 1 ), pool: Some( 1 ), pool_all: Some( 1 ), ..Default::default() },
    ],
  }
}
