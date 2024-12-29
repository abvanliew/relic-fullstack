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
          class: "grid dim-skill-table",
          SkillTable { skills: skills.to_owned() }
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
