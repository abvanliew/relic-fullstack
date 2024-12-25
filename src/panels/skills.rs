use dioxus::prelude::*;
use crate::relic::prelude::*;
use crate::server::prelude::*;
use crate::Route;

#[component]
pub fn Skills() -> Element {
  let response: Resource<Result<Vec<Skill>, ServerFnError>> = use_resource( move || list_skills() );
  match &*response.read_unchecked() {
    Some( Ok( list ) ) => {
      rsx! {
        div {
          for skill in list {
            SkillSummary { skill: skill.clone() }
          }
        }
      }
    }
    Some(Err(err)) => {
      rsx! { "An error occurred when loading skills: {err}" }
    }
    None => {
      rsx! { "Loading skills" }
    }
  }
}

#[component]
pub fn SingleSkill( id: String ) -> Element {
  tracing::info!( "calling server" );
  let response: Resource<Result<Skill, ServerFnError>> = use_resource( move || get_skill( id.clone() ) );
  tracing::info!( "parsing response" );
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
fn SkillSummary( skill: ReadOnlySignal<Skill> ) -> Element {
  let title = skill.read().title.clone();
  let id = skill.read().oid.to_string();
  rsx!(
    div {
      Link { to: Route::SingleSkill { id }, "{title}" }
    }
  )
}

#[component]
fn SkillDescription( skill: ReadOnlySignal<Skill> ) -> Element {
  match skill.try_read(){
    Ok( skill_ref ) => {
      let title = skill_ref.title.clone();
      let tier = skill_ref.tier();
      let cost = skill_ref.cost();
      let action = skill_ref.action.clone();
      rsx!(
        div { "{title} [{tier} Tier]" }
        div { "Training Cost: {cost}" }
        ActionDescription { action }
      )
    },
    Err( error ) => rsx!( div{ "error: {error}" } ),
  }
}

#[component]
fn ActionDescription( action: Action ) -> Element {
  rsx!(
    div{ "{action.display_type()}" }
    if let Some( duration ) = action.duration {
      div { "Duration: {duration.display()}" }
    }
    if let Some( target ) = action.target {
      div { "Target: {target}" }
    }
    if let Some( body ) = action.body {
      div { "{body}" }
    }
  )
}
