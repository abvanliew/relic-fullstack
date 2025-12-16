use crate::server::prelude::*;
use crate::skill::component::*;
use crate::skill::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn InherentSkillsPage() -> Element {
  let SkillCache(skill_cache) = use_context::<SkillCache>();
  let mut skills = skill_cache
    .into_vec()
    .into_iter()
    .filter(|skill| match skill.training_cost {
      TrainingCost::Inherient => true,
      _ => false,
    })
    .collect::<Vec<Skill>>();
  skills.sort_by_key(|skill| skill.title.clone());
  return rsx! {
    SkillCardList { skills }
  };
}
