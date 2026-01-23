mod display;
mod operator;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::modifiers::prelude::Bonus;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Default)]
pub struct ModifierSet(HashMap<ModifierClass, Bonus<i32>>);

impl ModifierSet {
  pub fn get(&self, class: &ModifierClass) -> i32 {
    let ModifierSet(map) = self;
    return map.get(class).unwrap_or(&Bonus::<i32>::default()).value();
  }

  pub fn add(&mut self, class: &ModifierClass, value: Bonus<i32>) {
    let ModifierSet(ref mut map) = self;
    let entry = map.entry(class.clone()).or_default();
    *entry = entry.clone() + value;
  }

  pub fn add_bonus(&mut self, class: &ModifierClass, value: i32) {
    self.add(class, Bonus::from_bonus(value));
  }

  pub fn append(&mut self, rhs: &ModifierSet) {
    let ModifierSet(rhs_map) = rhs;
    for (class, value) in rhs_map {
      self.add(class, value.clone());
    }
  }

  pub fn contains_key(&self, class: &ModifierClass) -> bool {
    return self.0.contains_key(class);
  }

  pub fn len(&self) -> usize {
    return self.0.len();
  }

  pub fn multiple(&self, multiplier: i32) -> Self {
    let mut new_map = self.0.clone();
    for value in new_map.values_mut() {
      value.multiplier(multiplier);
    }
    return Self(new_map);
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ModifierClass {
  HP,
  Constituion,
  RankMax,
  AttributeRank,
  CapabilityMaxRank,
  DefenseMaxRank,
  ExpertiseRank,
  ExpertiseMaxRank,
  Feature,
  MinorFeature,
  InitiatePathMin,
  InitiatePathMax,
  GrowthRanks,
  WalkingSpeed,
  DashSpeed,
  AnointmentPool,
  AnimalismPool,
  SanguinePool,
  RagePool,
  InnatePool,
  InnatePoolAll,
  InnateFlow,
  MasteryPool,
  ChannelPool,
  KiPool,
  VirtuosoPool,
  ResonancePool,
  ResonancePoolAll,
  ResonanceFlow,
  ManaPoolMinor,
  ManaPoolModerate,
  ManaPoolMajor,
  MagicFlow,
}

pub mod prelude {
  pub use super::operator::Bonus;
  pub use super::{ModifierClass, ModifierSet};
}
