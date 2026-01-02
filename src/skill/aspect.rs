use super::Skill;
use crate::keyword::prelude::*;
use crate::path::prelude::{PathFilter, PathSelectionClass};
use crate::rules::prelude::*;
use bson::oid::ObjectId;
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
  pub fn feature_weight(&self) -> i32 {
    match &self.training_cost {
      TrainingCost::Inherient | TrainingCost::Keystone => 0,
      TrainingCost::Full | TrainingCost::Spell => 2,
      TrainingCost::Half | TrainingCost::Cantrip => 1,
    }
  }

  pub fn is_ranked(&self) -> bool {
    match &self.ranked {
      Some( true ) => true,
      _ => false,
    }
  }

  pub fn is_match( &self, path_filter: &PathFilter, selection_class: &PathSelectionClass ) -> bool {
    if !self.is_path_match( path_filter ) { return false }
    return self.is_selection_match( selection_class );
  }

  pub fn is_path_match( &self, path_filter: &PathFilter ) -> bool {
    return match ( path_filter, &self.paths ) {
      ( PathFilter::All, _ ) => { true },
      ( PathFilter::Single( path_id ), Some( paths) ) => {
        for path in paths.iter() {
          if path.to_string().eq( path_id ) {
            return true
          }
        }
        false
      },
      ( PathFilter::Single( _ ), _ ) => { false },
    }
  }

  pub fn is_selection_match( &self, selection_class: &PathSelectionClass ) -> bool {
    match ( selection_class, &self.training_cost, &self.core ) {
      (
        PathSelectionClass::Features, 
        TrainingCost::Full | TrainingCost::Half | TrainingCost::Spell | TrainingCost::Cantrip, 
        _
      ) | ( 
        PathSelectionClass::MinorFeatures, 
        TrainingCost::Half | TrainingCost::Cantrip,
        _  
      ) | (
        PathSelectionClass::CoreFeatures, 
        TrainingCost::Full | TrainingCost::Half | TrainingCost::Spell | TrainingCost::Cantrip, 
        Some( true ),
      ) | (
        PathSelectionClass::CoreMinorFeatures, 
        TrainingCost::Half | TrainingCost::Cantrip, 
        Some( true ),
      ) | (
        PathSelectionClass::Spells, 
        TrainingCost::Spell | TrainingCost::Cantrip, 
        _,
      ) | (
        PathSelectionClass::Cantrips, 
        TrainingCost::Cantrip, 
        _,
      ) => { return true },
      _ => { return false },
    }
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
  pub rules: RulesBlocks,
}

impl Property {
  pub fn get_keyword_ids(&self) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();

    if let Some(id) = self.term.keyword_id {
      ids.insert(id);
    }
    for rule in &self.rules {
      ids.extend(rule.get_keyword_ids());
    }
    return ids;
  }
}

