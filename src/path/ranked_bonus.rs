use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{
  operator::{BonusPair, InstanceBonus, StackingBonus},
  skill::prelude::*,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RankedBonuses {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub title: String,
  pub training_cost: TrainingCost,
  pub summary: Option<String>,
  pub modifiers: Option<ModifierSet>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModifierSet {
  pub hp: Option<ModifierPair>,
  pub attribute_rank: Option<ModifierPair>,
  pub capability_rank: Option<ModifierPair>,
  pub defense_rank: Option<ModifierPair>,
  pub expretise_rank: Option<ModifierPair>,
  pub path_skill: Option<ModifierPair>,
  pub path_half_skill: Option<ModifierPair>,
  pub path_spell: Option<ModifierPair>,
  pub path_cantrip: Option<ModifierPair>,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Eq)]
pub struct ModifierPair {
  pub bonus: Option<i32>,
  pub base: Option<i32>,
}

impl ModifierPair {
  pub fn to_bonus_pair(&self) -> BonusPair<i32> {
    return BonusPair {
      base: InstanceBonus(self.base),
      bonus: StackingBonus(self.bonus),
    };
  }
}
