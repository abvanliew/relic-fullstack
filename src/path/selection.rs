use serde::{Deserialize, Serialize};
use self::PathSelectionClass::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathFilter {
  All,
  Single( String ),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathSelectionClass {
  Features,
  CoreFeatures,
  MinorFeatures,
  CoreMinorFeatures,
  Spells,
  Cantrips,
}

impl PathSelectionClass {
  pub fn weight_multiplier( &self ) ->  i32 {
    match self {
      Features | CoreFeatures | Spells => 2,
      MinorFeatures | CoreMinorFeatures | Cantrips => 1,
    }
  }
}