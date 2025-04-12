use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{flow::Flow, ResourcePool};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TrainingRanks {
  pub expert: i32,
  pub adept: i32,
  pub endurance: i32,
  pub innate: i32,
  pub resonnance: i32,
  pub magic: i32,
}

impl fmt::Display for TrainingRanks {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    let mut ranks: Vec<String> = Vec::new();
    if self.expert > 0 { ranks.push( format!( "Expert {}", self.expert ) ); }
    if self.adept > 0 { ranks.push( format!( "Adept {}", self.adept ) ); }
    if self.endurance > 0 { ranks.push( format!( "Endurance {}", self.endurance ) ); }
    if self.innate > 0 { ranks.push( format!( "Innate {}", self.innate ) ); }
    if self.resonnance > 0 { ranks.push( format!( "Resonnance {}", self.resonnance ) ); }
    if self.magic > 0 { ranks.push( format!( "Magic {}", self.magic ) ); }
    write!( f, "{}", ranks.join( ", " ) )
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BodyStats {
  pub hp: i32,
  pub constitution: i32,
  pub speed: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FlowStat {
  pub flow: Flow,
  pub base: i32,
  pub pools: Vec<ResourceStat>,
  pub skills: Vec<ObjectId>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ResourceStat {
  pub resource: ResourcePool,
  pub base: i32,
}
