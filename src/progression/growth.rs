use crate::rule::prelude::Tier;

#[derive(Clone, Debug)]
pub struct CharacterGrowth {
  pub levels: Vec<LevelGrowth>,
}

impl CharacterGrowth {
  pub fn stats( &self, level: usize ) -> LevelStats {
    let mut stats = LevelStats::default();
    let mut tier = Tier::Initiate;
    let mut count = 1;
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
        tier_stats.path_min += value.try_into().unwrap_or( 0 );
        tier_stats.path_max += value.try_into().unwrap_or( 0 );
      }
      if let Some( value ) = level_growth.path_features {
        tier_stats.path_max += value.try_into().unwrap_or( 0 );
        tier_stats.features += value.try_into().unwrap_or( 0 );
      }
      if let Some( value ) = level_growth.features { tier_stats.features += value; }
      if let Some( value ) = level_growth.half_features { tier_stats.half_features += value; }
      if count >= level { break; }
      count += 1;
    }
    return stats;
  }

  pub fn get_path_qualifiers( &self, level: usize ) -> ( u8, u8 ) {
    let mut paths: u8 = 0;
    let mut path_features: u8 = 0;
    let mut tier = Tier::Initiate;
    let mut count = 1;
    for level_growth in self.levels.clone() {
      if let Some( value ) = level_growth.tier { tier = value; }
      match tier {
        Tier::Initiate => {
          if let Some( value ) = level_growth.paths { paths += value; }
          if let Some( value ) = level_growth.path_features { path_features += value; }
        },
        Tier::Journeyman => { break; },
        Tier::Master => { break; },
      };
      if count >= level { break; }
      count += 1;
    }
    return ( paths, path_features );
  }
}

#[derive(Default, Clone, Debug)]
pub struct LevelGrowth {
  pub tier: Option<Tier>,
  pub hp: Option<u32>,
  pub max_ranks: Option<u32>,
  pub attributes: Option<u32>,
  pub expertise: Option<u32>,
  pub paths: Option<u8>,
  pub path_features: Option<u8>,
  pub features: Option<usize>,
  pub half_features: Option<usize>,
  pub capstones: Option<usize>,
}

#[derive(Default, Clone, Debug)]
pub struct LevelStats {
  pub hp: u32,
  pub max_ranks: u32,
  pub attributes: u32,
  pub expertise: u32,
  pub capstones: usize,
  pub iniatite: TierStats,
  pub journeyman: TierStats,
  pub master: TierStats,
}

#[derive(Default, Clone, Debug)]
pub struct TierStats {
  pub path_min: usize,
  pub path_max: usize,
  pub features: usize,
  pub half_features: usize,
}
