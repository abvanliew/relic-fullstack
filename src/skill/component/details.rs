use dioxus::prelude::*;
use crate::rule::prelude::*;
use crate::skill::prelude::*;

#[component]
pub fn SkillDescription( skill: ReadOnlySignal<Skill> ) -> Element {
  let title = skill.read().title.clone();
  let tier = skill.read().tier.clone();
  let training_cost = skill.read().training_cost.clone();
  let action = skill.read().action.clone();
  rsx!(
    div {
      class: "card grid dim-keywords",
      div { class: "uv-title-property title nowrap", "{title}" }
      div { class: "uv-property", 
        div { class: "nowrap italics", "{tier} - {training_cost}" }
      }
      ActionProperties { action }
    }
  )
}

#[component]
fn ActionProperties( action: Action ) -> Element {
  let activation = action.activation();
  rsx!(
    div{ class: "uv-full nowrap", "{activation}" }
    if let Some( condition ) = action.condition {
      div { class: "uv-title nowrap highlight", "Condition" }
      div { class: "uv-details", SnippetSetDetails { rules: condition } }
    }
    if let Some( cost ) = action.cost {
      div { class: "uv-title nowrap highlight", "Cost" }
      div { class: "uv-details", "{cost}" }
    }
    if let Some( duration ) = action.duration {
      div { class: "uv-title nowrap highlight", "Duration" }
      div { class: "uv-details", "{duration}" }
    }
    if let Some( target ) = action.target {
      div { class: "uv-title nowrap highlight", "Target" }
      div { class: "uv-details", "{target}" }
    }
    if let Some( rules ) = action.rules {
      div {
        class: "uv-full",
        SnippetSetDetails { rules }
      }
    }
  )
}
