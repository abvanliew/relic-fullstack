use std::collections::HashMap;

use dioxus::prelude::*;
use crate::skill::prelude::*;
use crate::server::prelude::*;

#[component]
pub fn SkillList() -> Element {
  let response_skills: Resource<Result<Vec<Skill>, ServerFnError>> = use_resource( move || list_skills() );
  return match &*response_skills.read_unchecked() {
    Some( Ok( skills ) ) => {
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
    },
    None => { rsx! { "Loading skills" } },
    skills_status => {
      rsx!(
        if let Some( Err( err ) ) = skills_status {
          div { "An error occurred when loading skills: {err}" }
        }
      )
    },
  }
}

#[component]
pub fn SingleSkill( id: String ) -> Element {
  let response_skill: Resource<Result<Skill, ServerFnError>> = use_resource( move || get_skill( id.clone() ) );
  let response_keywords: Resource<Result<HashMap<String,Keyword>, ServerFnError>> = use_resource( move || list_keywords() );
  match (
    &*response_skill.read_unchecked(),
    &*response_keywords.read_unchecked(),
  ) {
    ( Some( Ok( skill ) ), Some( Ok( keywords ) ) ) => {
      rsx! {
        SkillDescription { skill: skill.clone(), keywords: keywords.to_owned(), show_terms: false }
      }
    },
    ( None, None ) => { rsx! { "Loading skill" } },
    ( skill_status, keyword_status ) => {
      rsx!(
        div { "Loading Skill Failure" }
        if let Some( Err( err ) ) = skill_status {
          div { "An error occurred when loading skill: {err}" }
        }
        if let Some( Err( err ) ) = keyword_status {
          div { "An error occurred when loading keywords: {err}" }
        }
      )
    },
  }
}

#[component]
pub fn FullSkillList() -> Element {
  let response: Resource<Result<Vec<Skill>, ServerFnError>> = use_resource( move || list_skills() );
  let response_keywords: Resource<Result<HashMap<String,Keyword>, ServerFnError>> = use_resource( move || list_keywords() );
  return match (
    &*response.read_unchecked(),
    &*response_keywords.read_unchecked(),
   ) {
    ( Some( Ok( skills ) ), Some( Ok( keywords ) ) ) => {
      rsx! {
        div {
          class: "row-wrap",
          for skill in skills {
            SkillDescription { skill: skill.clone(), keywords: keywords.to_owned(), show_terms: true }
          }
        }
      }
    }
    ( None, None ) => { rsx! { "Loading skills" } }
    ( skill_status, keyword_status ) => {
      rsx!(
        div { "Loading Skill Failure" }
        if let Some( Err( err ) ) = skill_status {
          div { "An error occurred when loading skill: {err}" }
        }
        if let Some( Err( err ) ) = keyword_status {
          div { "An error occurred when loading keywords: {err}" }
        }
      )
    },
  }
}
