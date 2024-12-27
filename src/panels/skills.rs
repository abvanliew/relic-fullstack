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
          class: "grid dim-title-details",
          for skill in list {
            SkillSummary { skill: skill.clone() }
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
fn SkillSummary( skill: ReadOnlySignal<Skill> ) -> Element {
  let title = skill.read().title.clone();
  let id = skill.read().oid.to_string();
  let summary = skill.read().summary.clone().unwrap_or( "".into() );
  rsx!(
    div {
      class: "uv-title",
      Link {
        to: Route::SingleSkill { id }, "{title}"
      }
    }
    div {
      class: "uv-details",
      "{summary}"
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
        div { "{title}" }
        div { "{tier} Tier" }
        div { "Cost: {cost}" }
        ActionDescription { action }
      )
    },
    Err( error ) => rsx!( div{ "error: {error}" } ),
  }
}

#[component]
fn ActionDescription( action: Action ) -> Element {
  let debug = action.clone();
  let class = match action.class {
    Some( value ) => value.to_string(),
    _ => "Unknown".into(),
  };
  rsx!(
    div{ "{class}" }
    if let Some( duration ) = action.duration {
      div {
        class: "row",
        div { "Duration" }
        div { "{duration}" }
      }
    }
    if let Some( target ) = action.target {
      div {
        class: "row",
        div { "Target" }
        div { "{target}" }
      }
    }
    if let Some( rules ) = action.rules {
      for rule in rules {
        RuleDescription { rule }
      }
    }
    div { "{debug:?}" }
  )
}

#[component]
fn RuleDescription( rule: RuleElement ) -> Element {
  match rule.class {
    RuleClass::Text => {
      let text: String = rule.text.unwrap_or_default();
      rsx!( span { "{text}" } )
    },
    RuleClass::Roll => {
      let Some( roll ) = rule.roll else { return rsx!() };
      rsx!( span { "{roll}" } )
    },
    RuleClass::Outcome => {
      let outcomes = rule.outcomes.unwrap_or_default();
      rsx!(
        div {
          class: "grid dim-title-details",
          for outcome in outcomes {
            div {
              class: "uv-title",
              "{outcome.result}"
            }
            div {
              class: "uv-details",
              for rule in outcome.rules {
                RuleDescription { rule }
              }
            }
          }
        }
      )
    },
  }
}
