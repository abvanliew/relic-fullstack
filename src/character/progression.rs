use crate::rule::prelude::Tier;

#[derive(Clone, Debug)]
pub struct CharacterGrowth {
  pub levels: Vec<LevelGrowth>,
}

impl Default for CharacterGrowth {
  fn default() -> Self {
    Self {
      levels: vec![
        // Level 1
        LevelGrowth {
          tier: Some( Tier::Initiate ),
          hp: Some( 25 ),
          max_ranks: Some( 5 ),
          attributes: Some( 19 ),
          expertise: Some( 20 ),
          paths: Some( 1 ),
          path_features: Some( 1 ),
          ..Default::default()
        },
        // Level 2
        LevelGrowth {
          hp: Some( 1 ),
          attributes: Some( 1 ),
          expertise: Some( 1 ),
          path_features: Some( 1 ),
          features: Some( 1 ),
          ..Default::default()
        },
        // Level 3
        LevelGrowth {
          hp: Some( 1 ),
          attributes: Some( 2 ),
          expertise: Some( 2 ),
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
          ..Default::default()
        },
        // Level 5
        LevelGrowth {
          hp: Some( 1 ),
          attributes: Some( 2 ),
          expertise: Some( 2 ),
          features: Some( 1 ),
          half_features: Some( 1 ),
          ..Default::default()
        },
        // Level 6
        LevelGrowth {
          hp: Some( 1 ),
          attributes: Some( 2 ),
          expertise: Some( 1 ),
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
          paths: Some( 1 ),
          features: Some( 1 ),
          ..Default::default()
        },
        // Level 8
        LevelGrowth {
          hp: Some( 1 ),
          attributes: Some( 1 ),
          expertise: Some( 1 ),
          path_features: Some( 1 ),
          half_features: Some( 1 ),
          ..Default::default()
        },
        // Level 9
        LevelGrowth {
          hp: Some( 1 ),
          attributes: Some( 2 ),
          expertise: Some( 2 ),
          path_features: Some( 1 ),
          ..Default::default()
        },
        // Level 10
        LevelGrowth {
          max_ranks: Some( 1 ),
          hp: Some( 1 ),
          attributes: Some( 1 ),
          expertise: Some( 1 ),
          features: Some( 1 ),
          ..Default::default()
        },
        // Level 11
        LevelGrowth {
          hp: Some( 1 ),
          attributes: Some( 2 ),
          expertise: Some( 2 ),
          features: Some( 1 ),
          half_features: Some( 1 ),
          ..Default::default()
        },
        // Level 12
        LevelGrowth {
          hp: Some( 1 ),
          attributes: Some( 2 ),
          expertise: Some( 1 ),
          features: Some( 1 ),
          ..Default::default()
        },
      ]
    }
  }
}

impl CharacterGrowth {
  pub fn stats( &self, level: i32 ) -> LevelStats {
    let mut stats = LevelStats::default();
    let mut tier = Tier::Initiate;
    let mut count: i32 = 1;
    for level_growth in self.levels.clone() {
      if let Some( value ) = level_growth.hp { stats.hp += value; }
      if let Some( value ) = level_growth.max_ranks { stats.max_ranks += value; }
      if let Some( value ) = level_growth.attributes { stats.attributes += value; }
      if let Some( value ) = level_growth.expertise { stats.expertise += value; }
      if let Some( value ) = level_growth.capstones { stats.capstones += value; }
      if let Some( value ) = level_growth.tier { tier = value; }
      let tier_stats = match tier {
        Tier::Initiate => &mut stats.iniatite,
        Tier::Journeyman => &mut stats.journeyman,
        Tier::Master => &mut stats.master,
      };
      if let Some( value ) = level_growth.paths {
        tier_stats.path_min += value;
        tier_stats.path_max += value;
      }
      if let Some( value ) = level_growth.path_features {
        tier_stats.path_max += value;
        tier_stats.features += value;
      }
      if let Some( value ) = level_growth.features { tier_stats.features += value; }
      if let Some( value ) = level_growth.half_features { tier_stats.half_features += value; }
      if count >= level { break; }
      count += 1;
    }
    return stats;
  }
}

#[derive(Default, Clone, Debug)]
pub struct LevelGrowth {
  pub tier: Option<Tier>,
  pub hp: Option<i32>,
  pub max_ranks: Option<i32>,
  pub attributes: Option<i32>,
  pub expertise: Option<i32>,
  pub paths: Option<i32>,
  pub path_features: Option<i32>,
  pub features: Option<i32>,
  pub half_features: Option<i32>,
  pub capstones: Option<i32>,
}

#[derive(Default, Clone, Debug)]
pub struct LevelStats {
  pub hp: i32,
  pub max_ranks: i32,
  pub attributes: i32,
  pub expertise: i32,
  pub capstones: i32,
  pub iniatite: TierStats,
  pub journeyman: TierStats,
  pub master: TierStats,
}

#[derive(Default, Clone, Debug)]
pub struct TierStats {
  pub path_min: i32,
  pub path_max: i32,
  pub features: i32,
  pub half_features: i32,
}
