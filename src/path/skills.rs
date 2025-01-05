use super::{Path, Skill, TrainingCost};

impl Path {
  pub fn skills_keystone( &self ) -> Vec<Skill> {
    self.skills.clone().unwrap_or_default().into_iter()
    .filter( |skill|
      skill.training_cost == TrainingCost::Keystone ||
      skill.training_cost == TrainingCost::Inherient
    ).collect()
  }

  pub fn skills_full( &self ) -> Vec<Skill> {
    self.skills.clone().unwrap_or_default().into_iter()
    .filter( |skill| skill.training_cost == TrainingCost::Full ).collect()
  }

  pub fn skills_half( &self ) -> Vec<Skill> {
    self.skills.clone().unwrap_or_default().into_iter()
    .filter( |skill| skill.training_cost == TrainingCost::Half ).collect()
  }

  pub fn spells( &self ) -> Vec<Skill> {
    self.skills.clone().unwrap_or_default().into_iter()
    .filter( |skill| skill.training_cost == TrainingCost::Spell ).collect()
  }

  pub fn cantrips( &self ) -> Vec<Skill> {
    self.skills.clone().unwrap_or_default().into_iter()
    .filter( |skill| skill.training_cost == TrainingCost::Cantrip ).collect()
  }
}