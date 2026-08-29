use std::collections::HashSet;

use bson::oid::ObjectId;

use crate::keyword::prelude::*;

use super::prelude::*;

pub fn partitioned_sorted_skills<T>(skills_ranks: &Vec<(Skill, T)>) -> Vec<(TrainingCost,Vec<(Skill, T)>)> where T: Clone {
  let mut partitions = Vec::new();
  for cost in TrainingCost::iter() {
    let (mut matched_skills, _remaining_skills): (Vec<_>, Vec<_>) = skills_ranks.into_iter().partition(|(skill, _)|skill.training_cost.eq(cost));
    matched_skills.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));
    partitions.push((cost.clone(), matched_skills.into_iter().cloned().collect()));            
  }
  return partitions;
}

pub fn keywords_from_skills(skills: &Vec<Skill>) -> HashSet<ObjectId> {
  skills
    .iter()
    .flat_map(|skill| skill.get_keyword_ids())
    .collect()
}
