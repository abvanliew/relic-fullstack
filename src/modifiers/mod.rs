mod display;
mod operator;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::modifiers::prelude::Modifier;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Default)]
pub struct ModifierSet(HashMap<ModifierClass, Modifier<i32>>);

impl ModifierSet {
  pub fn get(&self, class: &ModifierClass) -> i32 {
    let ModifierSet(map) = self;
    return map
      .get(class)
      .unwrap_or(&Modifier::<i32>::default())
      .value();
  }

  pub fn add(&mut self, class: &ModifierClass, value: Modifier<i32>) {
    let ModifierSet(ref mut map) = self;
    let entry = map.entry(class.clone()).or_default();
    *entry = entry.clone() + value;
  }

  pub fn add_bonus(&mut self, class: &ModifierClass, value: i32) {
    self.add(class, Modifier::from_bonus(value));
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

  pub fn multiple( &self, multiplier: i32 ) -> Self {
    let mut new_map = self.0.clone();
    for value in new_map.values_mut() {
      value.multiplier( multiplier );
    }
    return Self( new_map );
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
pub enum ModifierClass {
  HP,
  RankMax,
  AttributeRank,
  CapabilityMaxRank,
  DefenseMaxRank,
  ExpertiseRank,
  ExpertiseMaxRank,
  Feature,
  MinorFeature,
  PathMin,
  PathMax,
  GrowthRanks,
  InnateFlow,
  InnatePool,
  InnatePoolAll,
  AnointmentPool,
  AnimalismPool,
  SanguinePool,
  RagePool,
  ResonanceFlow,
  ResonancePool,
  ResonancePoolAll,
  MasteryPool,
  ChannelPool,
  KiPool,
  VirtuosoPool,
  MagicFlow,
  MagicPool,
  MagicPoolAll,
  MinorManaPool,
  ModerateManaPool,
  MajorManaPool,
}

pub mod prelude {
  pub use super::operator::{opt_max, opt_sum, Modifier};
  pub use super::{ModifierClass, ModifierSet};
}
