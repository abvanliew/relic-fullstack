use dioxus::prelude::*;

use crate::progression::training::FlowBonus;
use crate::progression::training::RankBonus;
use crate::rule::prelude::*;
use crate::progression::prelude::*;

use crate::operator::InstanceBonus as I;
use crate::operator::StackingBonus as S;


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
      CharacterBonus {expertise: RankBonus {rank: S::plus(1),..Default::default()}, ..Default::default() },
      CharacterBonus {expertise: RankBonus {rank: S::plus(1),..Default::default()}, hp: S::plus( 1 ), ..Default::default() },
      CharacterBonus {expertise: RankBonus {max_rank: S::plus(1),..Default::default()}, ..Default::default() },
    ],
    adept: vec![
      CharacterBonus {hp: S::plus( 1 ), ..Default::default() },
      CharacterBonus {hp: S::plus( 1 ), capability: RankBonus {rank: S::plus(1),..Default::default()}, ..Default::default() },
      CharacterBonus {capability: RankBonus {max_rank: S::plus(1),..Default::default()}, ..Default::default() },
    ],
    endurance: vec![
      CharacterBonus {hp: S::plus( 2 ), ..Default::default() },
      CharacterBonus {hp: S::plus( 1 ), defense: RankBonus {rank: S::plus(1),..Default::default()}, ..Default::default() },
      CharacterBonus {hp: S::plus( 2 ), defense: RankBonus {max_rank: S::plus(1),..Default::default()}, ..Default::default() },
      CharacterBonus {hp: S::plus( 1 ), ..Default::default() },
      CharacterBonus {hp: S::plus( 2 ), defense: RankBonus {rank: S::plus(1),..Default::default()}, ..Default::default() },
      CharacterBonus {con: S::plus( 1 ), defense: RankBonus {max_rank: S::plus(1),..Default::default()}, ..Default::default() },
    ],
    innate: vec![
      CharacterBonus { hp: S::plus( 1 ), innate: FlowBonus { pool: S::plus(1), ..Default::default()}, ..Default::default() },
      CharacterBonus { hp: S::plus( 1 ), innate: FlowBonus { pool: S::plus(1), flow: S::plus(1), ..Default::default()}, ..Default::default() },
      CharacterBonus { hp: S::plus( 1 ), innate: FlowBonus { pool: S::plus(1), pool_all: S::plus(1), ..Default::default()}, ..Default::default() },

    ],
    resonance: vec![
      CharacterBonus { resonance: FlowBonus { pool: S::plus(1), ..Default::default()}, ..Default::default() },
      CharacterBonus { resonance: FlowBonus { pool: S::plus(1), flow: S::plus(1), ..Default::default()}, ..Default::default() },
      CharacterBonus { resonance: FlowBonus { pool: S::plus(1), pool_all: S::plus(1), ..Default::default()}, hp: S::plus( 1 ), ..Default::default() },
    ],
    magic: vec![
      CharacterBonus { magic: FlowBonus { flow: S::plus( 1 ), pool: S::plus(1), ..Default::default()}, ..Default::default() },
      CharacterBonus { magic: FlowBonus { flow: S::plus( 1 ), pool: S::plus(1), ..Default::default()}, ..Default::default() },
      CharacterBonus { magic: FlowBonus { flow: S::plus( 1 ), pool: S::plus(1), pool_all: S::plus(1), ..Default::default()}, ..Default::default() },
    ],
  }
}
