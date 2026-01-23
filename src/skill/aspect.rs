use super::Skill;
use crate::keyword::prelude::*;
use crate::path::prelude::*;
use crate::rules::prelude::*;
use crate::server::prelude::KeywordCache;
use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd, Eq)]
pub enum TrainingCost {
  Inherient,
  Keystone,
  Full,
  Half,
  Cantrip,
  Spell,
}

impl fmt::Display for TrainingCost {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        TrainingCost::Inherient => "Inherient",
        TrainingCost::Full => "Feature",
        TrainingCost::Half => "Minor Feature",
        TrainingCost::Keystone => "Keystone",
        TrainingCost::Cantrip => "Cantrip",
        TrainingCost::Spell => "Spell",
      }
    )
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, PartialOrd, Ord, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelicOrdering {
  category: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
  id: ObjectId,
  title: String,
}

impl Skill {
  pub fn weight(&self) -> i32 {
    match &self.training_cost {
      TrainingCost::Full | TrainingCost::Spell => 2,
      TrainingCost::Half | TrainingCost::Cantrip => 1,
      TrainingCost::Inherient | TrainingCost::Keystone => 0,
    }
  }

  pub fn resource_cost(&self) -> i32 {
    self.action.get_minimum_resource_cost()
  }

  pub fn is_ranked(&self) -> bool {
    match &self.ranked {
      Some(true) => true,
      _ => false,
    }
  }

  pub fn is_core(&self) -> bool {
    return self.core.unwrap_or_default();
  }

  pub fn is_match(&self, filter: &SelectionFilter) -> bool {
    return self.is_path_match(&filter.path_filter) && self.is_skill_match(&filter.skill_filter);
  }

  pub fn is_path_match(&self, path_filter: &PathFilter) -> bool {
    return match (path_filter, &self.paths) {
      (PathFilter::All, _) => true,
      (PathFilter::Single(path_id), Some(paths)) => {
        for path in paths.iter() {
          if path.to_string().eq(path_id) {
            return true;
          }
        }
        false
      },
      (PathFilter::Single(_), _) => false,
    };
  }

  pub fn is_skill_match(&self, skill_filter: &SkillFilter) -> bool {
    return match (skill_filter, &self.training_cost, &self.is_core()) {
      (
        SkillFilter::Features,
        TrainingCost::Full | TrainingCost::Spell | TrainingCost::Half | TrainingCost::Cantrip,
        _,
      )
      | (SkillFilter::MinorFeatures, TrainingCost::Half | TrainingCost::Cantrip, _)
      | (
        SkillFilter::CoreFeatures,
        TrainingCost::Full | TrainingCost::Spell | TrainingCost::Half | TrainingCost::Cantrip,
        true,
      )
      | (SkillFilter::CoreMinorFeatures, TrainingCost::Half | TrainingCost::Cantrip, true)
      | (SkillFilter::Spells, TrainingCost::Spell, _)
      | (SkillFilter::Cantrips, TrainingCost::Cantrip, _) => true,
      _ => false,
    };
  }
}

impl KeywordClassified for Skill {
  fn get_keyword_ids(&self) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    ids.extend(self.action.get_keyword_ids());
    if let Some(sub_actions) = &self.sub_actions {
      for action in sub_actions {
        ids.extend(action.get_keyword_ids());
      }
    }
    return ids;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Property {
  pub term: Term,
  pub rules: Option<RulesBlocks>,
  pub block: Option<bool>,
}

impl Property {
  pub fn get_keyword_ids(&self) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    if let Some(id) = self.term.keyword_id {
      ids.insert(id);
    }
    if let Some(rules) = &self.rules {
      for rule in rules {
        ids.extend(rule.get_keyword_ids());
      }
    }
    return ids;
  }

  pub fn get_title_and_blocks(&self) -> (String, RulesBlocks) {
    let title = self.term.to_title();
    let blocks = match (&self.rules, self.term.keyword_id) {
      (Some(rules), _) => rules.clone(),
      (_, Some(keyword_id)) => {
        let KeywordCache(ref keyword_cache) = use_context();
        match keyword_cache.from_object_id(&keyword_id) {
          Some(keyword) => keyword.blocks(),
          None => Vec::new(),
        }
      },
      _ => Vec::new(),
    };
    return (title, blocks);
  }
}
