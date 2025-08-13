use dioxus::prelude::*;
use crate::rule::prelude::*;
use crate::skill::prelude::*;

#[component]
pub fn SkillDescription(
  skill: ReadOnlySignal<Skill>,
  show_terms: bool
) -> Element {
  let title = skill.read().title.clone();
  let tier = skill.read().tier.clone();
  let training_cost = skill.read().training_cost.clone();
  let opt_description = skill.read().description.clone();
  let action = skill.read().action.clone();
  let opt_sub_actions = skill.read().sub_actions.clone();
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
        if let Some( description ) = opt_description {
          div { class: "uv-full", "{description}" }
        }
        ActionDetails { action }
        if let Some( sub_actions ) = opt_sub_actions {
          for action in sub_actions {
            div { class: "spacer" }
            ActionDetails { action }
          }
        }
      }
      if show_terms {
        for term in terms {
          TermSnippet {
            term: Term { keyword_id: Some( term ), title: None },
            hover: false,
          }
        }
      }
    }
  )
}

#[component]
fn ActionDetails( action: Action ) -> Element {
  let activation = action.base();
  let suffix_opt = action.suffix();
  rsx!(
    div { class: "uv-full inline",
      if let Some( sub_title ) = action.sub_title {
        div { class: "subtitle", "{sub_title}" }
      }
      div { class: "highlight", "{activation}" }
      if let Some( suffix ) = suffix_opt {
        div { "{suffix}" }
      }
      if let Some( keywords ) = action.keywords {
        div { class: "italics", "{keywords}" }
      }
    }
    if let Some( blocks ) = action.condition {
      PropertyDetail { title: "Condition", blocks }
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
    if let Some( properties ) = action.properties {
      for property in properties {
        div { class: "uv-title nowrap highlight", "{property.title}" }
        div { class: "uv-details", RuleBlockSet { blocks: property.rules } }
      }
    }
    if let Some( stacks ) = action.rules {
      RulesStackDetail { stacks }
    }
  )
}
