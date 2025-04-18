use dioxus::prelude::*;
use crate::skill::prelude::*;
use crate::server::prelude::*;

#[component]
pub fn SkillList() -> Element {
  let response: Resource<Result<Vec<Skill>, ServerFnError>> = use_resource( move || list_skills() );
  return match &*response.read_unchecked() {
    Some( Ok( skills ) ) => {
      rsx! {
        div {
          class: "grid spacer dim-skills",
          div { class: "uv-title underline", "Title" }
          div { class: "uv-cost underline centered", "Training Cost" }
          div { class: "uv-activation underline centered", "Activation" }
          div { class: "uv-details underline",  "Summary" }
          SkillTable { skills: skills.to_owned(), training: true }
        }
      }
    }
    Some(Err(err)) => {
      rsx! { "An error occurred when loading skills: {err}" }
    }
    None => { rsx! { "Loading skills" } }
  }
}

#[component]
pub fn SingleSkill( id: String ) -> Element {
  let response: Resource<Result<Skill, ServerFnError>> = use_resource( move || get_skill( id.clone() ) );
  match &*response.read_unchecked() {
    Some( Ok( skill ) ) => {
      rsx! {
        SkillDescription { skill: skill.clone() }
      }
    },
    Some( Err( err) ) => {
      rsx! { "An error occurred when loading skill: {err}" }
    },
    None => { rsx! { "Loading skill" } },
  }
}


#[component]
pub fn FullSkillList() -> Element {
  let response: Resource<Result<Vec<Skill>, ServerFnError>> = use_resource( move || list_skills() );
  return match &*response.read_unchecked() {
    Some( Ok( skills ) ) => {
      rsx! {
        div {
          class: "row-wrap",
          for skill in skills {
            SkillDescription { skill: skill.clone() }
          }
        }
      }
    }
    Some(Err(err)) => {
      rsx! { "An error occurred when loading skills: {err}" }
    }
    None => { rsx! { "Loading skills" } }
  }
}
