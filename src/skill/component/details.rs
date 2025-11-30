use std::collections::{HashMap, HashSet};

use crate::common::HorizontalBar;
use crate::keyword::prelude::*;
use crate::rules::prelude::*;
use crate::server::prelude::ServerRequestSignals;
use crate::skill::prelude::*;
use crate::Route;

use bson::oid::ObjectId;
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub enum SkillTermDisplay {
  #[default]
  Minimal,
  Embeded,
  External,
}

#[component]
pub fn SkillDescription(
  id: String,
  display: SkillTermDisplay,
  #[props(default)] title_as_link: bool,
) -> Element {
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
  let keywords = filter_keywords(&skill.get_keyword_ids());
  rsx!(
    div {
      class: "group column",
      SkillCard { id, display,title_as_link }
      match &display {
        SkillTermDisplay::External => {
          rsx!{
            for keyword in keywords {
              TermSnippet { term: Term::keyword(keyword), display: TermDisplay::Card }
            }
          }
        },
        _ => rsx!{},
      }
    }
  )
}

pub fn filter_keywords(keywords: &HashSet<ObjectId>) -> HashSet<ObjectId> {
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
    .filter_map(|&id| match keyword_map.get(&id.to_string()) {
      None => None,
      Some(keyword) => match keyword.class {
        Some(KeywordClass::Classifier) => None,
        _ => Some(id),
      },
    })
    .collect()
}

#[component]
pub fn SkillCard(
  id: String,
  display: SkillTermDisplay,
  #[props(default)] title_as_link: bool,
) -> Element {
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
    SkillTermDisplay::Embeded => filter_keywords(&keyword_ids),
    _ => HashSet::new(),
  };
  rsx!(
    div {
      class: "card grid dim-keywords",
      div { class: "uv-title-property title nowrap",
        if title_as_link {
          Link { to: Route::SingleSkill { id }, "{title}" }
        } else {
          "{title}"
        }
      }
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
        HorizontalBar {}
      }
      for keyword in keywords {
        TermSnippet {
          term: Term::keyword(keyword),
          display: TermDisplay::Block
        }
      }
      if let Some( pick_keywords ) = opt_pick_keywords {
        for keyword in pick_keywords {
          HorizontalBar {}
          TermSnippet {
            term: Term::keyword(keyword),
            display: TermDisplay::Block
          }
        }
      }
    }
  )
}

#[component]
fn ActionDetails(action: Action, display: SkillTermDisplay) -> Element {
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
  let (duration, upkeep) = match &action.duration {
    Some(duration) => (Some(duration.base()), duration.upkeep()),
    None => (None, None),
  };

  let mut property_props: Vec<(String, RulesBlocks)> = Vec::new();
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
      span { class: "highlight", "{activation}" }
      if let Some( suffix ) = suffix_opt {
        span {" {suffix} "}
      }
      if keywords.len() > 0 {
        span {class: "italics", " - {keyword_display}"}
      }
    }
    if let Some( blocks ) = action.condition {
      PropertyDetail {
        title: "Condition".to_string(),
        details: rsx!{RulesBlockSet { blocks } }
      }
    }
    if let Some( cost ) = action.cost {
      PropertyDetail {
        title: "Cost".to_string(),
        details: rsx!{ "{cost}" }
      }
    }
    if let Some( duration ) = duration {
      PropertyDetail {
        title: "Duration".to_string(),
        details: rsx!{ "{duration}" }
      }
    }
    if let Some( upkeep ) = upkeep {
      PropertyDetail {
        title: "Upkeep".to_string(),
        details: rsx!{ "{upkeep}" }
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
        details: rsx!{RulesBlockSet { blocks } }
      }
    }
    for (title, blocks) in property_props {
      PropertyDetail {
        title,
        details: rsx!{RulesBlockSet { blocks }}
      }
    }
    if let Some( stacks ) = action.rules {
      RulesStackDetail { stacks, display }
    }
  }
}
