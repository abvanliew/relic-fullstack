use std::fmt::Display;

use serde::{Deserialize, Serialize};
use super::Path;

use self::SkillFilter::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SelectionFilter {
  pub path_filter: PathFilter,
  pub skill_filter: SkillFilter,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum PathFilter {
  #[default]
  All,
  Single( String ),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum SkillFilter {
  #[default]
  Features,
  CoreFeatures,
  MinorFeatures,
  CoreMinorFeatures,
  Spells,
  Cantrips,
}

impl Display for SkillFilter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        SkillFilter::Features => "Features",
        SkillFilter::CoreFeatures => "Core Features",
        SkillFilter::MinorFeatures => "Minor Features",
        SkillFilter::CoreMinorFeatures => "Core Minor Features",
        SkillFilter::Spells => "Spells",
        SkillFilter::Cantrips => "Cantrips",
      }
    )
  }
}

impl SkillFilter {
  pub fn weight( &self ) ->  i32 {
    match self {
      Features | CoreFeatures | Spells => 2,
      MinorFeatures | CoreMinorFeatures | Cantrips => 1,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
  pub filter: SelectionFilter,
  pub required_weight: i32,
  pub overages: i32,
}

impl Default for Constraint {
  fn default() -> Self {
    Self {
      filter: SelectionFilter::default(),
      required_weight: 0,
      overages: 0
    }
  }
}

impl Constraint {
  pub fn feature( required_weight: i32 ) -> Self {
    Self {
      filter: SelectionFilter {
        skill_filter: SkillFilter::Features,
        ..SelectionFilter::default()
      },
      required_weight,
      ..Default::default()
    }
  }
  
  pub fn minor_feature( required_weight: i32 ) -> Self {
    Self {
      filter: SelectionFilter {
        skill_filter: SkillFilter::MinorFeatures,
        ..SelectionFilter::default()
      },
      required_weight,
      ..Default::default()
    }
  }
}

impl Path {
  pub fn selection_constraints(&self) -> (Vec<Constraint>, i32) {
    let selections = self.selections.clone().unwrap_or_default();
    let mut constraints = Vec::<Constraint>::new();
    let mut constraint_weights = 0;
    for (skill_filter, ranks) in selections {
      let required_weight = skill_filter.weight() * ranks;
      constraint_weights += required_weight;
      constraints.push( Constraint {
        filter: SelectionFilter {
          path_filter: PathFilter::Single( self.id.to_string() ),
          skill_filter: skill_filter.clone(),
        },
        required_weight,
        ..Default::default()
      } );
    }
    return (constraints, constraint_weights);
  }
}
