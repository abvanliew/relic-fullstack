mod build_attributes;
mod build_common;
mod build_development;
mod build_equipment;
mod build_expertise;
mod build_features;
mod build_panel;
mod build_paths;
mod character_build;

use std::collections::HashSet;

use bson::oid::ObjectId;
pub use build_panel::CharacterBuildPanel;

use crate::builder::build_development::Development;
use crate::builder::build_equipment::ActiveEquipment;
use crate::modifiers::ModifierSet;
use crate::progression::fixed::MAX_LEVEL;
use crate::progression::prelude::LevelTrack;

use self::build_attributes::AttributeRanks;
use self::character_build::SkillRanks;

#[derive(Debug, Clone)]
pub struct CharacterBuild {
  name: String,
  current_level_index: usize,
  level_selections: Vec<LevelSelections>,
}

impl Default for CharacterBuild {
  fn default() -> Self {
    let level_selections = (1..=MAX_LEVEL)
      .map(|level| LevelSelections::new(level))
      .collect();
    Self {
      name: Default::default(),
      current_level_index: Default::default(),
      level_selections,
    }
  }
}

impl CharacterBuild {
  fn get_current_mut(&mut self) -> &mut LevelSelections {
    if self.level_selections.len() < self.current_level_index + 1 {
      for index in self.level_selections.len()..=self.current_level_index {
        self
          .level_selections
          .push(LevelSelections::new(index as i32 + 1));
      }
    }
    return &mut self.level_selections[self.current_level_index];
  }

  pub fn current_selection_ref(&self) -> Option<&LevelSelections> {
    return self.level_selections.get(self.current_level_index);
  }

  pub fn current_selection(&self) -> LevelSelections {
    return self.current_selection_ref().cloned().unwrap_or_default();
  }

  fn previous_level_selections(&self) -> impl Iterator<Item = &LevelSelections> {
    let index = self.current_level_index.min(self.level_selections.len());
    return self.level_selections[..index].iter();
  }

  pub fn get_character_name(&self) -> String {
    return self.name.clone();
  }

  pub fn set_character_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn get_level(&self) -> i32 {
    self.current_level_index as i32 + 1
  }

  pub fn set_level(&mut self, level: i32) {
    self.current_level_index = (level - 1) as usize;
  }

  fn get_previous_paths(&self) -> HashSet<ObjectId> {
    let mut paths = HashSet::new();
    for selections in self.previous_level_selections() {
      paths.extend(selections.paths.clone());
    }
    return paths;
  }

  fn get_selection_pair(&self) -> (LevelSelections, LevelSelections) {
    let mut previous_selections = LevelSelections::default();
    for selection in self.previous_level_selections() {
      previous_selections.extend(selection);
    }
    let mut current_selections = previous_selections.clone();
    current_selections.extend(&self.current_selection());
    return (previous_selections, current_selections);
  }
}

#[derive(Debug, Clone, Default)]
pub struct LevelSelections {
  level: i32,
  paths: HashSet<ObjectId>,
  development: Development,
  skill_ranks: SkillRanks,
  attributes: AttributeRanks,
  equipment: ActiveEquipment,
}

impl LevelSelections {
  pub fn new(level: i32) -> Self {
    return Self {
      level,
      ..Self::default()
    };
  }

  pub fn extend(&mut self, other: &Self) {
    self.level = self.level.max(other.level);
    self.paths.extend(other.paths.clone());
    self.development.extend(&other.development);
    self.skill_ranks.extend(&other.skill_ranks);
    self.attributes.extend(&other.attributes);
    self.equipment.extend(&other.equipment);
  }

  pub fn get_modifiers(&self) -> ModifierSet {
    let mut all_modifiers = ModifierSet::default();
    all_modifiers.append(&self.development.get_modifiers());
    all_modifiers.append(&LevelTrack::as_of(self.level));

    return all_modifiers;
  }
}
