mod display;
mod operator;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{modifiers::prelude::Bonus, skill::prelude::ResourcePool};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Default)]
pub struct ModifierSet(HashMap<ModifierClass, Bonus<i32>>);

impl ModifierSet {
  pub fn from_bonuses(modifiers: Vec<(ModifierClass, i32)>) -> Self {
    let mut modifier_map: HashMap<ModifierClass, Bonus<i32>> = HashMap::new();
    for (class, value) in modifiers {
      modifier_map.insert(class, Bonus::from_bonus(value));
    }
    return Self(modifier_map);
  }

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
  SpecializationMax,
  AttributeRank,
  CapabilityRank,
  CapabilitySpecialization,
  DefenseRank,
  DefenseSpecialization,
  ExpertiseRank,
  ExpertiseSpecialization,
  Feature,
  MinorFeature,
  InitiatePathRequired,
  InitiatePathMax,
  InitiatePathOptional,
  JourneymanPathRequired,
  JourneymanPathOptional,
  MasterPathRequired,
  MasterPathOptional,
  DevelopmentPoints,
  WalkingSpeed,
  DashSpeed,
  AnimismPool,
  AnointmentPool,
  RagePool,
  SanguinePool,
  InnatePool,
  InnatePoolAll,
  InnateFlow,
  ChannelPool,
  KiPool,
  MasteryPool,
  VirtuosoPool,
  ResonancePool,
  ResonancePoolAll,
  ResonanceFlow,
  ManaPool,
  ManaPoolAll,
  ManaPoolMinor,
  ManaPoolModerate,
  ManaPoolMajor,
  MagicFlow,
}

impl ModifierClass {
  pub fn innate_pool_iter<'a>() -> impl Iterator<Item = &'a (Self, ResourcePool)> {
    return [
      (Self::AnimismPool, ResourcePool::Animism),
      (Self::AnointmentPool, ResourcePool::Anointment),
      (Self::RagePool, ResourcePool::Rage),
      (Self::VirtuosoPool, ResourcePool::Sanguine),
    ]
    .iter();
  }

  pub fn resonance_pool_iter<'a>() -> impl Iterator<Item = &'a (Self, ResourcePool)> {
    return [
      (Self::ChannelPool, ResourcePool::Channel),
      (Self::KiPool, ResourcePool::Ki),
      (Self::MasteryPool, ResourcePool::Mastery),
      (Self::VirtuosoPool, ResourcePool::Virtuoso),
    ]
    .iter();
  }

  pub fn magic_pool_iter<'a>() -> impl Iterator<Item = &'a (Self, ResourcePool)> {
    return [
      (Self::ManaPoolMinor, ResourcePool::MinorMana),
      (Self::ManaPoolModerate, ResourcePool::ModerateMana),
      (Self::ManaPoolMajor, ResourcePool::MajorMana),
    ]
    .iter();
  }
}

pub mod prelude {
  pub use super::operator::Bonus;
  pub use super::{ModifierClass, ModifierSet};
}
