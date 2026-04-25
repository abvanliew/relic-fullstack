use crate::{
  modifiers::prelude::{ModifierClass, ModifierSet},
  progression::training::TrainingClass,
};

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
    if track_len == 0 {
      return modifiers;
    }
    for index in 0..max_index {
      let i = index % track_len;
      for (class, value) in track[i].iter() {
        modifiers.add_bonus(class, *value);
      }
    }
    return modifiers;
  }

  pub fn class_at(class: &TrainingClass, rank: i32) -> ModifierSet {
    return GrowthTrack::as_of(
      rank,
      match class {
        TrainingClass::Expert => expert_growth_bonuses(),
        TrainingClass::Adept => adept_growth_bonuses(),
        TrainingClass::Endurance => endurance_growth_bonuses(),
        TrainingClass::Innate => innate_growth_bonuses(),
        TrainingClass::Resonance => resonance_growth_bonuses(),
        TrainingClass::Magic => magic_growth_bonuses(),
      },
    );
  }
}

type ProgressTrack = Vec<Vec<(ModifierClass, i32)>>;

fn level_bonuses() -> ProgressTrack {
  return vec![
    // Level 1
    vec![
      (ModifierClass::HP, 30),
      (ModifierClass::Constituion, 4),
      (ModifierClass::MinorFeature, 1),
      (ModifierClass::InitiatePathMin, 1),
      (ModifierClass::InitiatePathMax, 2),
      (ModifierClass::RankMax, 5),
      (ModifierClass::AttributeRank, 4),
      (ModifierClass::CapabilityRank, 8),
      (ModifierClass::DefenseRank, 8),
      (ModifierClass::GrowthRanks, 1),
      (ModifierClass::WalkingSpeed, 6),
      (ModifierClass::DashSpeed, 3),
    ],
    // Level 2
    vec![
      (ModifierClass::HP, 2),
      (ModifierClass::Feature, 1),
      (ModifierClass::InitiatePathMax, 1),
      (ModifierClass::AttributeRank, 1),
      (ModifierClass::GrowthRanks, 2),
    ],
    // Level 3
    vec![
      (ModifierClass::HP, 2),
      (ModifierClass::MinorFeature, 1),
      (ModifierClass::InitiatePathMax, 1),
      (ModifierClass::AttributeRank, 2),
      (ModifierClass::GrowthRanks, 1),
    ],
    // Level 4
    vec![
      (ModifierClass::HP, 2),
      (ModifierClass::Feature, 1),
      (ModifierClass::RankMax, 1),
      (ModifierClass::AttributeRank, 1),
      (ModifierClass::GrowthRanks, 2),
    ],
    // Level 5
    vec![
      (ModifierClass::HP, 2),
      (ModifierClass::Feature, 1),
      (ModifierClass::MinorFeature, 1),
      (ModifierClass::AttributeRank, 1),
      (ModifierClass::GrowthRanks, 1),
    ],
    // Level 6
    vec![
      (ModifierClass::HP, 2),
      (ModifierClass::Feature, 1),
      (ModifierClass::AttributeRank, 2),
      (ModifierClass::GrowthRanks, 2),
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
    vec![
      (ModifierClass::HP, 1),
      (ModifierClass::CapabilitySpecialization, 1),
    ],
  ];
}

fn endurance_growth_bonuses() -> ProgressTrack {
  return vec![
    // Rank 1
    vec![(ModifierClass::HP, 2)],
    // Rank 2
    vec![(ModifierClass::HP, 2), (ModifierClass::DefenseRank, 1)],
    // Rank 3
    vec![(ModifierClass::HP, 2), (ModifierClass::DefenseSpecialization, 1)],
  ];
}

fn expert_growth_bonuses() -> ProgressTrack {
  return vec![
    // Rank 1
    vec![(ModifierClass::ExpertiseRank, 1)],
    // Rank 2
    vec![(ModifierClass::HP, 1), (ModifierClass::ExpertiseRank, 1)],
    // Rank 3
    vec![
      (ModifierClass::ExpertiseRank, 1),
      (ModifierClass::ExpertiseSpecialization, 1),
    ],
    // Rank 4
    vec![(ModifierClass::HP, 1), (ModifierClass::ExpertiseRank, 1)],
    // Rank 5
    vec![(ModifierClass::ExpertiseRank, 1)],
    // Rank 6
    vec![
      (ModifierClass::HP, 1),
      (ModifierClass::ExpertiseRank, 1),
      (ModifierClass::ExpertiseSpecialization, 1),
    ],
  ];
}

fn innate_growth_bonuses() -> ProgressTrack {
  return vec![
    // Rank 1
    vec![(ModifierClass::HP, 2), (ModifierClass::InnatePool, 1)],
    // Rank 2
    vec![
      (ModifierClass::HP, 1),
      (ModifierClass::InnatePool, 1),
      (ModifierClass::InnateFlow, 1),
    ],
    // Rank 3
    vec![(ModifierClass::HP, 2), (ModifierClass::InnatePoolAll, 1)],
    // Rank 4
    vec![(ModifierClass::HP, 1), (ModifierClass::InnatePool, 1)],
    // Rank 5
    vec![
      (ModifierClass::HP, 2),
      (ModifierClass::InnatePool, 1),
      (ModifierClass::InnateFlow, 1),
    ],
    // Rank 6
    vec![(ModifierClass::HP, 1), (ModifierClass::InnatePoolAll, 1)],
  ];
}

fn resonance_growth_bonuses() -> ProgressTrack {
  return vec![
    // Rank 1
    vec![(ModifierClass::ResonancePool, 1)],
    // Rank 2
    vec![
      (ModifierClass::ResonancePool, 1),
      (ModifierClass::ResonanceFlow, 1),
    ],
    // Rank 3
    vec![(ModifierClass::HP, 1), (ModifierClass::ResonancePoolAll, 1)],
  ];
}

fn magic_growth_bonuses() -> ProgressTrack {
  return vec![
    // Rank 1
    vec![
      (ModifierClass::MagicFlow, 1),
      (ModifierClass::ManaPoolMinor, 1),
    ],
    // Rank 2
    vec![
      (ModifierClass::MagicFlow, 1),
      (ModifierClass::ManaPoolMinor, 1),
    ],
    // Rank 3
    vec![
      (ModifierClass::MagicFlow, 1),
      (ModifierClass::ManaPoolMinor, 1),
    ],
    // Rank 4
    vec![
      (ModifierClass::MagicFlow, 1),
      (ModifierClass::ManaPoolMinor, 1),
    ],
    // Rank 5
    vec![
      (ModifierClass::MagicFlow, 1),
      (ModifierClass::ManaPoolMinor, 1),
    ],
    // Rank 6
    vec![
      (ModifierClass::MagicFlow, 1),
      (ModifierClass::ManaPoolMinor, 1),
    ],
    // Rank 7
    vec![
      (ModifierClass::ManaPoolMinor, 1),
      (ModifierClass::ManaPoolModerate, 1),
    ],
    // Rank 8
    vec![
      (ModifierClass::ManaPoolMinor, 1),
      (ModifierClass::ManaPoolModerate, 1),
    ],
    // Rank 9
    vec![
      (ModifierClass::ManaPoolMinor, 2),
      (ModifierClass::ManaPoolModerate, 1),
    ],
    // Rank 10
    vec![
      (ModifierClass::ManaPoolMinor, 1),
      (ModifierClass::ManaPoolModerate, 1),
    ],
    // Rank 11
    vec![
      (ModifierClass::ManaPoolMinor, 1),
      (ModifierClass::ManaPoolModerate, 1),
    ],
    // Rank 12
    vec![
      (ModifierClass::ManaPoolMinor, 2),
      (ModifierClass::ManaPoolModerate, 1),
    ],
    // Rank 13
    vec![
      (ModifierClass::ManaPoolMinor, 1),
      (ModifierClass::ManaPoolModerate, 1),
      (ModifierClass::ManaPoolMajor, 1),
    ],
    // Rank 14
    vec![
      (ModifierClass::ManaPoolMinor, 2),
      (ModifierClass::ManaPoolModerate, 1),
      (ModifierClass::ManaPoolMajor, 1),
    ],
    // Rank 15
    vec![
      (ModifierClass::ManaPoolMinor, 1),
      (ModifierClass::ManaPoolModerate, 2),
      (ModifierClass::ManaPoolMajor, 1),
    ],
    // Rank 16
    vec![
      (ModifierClass::ManaPoolMinor, 1),
      (ModifierClass::ManaPoolModerate, 1),
      (ModifierClass::ManaPoolMajor, 1),
    ],
    // Rank 17
    vec![
      (ModifierClass::ManaPoolMinor, 2),
      (ModifierClass::ManaPoolModerate, 1),
      (ModifierClass::ManaPoolMajor, 1),
    ],
    // Rank 18
    vec![
      (ModifierClass::ManaPoolMinor, 1),
      (ModifierClass::ManaPoolModerate, 2),
      (ModifierClass::ManaPoolMajor, 1),
    ],
  ];
}
