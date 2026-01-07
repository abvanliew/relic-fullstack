use crate::{modifiers::prelude::{ModifierClass, ModifierSet}, progression::training::TrainingClass};

#[derive(Debug, Clone)]
pub struct LevelTrack;

impl LevelTrack {
  pub fn as_of(level: i32) -> ModifierSet {
    let mut modifiers = ModifierSet::default();
    let max_index = usize::try_from(level).ok().unwrap_or_default();
    for i in 0..max_index {
      for (class, value) in level_bonuses()[i].iter() {
        modifiers.add_bonus(class, *value);
      }
    }
    return modifiers;
  }
}

#[derive(Debug, Clone)]
pub struct GrowthTrack;

impl GrowthTrack {
  fn as_of(rank: i32, track: ProgressTrack) -> ModifierSet {
    let mut modifiers = ModifierSet::default();
    let max_index = usize::try_from(rank).ok().unwrap_or_default();
    let track_len = track.len();
    if track_len == 0 { return modifiers; }
    for index in 0..max_index {
      let i = index % track_len;
      for (class, value) in track[i].iter() {
        modifiers.add_bonus(class, *value);
      }
    }
    return modifiers;
  }

  pub fn class_at(class: &TrainingClass, rank: i32) -> ModifierSet {
    return GrowthTrack::as_of(rank, match class {
      TrainingClass::Expert => expert_growth_bonuses(),
      TrainingClass::Adept => adept_growth_bonuses(),
      TrainingClass::Endurance => endurance_growth_bonuses(),
      TrainingClass::Innate => innate_growth_bonuses(),
      TrainingClass::Resonance => resonance_growth_bonuses(),
      TrainingClass::Magic => magic_growth_bonuses(),
    })
  }
}

type ProgressTrack = Vec<Vec<(ModifierClass, i32)>>;

fn level_bonuses() -> ProgressTrack {
  return vec![
    // Level 1
    vec![
      (ModifierClass::HP, 30),
      (ModifierClass::MinorFeature, 1),
      (ModifierClass::PathMin, 1),
      (ModifierClass::PathMax, 2),
      (ModifierClass::AttributeRank, 17),
      (ModifierClass::GrowthRanks, 1),
    ],
    // Level 2
    vec![
      (ModifierClass::HP, 2),
      (ModifierClass::Feature, 1),
      (ModifierClass::PathMax, 1), 
      (ModifierClass::AttributeRank, 1),
      (ModifierClass::GrowthRanks, 2)
    ],
    // Level 3
    vec![
      (ModifierClass::HP, 2),
      (ModifierClass::MinorFeature, 1),
      (ModifierClass::PathMax, 1), 
      (ModifierClass::AttributeRank, 2),
      (ModifierClass::GrowthRanks, 1)
    ],
    // Level 4
    vec![
      (ModifierClass::HP, 2),
      (ModifierClass::Feature, 1),
      (ModifierClass::AttributeRank, 1),
      (ModifierClass::GrowthRanks, 2)
    ],
    // Level 5
    vec![
      (ModifierClass::HP, 2),
      (ModifierClass::Feature, 1),
      (ModifierClass::MinorFeature, 1),
      (ModifierClass::AttributeRank, 1),
      (ModifierClass::GrowthRanks, 1)
    ],
    // Level 6
    vec![
      (ModifierClass::HP, 2),
      (ModifierClass::Feature, 1),
      (ModifierClass::AttributeRank, 2),
      (ModifierClass::GrowthRanks, 2)
    ],
  ];
}

fn adept_growth_bonuses() -> ProgressTrack {
  return vec![
    // Rank 1
    vec![(ModifierClass::HP, 1)],
    // Rank 2
    vec![(ModifierClass::HP, 1), (ModifierClass::CapabilityRank, 1)],
    // Rank 3
    vec![(ModifierClass::HP, 1), (ModifierClass::CapabilityMaxRank, 1)],
  ];
}

fn endurance_growth_bonuses() -> ProgressTrack {
  return vec![
    // Rank 1
    vec![(ModifierClass::HP, 2)],
    // Rank 2
    vec![(ModifierClass::HP, 2), (ModifierClass::DefenseRank, 1)],
    // Rank 3
    vec![(ModifierClass::HP, 2), (ModifierClass::DefenseMaxRank, 1)],
  ];
}

fn expert_growth_bonuses() -> ProgressTrack {
  return vec![
    // Rank 1
    vec![(ModifierClass::ExpertiseRank, 1)],
    // Rank 2
    vec![(ModifierClass::HP, 1), (ModifierClass::ExpertiseRank, 1)],
    // Rank 3
    vec![(ModifierClass::ExpertiseRank, 1), (ModifierClass::ExpertiseMaxRank, 1)],
    // Rank 4
    vec![(ModifierClass::HP, 1), (ModifierClass::ExpertiseRank, 1)],
    // Rank 5
    vec![(ModifierClass::ExpertiseRank, 1)],
    // Rank 6
    vec![(ModifierClass::HP, 1), (ModifierClass::ExpertiseRank, 1), (ModifierClass::ExpertiseMaxRank, 1)],
  ];
}

fn innate_growth_bonuses() -> ProgressTrack {
  return vec![
    // Rank 1
    vec![(ModifierClass::HP, 2), (ModifierClass::InnatePool, 1)],
    // Rank 2
    vec![(ModifierClass::HP, 1), (ModifierClass::InnatePool, 1), (ModifierClass::InnateFlow, 1)],
    // Rank 3
    vec![(ModifierClass::HP, 2), (ModifierClass::InnatePoolAll, 1)],
    // Rank 4
    vec![(ModifierClass::HP, 1), (ModifierClass::InnatePool, 1)],
    // Rank 5
    vec![(ModifierClass::HP, 2), (ModifierClass::InnatePool, 1), (ModifierClass::InnateFlow, 1)],
    // Rank 6
    vec![(ModifierClass::HP, 1), (ModifierClass::InnatePoolAll, 1)],
  ];
}

fn resonance_growth_bonuses() -> ProgressTrack {
  return vec![
    // Rank 1
    vec![(ModifierClass::ResonancePool, 1)],
    // Rank 2
    vec![(ModifierClass::ResonancePool, 1), (ModifierClass::ResonanceFlow, 1)],
    // Rank 3
    vec![(ModifierClass::HP, 1), (ModifierClass::ResonancePoolAll, 1)],
  ];
}

fn magic_growth_bonuses() -> ProgressTrack {
  return vec![
    // Rank 1
    vec![(ModifierClass::MagicFlow, 1), (ModifierClass::MagicPool, 1)],
    // Rank 2
    vec![(ModifierClass::MagicFlow, 1), (ModifierClass::MagicPool, 1)],
    // Rank 3
    vec![(ModifierClass::MagicFlow, 1), (ModifierClass::MagicPool, 1), (ModifierClass::MagicPoolAll, 1)],
  ];
}

// pub fn character_growth_track() -> CharacterGrowth {
//   CharacterGrowth {
//     levels: vec![
//       // Level 1
//       LevelGrowth {
//         tier: Some(Tier::Initiate),
//         hp: Some(25),
//         max_ranks: Some(5),
//         attributes: Some(19),
//         expertise: Some(20),
//         max_training: Some(1),
//         training: Some(1),
//         paths: Some(1),
//         path_features: Some(1),
//         ..Default::default()
//       },
//       // Level 2
//       LevelGrowth {
//         hp: Some(1),
//         attributes: Some(1),
//         expertise: Some(1),
//         max_training: Some(1),
//         training: Some(2),
//         path_features: Some(1),
//         features: Some(1),
//         ..Default::default()
//       },
//       // Level 3
//       LevelGrowth {
//         hp: Some(1),
//         attributes: Some(2),
//         expertise: Some(2),
//         max_training: Some(1),
//         training: Some(1),
//         path_features: Some(1),
//         half_features: Some(1),
//         ..Default::default()
//       },
//       // Level 4
//       LevelGrowth {
//         max_ranks: Some(1),
//         hp: Some(1),
//         attributes: Some(1),
//         expertise: Some(1),
//         features: Some(1),
//         max_training: Some(1),
//         training: Some(2),
//         ..Default::default()
//       },
//       // Level 5
//       LevelGrowth {
//         hp: Some(1),
//         attributes: Some(2),
//         expertise: Some(2),
//         features: Some(1),
//         max_training: Some(1),
//         training: Some(1),
//         half_features: Some(1),
//         ..Default::default()
//       },
//       // Level 6
//       LevelGrowth {
//         hp: Some(1),
//         attributes: Some(2),
//         expertise: Some(1),
//         max_training: Some(1),
//         training: Some(2),
//         features: Some(1),
//         ..Default::default()
//       },
//       // Level 7
//       LevelGrowth {
//         tier: Some(Tier::Journeyman),
//         hp: Some(5),
//         max_ranks: Some(1),
//         attributes: Some(1),
//         expertise: Some(2),
//         max_training: Some(1),
//         training: Some(1),
//         paths: Some(1),
//         features: Some(1),
//         ..Default::default()
//       },
//       // Level 8
//       LevelGrowth {
//         hp: Some(1),
//         attributes: Some(1),
//         expertise: Some(1),
//         max_training: Some(1),
//         training: Some(2),
//         path_features: Some(1),
//         half_features: Some(1),
//         ..Default::default()
//       },
//       // Level 9
//       LevelGrowth {
//         hp: Some(1),
//         attributes: Some(2),
//         expertise: Some(2),
//         max_training: Some(1),
//         training: Some(1),
//         path_features: Some(1),
//         ..Default::default()
//       },
//       // Level 10
//       LevelGrowth {
//         max_ranks: Some(1),
//         hp: Some(1),
//         attributes: Some(1),
//         expertise: Some(1),
//         max_training: Some(1),
//         training: Some(2),
//         features: Some(1),
//         ..Default::default()
//       },
//       // Level 11
//       LevelGrowth {
//         hp: Some(1),
//         attributes: Some(2),
//         expertise: Some(2),
//         features: Some(1),
//         max_training: Some(1),
//         training: Some(1),
//         half_features: Some(1),
//         ..Default::default()
//       },
//       // Level 12
//       LevelGrowth {
//         hp: Some(1),
//         attributes: Some(2),
//         expertise: Some(1),
//         max_training: Some(1),
//         training: Some(2),
//         features: Some(1),
//         ..Default::default()
//       },
//     ],
//   }
// }

