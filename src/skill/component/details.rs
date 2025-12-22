use crate::common::HorizontalBar;
use crate::keyword::prelude::*;
use crate::rules::prelude::*;
use crate::server::prelude::{KeywordCache, SkillCache};
use crate::skill::prelude::*;
use crate::Route;

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub enum SkillTermDisplay {
  #[default]
  Minimal,
  Embeded,
}

#[component]
pub fn SkillCardList(
  skills: Vec<Skill>,
  #[props(default)] display: SkillTermDisplay,
  #[props(default)] title_as_link: bool,
) -> Element {
  rsx! {
    div {
      class: "block-columns",
      for skill in skills {
        SkillCard { skill, display, title_as_link }
      }
    }
  }
}

#[component]
pub fn SkillCardLoader(
  id: String,
  #[props(default)] display: SkillTermDisplay,
  #[props(default)] title_as_link: bool,
) -> Element {
  let SkillCache(skill_cache) = use_context();
  let skill_result = skill_cache.from_id(&id);
  return match skill_result {
    None => rsx! {
      div { class: "card", "No Skill found with id: {id}" }
    },
    Some(skill) => rsx! {
      SkillCard { skill, display, title_as_link }
    },
  };
}

#[component]
pub fn SkillCard(
  skill: Skill,
  #[props(default)] display: SkillTermDisplay,
  #[props(default)] title_as_link: bool,
  #[props(default)] input: Option<Element>,
) -> Element {
  let id = skill.id.to_string();
  let title = skill.title.clone();
  let tier = skill.tier.clone();
  let training_cost = skill.training_cost.clone();
  let opt_description = skill.description.clone();
  let action = skill.action.clone();
  let opt_sub_actions = skill.sub_actions.clone();
  let KeywordCache(keyword_cache) = use_context();
  let selectable_keyword_ids = skill.pick_one_keyword.clone().unwrap_or_default();
  let selectable_keywords = keyword_cache.from_object_ids(&selectable_keyword_ids);
  let keywords_optional = match &display {
    SkillTermDisplay::Minimal => None,
    SkillTermDisplay::Embeded => {
      let keyword_object_ids = skill.get_keyword_ids();
      Some(rules_specific(
        keyword_cache.from_object_ids(&Vec::from_iter(keyword_object_ids)),
      ))
    }
  };
  rsx!(
    div {
      class: "card grid dim-keywords",
      div { class: "uv-title-property title nowrap gap",
        if let Some( input_element ) = input {
          {input_element}
        }
        div {
          if title_as_link {
            Link { to: Route::SingleSkillPage { id }, "{title}" }
          } else {
            "{title}"
          }
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
      if let Some( keywords ) = keywords_optional {
        if keywords.len() > 0 {
          HorizontalBar {}
          KeywordSnippets { keywords, display: KeywordDisplay::Block }
        }
      }
      for keyword in selectable_keywords {
        HorizontalBar {}
        KeywordBlock { keyword }
      }
    }
  )
}

#[component]
fn ActionDetails(action: Action, display: SkillTermDisplay) -> Element {
  let activation = action.title();
  let suffix_opt = action.suffix();
  let KeywordCache(keyword_cache) = use_context::<KeywordCache>();
  let keyword_ids = action.keyword_ids.unwrap_or_default();
  let keywords = keyword_cache
    .from_object_ids(&keyword_ids)
    .iter()
    .map(|keyword| keyword.title.clone())
    .collect::<Vec<String>>();
  let keyword_display = keywords.join(", ");
  let (duration, upkeep) = match &action.duration {
    Some(duration) => (Some(duration.base()), duration.upkeep()),
    None => (None, None),
  };
  let mut property_props: Vec<(String, RulesBlocks)> = Vec::new();
  if let Some(properties) = action.properties {
    for property in properties {
      let title = property.term.to_title();
      property_props.push((title, property.rules));
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
        details: rsx!{ RulesBlockSet { blocks } }
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
