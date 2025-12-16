use std::collections::HashSet;

use bson::oid::ObjectId;

use crate::keyword::prelude::KeywordClassified;

use super::prelude::*;

pub fn partion_skills_by_cost(skills: Vec<Skill>) -> (Vec<Skill>, Vec<Skill>, Vec<Skill>) {
  let mut keystones: Vec<Skill> = Vec::new();
  let mut features: Vec<Skill> = Vec::new();
  let mut minor_features: Vec<Skill> = Vec::new();
  for skill in skills {
    match &skill.training_cost {
      TrainingCost::Inherient | TrainingCost::Keystone => keystones.push(skill),
      TrainingCost::Full | TrainingCost::Spell => features.push(skill),
      TrainingCost::Half | TrainingCost::Cantrip => minor_features.push(skill),
    }
  }
  return (keystones, features, minor_features);
}

pub fn keywords_from_skills(skills: &Vec<Skill>) -> HashSet<ObjectId> {
  skills.iter()
  .flat_map(|skill| skill.get_keyword_ids())
  .collect()
}
