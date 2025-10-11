use crate::rule::prelude::*;
use crate::server::prelude::GameLibrarySignal;
use crate::skill::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn SkillDescription(id: String, show_terms: bool) -> Element {
  let library = use_context::<GameLibrarySignal>();
  let res_skills = library.get_skills();
  let skill = match res_skills {
    Some(map) => match map.get(&id) {
      Some(skill) => skill.clone(),
      None => return rsx! {},
    },
    _ => {
      return rsx! { div { "Loading" } };
    }
  };
  let terms = skill.get_keyword_ids().clone();
  let display = if show_terms {
    TermDisplay::TitleOnly
  } else {
    TermDisplay::Hover
  };
  rsx!(
    div {
      class: "group column",
      SkillCard { id, display }
      if show_terms {
        for term in terms {
          TermSnippet {
            id: Some( term.to_string() ),
            display: TermDisplay::Block,
          }
        }
      }
    }
  )
}

#[component]
pub fn SkillCard(id: String, display: TermDisplay) -> Element {
  let library = use_context::<GameLibrarySignal>();
  let res_skills = library.get_skills();
  let skill = match res_skills {
    Some(map) => match map.get(&id) {
      Some(skill) => skill.clone(),
      None => return rsx! {},
    },
    _ => {
      return rsx! { div { class: "card", "Loading" } };
    }
  };
  let title = skill.title.clone();
  let tier = skill.tier.clone();
  let training_cost = skill.training_cost.clone();
  let opt_description = skill.description.clone();
  let action = skill.action.clone();
  let opt_sub_actions = skill.sub_actions.clone();
  let opt_pick_keywords = skill.pick_one_keyword.clone();
  rsx!(
    div {
      class: "card grid dim-keywords",
      div { class: "uv-title-property title nowrap", "{title}" }
      div { class: "uv-property",
        div { class: "nowrap italics", "{tier} {training_cost}" }
      }
      if let Some( description ) = opt_description {
        div { class: "uv-full", "{description}" }
      }
      ActionDetails { action, display }
      if let Some( sub_actions ) = opt_sub_actions {
        for action in sub_actions {
          div { class: "spacer" }
          ActionDetails { action, display }
        }
      }
      if let Some( pick_keywords ) = opt_pick_keywords {
        for keyword in pick_keywords {
          div { class: "uv-full horizontal-bar" }
          TermSnippet {
            id: Some( keyword.to_string() ),
            display: TermDisplay::Row,
          }
        }
      }
    }
  )
}

#[component]
fn ActionDetails(action: Action, display: TermDisplay) -> Element {
  let activation = action.base();
  let suffix_opt = action.suffix();
  rsx!(
    if let Some( sub_title ) = action.sub_title {
      div { class: "uv-full subtitle", "{sub_title}" }
    }
    div { class: "uv-full inline",
      span { class: "highlight", "{activation} " }
      if let Some( suffix ) = suffix_opt {
        span {"{suffix} "}
      }
      if let Some( keywords ) = action.keywords {
        span {class: "italics", "- {keywords}"}
      }
    }
    if let Some( blocks ) = action.condition {
      PropertyDetail { title: "Condition", blocks, display }
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
        div { class: "uv-details", RuleBlockSet { blocks: property.rules, display } }
      }
    }
    if let Some( stacks ) = action.rules {
      RulesStackDetail { stacks, display }
    }
  )
}
