use serde::{Deserialize, Serialize};

use crate::character::ResourcePool;

use super::TrainingCost;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
  pub training_cost: TrainingCost,
  pub bonus_hp: Option<i32>,
  pub bonus_expertise: Option<i32>,
  pub spell_knowledge: Option<SpellKnowledge>,
  pub resource_pool: Option<ResourcePool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpellKnowledge {
  pub tier: SpellTier,
  pub style: SpellStyle,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SpellTier {
  Cantrip,
  Minor,
  Moderate,
  Major,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SpellStyle {
  Slot,
  Spontaneous,
}
