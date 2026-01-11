use dioxus::prelude::*;
use crate::keyword::prelude::*;
use crate::server::prelude::*;
use crate::skill::component::*;
use crate::skill::prelude::*;
use crate::path::components::*;

#[component]
pub fn SkillList() -> Element {
  let response_skills: Resource<Result<Vec<Skill>, ServerFnError>> =
    use_resource(move || list_skills());
  return match &*response_skills.read_unchecked() {
    Some(Ok(skills)) => {
      rsx! {
        div {
          class: "grid spacer-medium dim-skills",
          div { class: "uv-title underline", "Title" }
          div { class: "uv-cost underline centered", "Training Cost" }
          div { class: "uv-activation underline centered", "Activation" }
          div { class: "uv-details underline",  "Summary" }
          SkillTable { skills: skills.to_owned(), training: true }
        }
      }
    }
    None => {
      rsx! { "Loading skills" }
    }
    skills_status => {
      rsx!(
        if let Some( Err( err ) ) = skills_status {
          div { "An error occurred when loading skills: {err}" }
        }
      )
    }
  };
}

#[component]
pub fn SingleSkillPage( id: String ) -> Element {
  let SkillCache( ref skill_cache ) = use_context();
  if let Some( element ) = skill_cache.status_element() {
    return element;
  }
  let skill_result = skill_cache.from_id( &id );
  let Some( skill ) = skill_result else {
    return rsx! {
      div { "Cannot find skill with id: {id}" }
    };
  };
  let KeywordCache( ref keyword_cache ) = use_context();
  let keyword_status_element = keyword_cache.status_element();
  let keywords_all = keyword_cache.from_object_set( &skill.get_keyword_ids() ) ;
  let mut keywords = rules_specific( keywords_all );
  keywords.sort();
  let path_ids = skill.paths.clone().unwrap_or_default();
  return rsx! {
    div {
      class: "column gap-medium",
      SkillCard { skill, display: SkillTermDisplay::Minimal }
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
  let SkillCache( ref skill_cache ) = use_context();
  if let Some( element ) = skill_cache.status_element() {
    return element;
  }
  let mut skills = skill_cache.into_vec();
  skills.sort();
  return rsx! {
    SkillCardList { skills, title_as_link: true }
  };
}
