use crate::rule::prelude::*;
use crate::server::prelude::*;
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
pub fn SingleSkill(id: String) -> Element {
  let response_skill: Resource<Result<Skill, ServerFnError>> =
    use_resource(move || get_skill(id.clone()));
  match &*response_skill.read_unchecked() {
    Some(Ok(skill)) => {
      rsx! {
        SkillDescription { id: skill.id.to_string(), display: TermDisplay::Embeded }
      }
    }
    None => {
      rsx! { "Loading skill" }
    }
    skill_status => {
      rsx!(
        div { "Loading Skill Failure" }
        if let Some( Err( err ) ) = skill_status {
          div { "An error occurred when loading skill: {err}" }
        }
      )
    }
  }
}

#[component]
pub fn FullSkillList() -> Element {
  let response: Resource<Result<Vec<Skill>, ServerFnError>> = use_resource(move || list_skills());
  return match &*response.read_unchecked() {
    Some(Ok(skills)) => {
      rsx! {
        div {
          class: "row-wrap gap-xlarge",
          div {
            class: "column-wrap gap-xlarge",
            for skill in skills.iter().step_by(2) {
              SkillDescription { id: skill.id.to_string(), display: TermDisplay::Embeded }
            }
          }
          div {
            class: "column-wrap gap-xlarge",
            for skill in skills.iter().skip(1).step_by(2) {
              SkillDescription { id: skill.id.to_string(), display: TermDisplay::Embeded }
            }
          }
        }
      }
    }
    None => {
      rsx! { "Loading skills" }
    }
    skill_status => {
      rsx!(
        div { "Loading Skill Failure" }
        if let Some( Err( err ) ) = skill_status {
          div { "An error occurred when loading skill: {err}" }
        }
      )
    }
  };
}
