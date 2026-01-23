use crate::keyword::prelude::*;
use crate::path::components::*;
use crate::server::prelude::*;
use crate::skill::component::*;
use dioxus::prelude::*;

#[component]
pub fn SingleSkillPage(id: String) -> Element {
  let SkillCache(ref skill_cache) = use_context();
  if let Some(element) = skill_cache.status_element() {
    return element;
  }
  let skill_result = skill_cache.from_id(&id);
  let Some(skill) = skill_result else {
    return rsx! {
      div { "Cannot find skill with id: {id}" }
    };
  };
  let KeywordCache(ref keyword_cache) = use_context();
  let keyword_status_element = keyword_cache.status_element();
  let keywords_all = keyword_cache.from_object_set(&skill.get_keyword_ids());
  let mut keywords = rules_specific(keywords_all);
  keywords.sort();
  let path_ids = skill.paths.clone().unwrap_or_default();
  return rsx! {
    div {
      class: "column gap-medium",
      SkillCard { skill, display: TermDisplay::Standard }
      match keyword_status_element {
        Some( element ) => element,
        None => rsx! { KeywordCards { keywords } }
      }
      if path_ids.len() > 0 {
        PathChipsLoader { path_ids, paths_as_links: true }
      }
    }
  };
}

#[component]
pub fn SkillsPage() -> Element {
  let SkillCache(ref skill_cache) = use_context();
  if let Some(element) = skill_cache.status_element() {
    return element;
  }
  let mut skills = skill_cache.into_vec();
  skills.sort();
  return rsx! {
    SkillCardList { skills, display: TermDisplay::Embeded, title_as_link: true }
  };
}
