use std::collections::HashMap;

use dioxus::prelude::*;
use crate::rule::prelude::*;
use crate::skill::prelude::*;

#[component]
pub fn SkillDescription( skill: ReadOnlySignal<Skill>, keywords: ReadOnlySignal<HashMap<String,Keyword>>, show_terms: bool ) -> Element {
  let title = skill.read().title.clone();
  let tier = skill.read().tier.clone();
  let training_cost = skill.read().training_cost.clone();
  let action = skill.read().action.clone();
  let terms = skill.read().get_keyword_ids();
  rsx!(
    div {
      class: "group column",
      div {
        class: "card grid dim-keywords",
        div { class: "uv-title-property title nowrap", "{title}" }
        div { class: "uv-property", 
          div { class: "nowrap italics", "{tier} {training_cost}" }
        }
        if let Some( description ) = skill.read().description.clone() {
          div { class: "uv-full", "{description}" }
        }
        ActionProperties { action, keywords }
      }
      if show_terms {
        for term in terms {
          TermSnippet { term: RuleTerm { keyword_id: Some( term ), title: None }, keywords, hover: false }
        }
      }
    }
  )
}

#[component]
fn ActionProperties( action: Action, keywords: ReadOnlySignal<HashMap<String,Keyword>> ) -> Element {
  let activation = action.activation();
  rsx!(
    div{
      class: "uv-full row",
      div {
        class: "highlight", "{activation}"
      }
      if let Some( keywords ) = action.keywords {
        div { class: "italics", "{keywords}" }
      }
    }
    if let Some( condition ) = action.condition {
      div { class: "uv-title nowrap highlight", "Condition" }
      div { class: "uv-details", SnippetSetDetails { rules: condition, keywords } }
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
        SnippetSetDetails { rules, keywords }
      }
    }
  )
}
