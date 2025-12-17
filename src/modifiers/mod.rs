mod aggregators;
mod display;
mod operator;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::modifiers::prelude::Modifier;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Default)]
pub struct ModifierSet(HashMap<ModifierClass, Modifier<u32>>);

impl ModifierSet {
  pub fn get(&self, class: &ModifierClass) -> u32 {
    let ModifierSet(map) = self;
    return map
      .get(class)
      .unwrap_or(&Modifier::<u32>::default())
      .value();
  }

  pub fn add(&mut self, class: &ModifierClass, value: Modifier<u32>) {
    let ModifierSet(ref mut map) = self;
    let entry = map.entry(class.clone()).or_default();
    *entry = entry.clone() + value;
  }

  pub fn add_bonus(&mut self, class: &ModifierClass, value: u32) {
    self.add(class, Modifier::from_bonus(value));
  }

  pub fn append(&mut self, rhs: &ModifierSet) {
    let ModifierSet(rhs_map) = rhs;
    for (class, value) in rhs_map {
      self.add(class, value.clone());
    }
  }

  pub fn contains_key(&self, class: &ModifierClass) -> bool {
    let ModifierSet(map) = self;
    return map.contains_key(class);
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
pub enum ModifierClass {
  HP,
  CapabilityRank,
  CapabilityMaxRank,
  DefenseRank,
  DefenseMaxRank,
  ExpertiseRank,
  ExpertiseMaxRank,
  PathMin,
  PathMax,
  GrowthRanks,
  InnateFlow,
  InnatePool,
  InnatePoolAll,
  ResonanceFlow,
  ResonancePool,
  ResonancePoolAll,
  MagicFlow,
  MagicPool,
  MagicPoolAll,
}

pub mod prelude {
  pub use super::aggregators::modifiers_from_skills;
  pub use super::operator::{opt_max, opt_sum, Modifier};
  pub use super::{ModifierClass, ModifierSet};
}
