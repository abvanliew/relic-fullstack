mod build_attributes;
mod build_development;
mod build_features;
mod build_panel;
mod build_paths;
mod character_build;
mod common;

use std::collections::HashSet;

use bson::oid::ObjectId;
pub use build_panel::CharacterBuildPanel;

use crate::builder::build_development::Development;

use self::build_attributes::AttributeRanks;
use self::character_build::SkillRanks;

#[derive(Debug, Clone, Default)]
pub struct CharacterBuild {
  name: String,
  current_level_index: usize,
  level_selections: Vec<LevelSelections>,
}

impl CharacterBuild {
  fn get_current_mut(&mut self) -> &mut LevelSelections {
    if self.level_selections.len() < self.current_level_index + 1 {
      for _ in self.level_selections.len()..=self.current_level_index {
        self.level_selections.push(LevelSelections::default());
      }
    }
    return &mut self.level_selections[self.current_level_index];
  }

  pub fn get_character_name(&self) -> String {
    return self.name.clone();
  }

  pub fn set_character_name(&mut self, name: String) {
    self.name = name;
  }
}

#[derive(Debug, Clone, Default)]
pub struct LevelSelections {
  paths: HashSet<ObjectId>,
  development: Development,
  skill_ranks: SkillRanks,
  attributes: AttributeRanks,
}
