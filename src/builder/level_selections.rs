use std::collections::HashMap;
use std::{collections::HashSet, ops::RangeInclusive};

use bson::oid::ObjectId;
use dioxus::prelude::*;

use crate::modifiers::{ModifierClass, ModifierSet};
use crate::progression::prelude::{GrowthTrack, LevelTrack, TrainingClass};
use crate::rules::prelude::Tier;
use crate::server::prelude::PathCache;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionValidity {
  Available,
  Minimal,
  Full,
  Invalid,
}

#[derive(Debug, Clone)]
pub enum SelectionStatus {
  Unselected,
  SelectedPreviously,
  SelectedCurrently,
}

#[derive(Debug, Clone, Default)]
pub struct CharacterBuild {
  current_level_index: usize,
  level_selections: Vec<LevelSelections>,
}

impl CharacterBuild {
  fn previous_index_range(&self) -> Option<RangeInclusive<usize>> {
    if self.current_level_index == 0 || self.level_selections.len() == 0 {
      return None;
    }
    let index = (self.current_level_index - 1).min(self.level_selections.len() - 1);
    return Some(0..=index);
  }

  pub fn get_level(&self) -> i32 {
    self.current_level_index as i32 + 1
  }

  pub fn set_level(&mut self, level: i32) {
    self.current_level_index = (level - 1) as usize;
  }

  pub fn current_selection_ref(&self) -> Option<&LevelSelections> {
    return self.level_selections.get(self.current_level_index);
  }

  pub fn path_selection_status(&self, path_id: &ObjectId) -> SelectionStatus {
    if let Some(current_selection) = self.current_selection_ref() {
      match current_selection.paths.get(path_id) {
        Some(_) => {
          return SelectionStatus::SelectedCurrently;
        },
        _ => (),
      }
    };
    return match self.get_previous_paths().get(path_id) {
      Some(_) => SelectionStatus::SelectedPreviously,
      None => SelectionStatus::Unselected,
    };
  }

  fn get_previous_paths(&self) -> HashSet<ObjectId> {
    let mut paths = HashSet::new();
    if let Some(range) = self.previous_index_range() {
      for i in range {
        let selections = &self.level_selections[i];
        paths.extend(selections.paths.clone());
      }
    }
    return paths;
  }

  fn get_current_mut(&mut self) -> &mut LevelSelections {
    if self.level_selections.len() < self.current_level_index + 1 {
      for _ in self.level_selections.len()..=self.current_level_index {
        self.level_selections.push(LevelSelections::default());
      }
    }
    return &mut self.level_selections[self.current_level_index];
  }

  pub fn add_path(&mut self, path_id: ObjectId) {
    let current = self.get_current_mut();
    current.add_path(path_id);
  }

  pub fn remove_path(&mut self, path_id: &ObjectId) {
    let current = self.get_current_mut();
    current.remove_path(path_id);
  }

  pub fn get_current_paths(&self) -> HashSet<ObjectId> {
    let mut paths = self.get_previous_paths();
    let Some(current_selection) = self.current_selection_ref() else {
      return paths;
    };
    paths.extend(current_selection.paths.clone());
    return paths;
  }

  pub fn get_path_validation_status(&self) -> (SelectionValidity, Counter, Counter) {
    let level_stats = LevelTrack::as_of(self.get_level());
    let PathCache(path_cache) = use_context();
    let path_ids = self.get_current_paths();
    let paths = path_cache.from_object_set(&path_ids);
    let mut initiate_count: i32 = 0;
    for path in paths {
      if path.tier == Tier::Initiate {
        initiate_count += 1;
      }
    }
    let initiate_required = level_stats.get(&ModifierClass::InitiatePathRequired);
    let initiate_optional = level_stats.get(&ModifierClass::InitiatePathOptional);
    let mut counter_required = Counter::from_max(initiate_required);
    let mut counter_initiate = Counter::from_max(initiate_optional);
    if initiate_count <= initiate_required {
      counter_required.min += initiate_count;
    }
    if initiate_count > initiate_required {
      counter_required.min += initiate_required;
      counter_initiate.min += initiate_count - initiate_required;
    }
    return (
      match (counter_required.valid(), counter_initiate.valid()) {
        (SelectionValidity::Available, _) => SelectionValidity::Available,
        (SelectionValidity::Full, SelectionValidity::Available) => SelectionValidity::Minimal,
        (SelectionValidity::Full, SelectionValidity::Full) => SelectionValidity::Full,
        _ => SelectionValidity::Invalid,
      },
      counter_required,
      counter_initiate,
    );
  }

  pub fn get_previous_trainings(&self) -> Training {
    let mut previous_trainings = Training::default();
    if let Some(range) = self.previous_index_range() {
      for i in range {
        let selections = &self.level_selections[i];
        previous_trainings.extend(&selections.trainings);
      }
    }
    return previous_trainings;
  }

  pub fn get_current_trainings(&self) -> Training {
    let mut previous_training = self.get_previous_trainings();
    let Some(level_selection) = self.current_selection_ref() else {
      return previous_training;
    };
    previous_training.extend(&level_selection.trainings);
    return previous_training;
  }

  pub fn get_training_ranks(&self) -> i32 {
    let level_stats = LevelTrack::as_of(self.get_level());
    return level_stats.get(&ModifierClass::GrowthRanks);
  }

  pub fn set_training(&mut self, class: &TrainingClass, value: i32) {
    let current = self.get_current_mut();
    current.trainings.set(class, value)
  }

  pub fn get_training_modifiers(&self) -> ModifierSet {
    let trainings = self.get_current_trainings();
    let mut modifiers = ModifierSet::default();
    modifiers.append(&GrowthTrack::class_at(
      &TrainingClass::Adept,
      trainings.adept.unwrap_or(0),
    ));
    modifiers.append(&GrowthTrack::class_at(
      &TrainingClass::Endurance,
      trainings.endurance.unwrap_or(0),
    ));
    modifiers.append(&GrowthTrack::class_at(
      &TrainingClass::Expert,
      trainings.expert.unwrap_or(0),
    ));
    modifiers.append(&GrowthTrack::class_at(
      &TrainingClass::Innate,
      trainings.innate.unwrap_or(0),
    ));
    modifiers.append(&GrowthTrack::class_at(
      &TrainingClass::Resonance,
      trainings.resonant.unwrap_or(0),
    ));
    modifiers.append(&GrowthTrack::class_at(
      &TrainingClass::Magic,
      trainings.magic.unwrap_or(0),
    ));
    return modifiers;
  }
}

#[derive(Debug, Clone, Default)]
pub struct LevelSelections {
  paths: HashSet<ObjectId>,
  trainings: Training,
  skill_ranks: HashMap<ObjectId, i32>,
}

impl LevelSelections {
  pub fn add_path(&mut self, path_id: ObjectId) {
    self.paths.insert(path_id);
  }

  pub fn remove_path(&mut self, path_id: &ObjectId) {
    self.paths.remove(path_id);
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
  pub min: i32,
  pub max: i32,
}

impl Counter {
  pub fn from_max(max: i32) -> Self {
    Self { min: 0, max }
  }

  pub fn valid(&self) -> SelectionValidity {
    if self.min < self.max {
      return SelectionValidity::Available;
    }
    if self.min == self.max {
      return SelectionValidity::Full;
    }
    return SelectionValidity::Invalid;
  }
}

#[derive(Debug, Clone, Default)]
pub struct Training {
  pub adept: Option<i32>,
  pub endurance: Option<i32>,
  pub expert: Option<i32>,
  pub innate: Option<i32>,
  pub resonant: Option<i32>,
  pub magic: Option<i32>,
}

impl Training {
  pub fn extend(&mut self, other: &Self) {
    self.adept = optional_max(&self.adept, &other.adept);
    self.endurance = optional_max(&self.endurance, &other.endurance);
    self.expert = optional_max(&self.expert, &other.expert);
    self.innate = optional_max(&self.innate, &other.innate);
    self.resonant = optional_max(&self.resonant, &other.resonant);
    self.magic = optional_max(&self.magic, &other.magic);
  }

  pub fn set(&mut self, class: &TrainingClass, value: i32) {
    match class {
      TrainingClass::Expert => self.expert = Some(value),
      TrainingClass::Adept => self.adept = Some(value),
      TrainingClass::Endurance => self.endurance = Some(value),
      TrainingClass::Innate => self.innate = Some(value),
      TrainingClass::Resonance => self.resonant = Some(value),
      TrainingClass::Magic => self.magic = Some(value),
    }
  }

  pub fn sum(&self) -> i32 {
    return self.adept.unwrap_or(0)
      + self.endurance.unwrap_or(0)
      + self.expert.unwrap_or(0)
      + self.innate.unwrap_or(0)
      + self.resonant.unwrap_or(0)
      + self.magic.unwrap_or(0);
  }

  pub fn summary(&self) -> String {
    let items: Vec<String> = vec![
      option_formater("Adept".into(), &self.adept),
      option_formater("Endurance".into(), &self.endurance),
      option_formater("Expert".into(), &self.expert),
      option_formater("Innate".into(), &self.innate),
      option_formater("Resonnance".into(), &self.resonant),
      option_formater("Magic".into(), &self.magic),
    ]
    .into_iter()
    .flatten()
    .collect();
    return items.join(", ");
  }
}

fn optional_max(lhs: &Option<i32>, rhs: &Option<i32>) -> Option<i32> {
  return match (lhs, rhs) {
    (None, None) => None,
    (Some(value), None) => Some(*value),
    (None, Some(value)) => Some(*value),
    (Some(left_value), Some(right_value)) => Some(*left_value.max(right_value)),
  };
}

fn option_formater(title: String, value: &Option<i32>) -> Option<String> {
  return match value {
    Some(value) => {
      if value.eq(&0) {
        None
      } else {
        Some(format!("{title} {value}"))
      }
    },
    None => None,
  };
}
