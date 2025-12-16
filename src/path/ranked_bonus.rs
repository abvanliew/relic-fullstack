use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use dioxus::prelude::*;

use std::fmt;

use crate::{
  operator::{BonusPair, InstanceBonus, StackingBonus}, skill::prelude::*
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

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
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
  pub flow_innate: Option<ModifierPair>,
  pub flow_resonance: Option<ModifierPair>,
  pub flow_magic: Option<ModifierPair>,
}

#[component]
pub fn RankedBonusCard(
  ranked_bonuses: RankedBonuses,
  #[props(default)]
  rank: Option<u32>,
) -> Element {
  let title = ranked_bonuses.title;
  let training_cost = ranked_bonuses.training_cost;
  let modifiers = ranked_bonuses.modifiers.unwrap_or_default();
  let hp_optional = modifiers.hp;
  let attribute_rank_optional = modifiers.attribute_rank;
  let expretise_rank_optional = modifiers.expretise_rank;
  let path_skill_optional = modifiers.path_skill;
  let path_cantrip_optional = modifiers.path_cantrip;

  return rsx! {
    div {
      class: "card grid dim-keywords",
      div {
        class: "uv-title-property title",
        "{title}"
      }
      div { class: "uv-property nowrap italics", "Ranked {training_cost}" }
      div {
        class: "uv-full",
        if let Some( hp ) = hp_optional {
          span { "You gain {hp} HP. " }
        }
        if let Some( attribute_rank ) = attribute_rank_optional {
          span { "You gain {attribute_rank} Attribute ranks. " }
        }
        if let Some( expretise_rank ) = expretise_rank_optional {
          span { "You gain {expretise_rank} Attribute ranks. " }
        }
        if let Some( path_skill ) = path_skill_optional {
          span { "You learn {path_skill} skill from this path. " }
        }
        if let Some( path_cantrip ) = path_cantrip_optional {
          span { "You learn {path_cantrip} Cantrip from this path. " }
        }
      }
    }
  }
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

impl fmt::Display for ModifierPair {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return match (self.base, self.bonus) {
      ( Some( base ), Some( bonus ) ) => {
        let sum = base + bonus;
        write!( f, "{sum} (+{bonus} for each additional)" )
      },
      ( Some( base ), _ ) => write!( f, "{base}" ),
      ( _, Some( bonus ) ) => write!( f, "{bonus}" ),
      _ => write!( f, "n/a" )
    }
  }
}