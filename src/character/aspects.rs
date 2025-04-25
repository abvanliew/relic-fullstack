// use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{flow::Flow, ResourcePool};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TrainingRanks {
  pub expert: Option<i32>,
  pub adept: Option<i32>,
  pub endurance: Option<i32>,
  pub innate: Option<i32>,
  pub resonnance: Option<i32>,
  pub magic: Option<i32>,
}

impl fmt::Display for TrainingRanks {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    let mut ranks: Vec<String> = Vec::new();
    if let Some( expert ) = self.expert { ranks.push( format!( "Expert {expert}" ) ); }
    if let Some( adept ) = self.adept { ranks.push( format!( "Adept {adept}" ) ); }
    if let Some( endurance ) = self.endurance { ranks.push( format!( "Endurance {endurance}" ) ); }
    if let Some( innate ) = self.innate { ranks.push( format!( "Innate {innate}" ) ); }
    if let Some( resonnance ) = self.resonnance { ranks.push( format!( "Resonnance {resonnance}" ) ); }
    if let Some( magic ) = self.magic { ranks.push( format!( "Magic {magic}" ) ); }
    write!( f, "{}", ranks.join( ", " ) )
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BodyStats {
  pub hp: i32,
  pub constitution: i32,
  pub speed: i32,
}
