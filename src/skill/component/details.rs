use std::collections::{HashMap, HashSet};

use crate::rule::prelude::*;
use crate::server::prelude::ServerRequestSignals;
use crate::skill::prelude::*;
use bson::oid::ObjectId;
use dioxus::prelude::*;

#[component]
pub fn SkillDescription(id: String, display: TermDisplay) -> Element {
  let library = use_context::<ServerRequestSignals>();
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
  rsx!(
    div {
      class: "group column",
      SkillCard { id, display }
      match &display {
        TermDisplay::Block => {
          rsx!{
            for term in terms {
              TermSnippet {
                id: Some( term.to_string() ),
                display: TermDisplay::Block,
              }
            }
          }
        },
        _ => rsx!{},
      }
    }
  )
}

pub fn filter_keywords(keywords: &HashSet<ObjectId>) -> HashSet<String> {
  let library = use_context::<ServerRequestSignals>();
  let keyword_result = library.get_keywords();
  let keyword_map = match keyword_result {
    Some(map) => map,
    None => {
      return HashSet::new();
    }
  };
  keywords
    .iter()
    .map(|id| id.to_string())
    .filter(|term| match keyword_map.get(term) {
      None => false,
      Some(keyword) => match keyword.class {
        Some(KeywordClass::Classifier) => false,
        _ => true,
      },
    })
    .collect()
}

#[component]
pub fn SkillCard(id: String, display: TermDisplay) -> Element {
  let library = use_context::<ServerRequestSignals>();
  let skill_result = library.get_skills();
  let skill = match skill_result {
    Some(skill_map) => match skill_map.get(&id) {
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
  let keyword_ids = skill.get_keyword_ids();
  let keywords = match &display {
    TermDisplay::Embeded => filter_keywords(&keyword_ids),
    _ => HashSet::new(),
  };
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
      if keywords.len() > 0 {
        div { class: "uv-full horizontal-bar" }
      }
      for keyword in keywords {
        TermSnippet {
          id: Some( keyword.to_string() ),
          display: TermDisplay::Row,
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
  let library = use_context::<ServerRequestSignals>();
  let keyword_results = library.get_keywords();
  let keyword_map = match &keyword_results {
    Some(map) => map.clone(),
    None => HashMap::new(),
  };
  let keywords: Vec<String> = if let Some(ref keyword_ids) = action.keyword_ids {
    keyword_ids
      .iter()
      .filter_map(|id| match keyword_map.get(&id.to_string()) {
        Some(keyword) => Some(keyword.title.clone()),
        None => None,
      })
      .collect()
  } else {
    Vec::new()
  };
  let keyword_display = keywords.join(", ");
  let activation = action.base();
  let suffix_opt = action.suffix();

  let mut property_props: Vec<(String, RuleBlocks)> = Vec::new();
  if let Some(properties) = action.properties {
    for property in properties {
      let title = match (property.term.title, property.term.keyword_id) {
        (None, None) => "undefined".to_string(),
        (_, Some(id)) => keyword_map
          .get(&id.to_string())
          .unwrap_or(&Keyword::default())
          .title
          .clone(),
        (Some(term), _) => term,
      };
      property_props.push((title, property.rules))
    }
  }

  rsx! {
    if let Some( sub_title ) = action.sub_title {
      div { class: "uv-full subtitle", "{sub_title}" }
    }
    div { class: "uv-full inline",
      span { class: "highlight", "{activation} " }
      if let Some( suffix ) = suffix_opt {
        span {"{suffix} "}
      }
      if keywords.len() > 0 {
        span {class: "italics", "- {keyword_display}"}
      }
    }
    if let Some( blocks ) = action.condition {
      PropertyDetail {
        title: "Condition".to_string(),
        details: rsx!{RuleBlockSet { blocks, display } }
      }
    }
    if let Some( cost ) = action.cost {
      PropertyDetail {
        title: "Cost".to_string(),
        details: rsx!{ "{cost}" }
      }
    }
    if let Some( duration ) = action.duration {
      PropertyDetail {
        title: "Duration".to_string(),
        details: rsx!{ "{duration}" }
      }
    }
    if let Some( target ) = action.target {
      PropertyDetail {
        title: "Target".to_string(),
        details: rsx!{ "{target}" }
      }
    }
    if let Some( blocks ) = action.refresh {
      PropertyDetail {
        title: "Refresh".to_string(),
        details: rsx!{RuleBlockSet { blocks, display } }
      }
    }
    for (title,rules) in property_props {
      PropertyDetail {
        title,
        details: rsx!{RuleBlockSet { blocks: rules, display }}
      }
    }
    if let Some( stacks ) = action.rules {
      RulesStackDetail { stacks, display }
    }
  }
}
