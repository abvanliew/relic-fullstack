use crate::keyword::prelude::KeywordClassified;
use crate::keyword::prelude::KeywordSnippetsLoader;
use crate::server::prelude::*;
use crate::skill::component::*;
use crate::skill::prelude::*;
use dioxus::prelude::*;

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
pub fn SingleSkillPage(id: String) -> Element {
  let SkillCache(skill_cache) = use_context();
  if let Some(element) = skill_cache.status_element() {
    return element;
  }
  let skill_result = skill_cache.from_id(&id);
  let Some(skill) = skill_result else {
    return rsx! {
      div { "Cannot find skill with id: {id}" }
    };
  };
  let KeywordCache(keyword_cache) = use_context();
  let keyword_status_element = keyword_cache.status_element();
  let keyword_id_objects = skill.get_keyword_ids();
  return rsx! {
    div {
      class: "column gap-large", 
      SkillCard { skill, display: SkillTermDisplay::Minimal }
      match keyword_status_element {
        Some( element ) => element,
        None => rsx! {
          KeywordSnippetsLoader { keyword_id_objects  }
        }
      }
    }
  };
}

#[component]
pub fn SkillsPage() -> Element {
  let SkillCache(skill_cache) = use_context();
  if let Some(element) = skill_cache.status_element() {
    return element;
  }
  let skills = skill_cache.into_vec();
  return rsx! {
    SkillCardList { skills, title_as_link: true }
  };
}
