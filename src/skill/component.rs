use crate::common::HorizontalBar;
use crate::keyword::prelude::*;
use crate::rules::prelude::*;
use crate::server::prelude::*;
use crate::skill::prelude::*;
use crate::Route;
use crate::common::*;

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub enum TermDisplay {
  #[default]
  Standard,
  Embeded,
}

#[component]
pub fn SkillCardList(
  skills: Vec<Skill>,
  #[props(default)] display: TermDisplay,
  #[props(default)] title_as_link: bool,
) -> Element {
  rsx! {
    div {
      class: "staggered-grid",
      for skill in skills {
        StaggeredCell {
          SkillCard { skill, display, title_as_link }
        }
      }
    }
  }
}

#[component]
pub fn SkillCard(
  skill: Skill,
  #[props(default)] display: TermDisplay,
  #[props(default)] title_as_link: bool,
  #[props(default)] input: Option<Element>,
  #[props(default)] on_click: Option<EventHandler<MouseEvent>>,
  #[props(default)] additional_classes: Option<String>,
) -> Element {
  let id = skill.id.to_string();
  let title = skill.title.clone();
  let tier = skill.tier.clone();
  let training_cost = skill.training_cost.clone();
  let opt_description = skill.description.clone();
  let action = skill.action.clone();
  let opt_sub_actions = skill.sub_actions.clone();
  let keywords_optional = match &display {
    TermDisplay::Standard => None,
    TermDisplay::Embeded => {
      let KeywordCache( ref keyword_cache ) = use_context();
      let keyword_object_ids = skill.get_keyword_ids();
      let mut unsorted_keywords = rules_specific(
        keyword_cache.from_object_ids(&Vec::from_iter(keyword_object_ids))
      );
      unsorted_keywords.sort();
      Some(unsorted_keywords)
    }
  };
  let extra_class = match additional_classes {
    Some( class ) => class,
    None => "".into(),
  };
  rsx!(
    div {
      class: "card grid dim-keywords {extra_class}",
      onclick: move |e| { if let Some(handler) = on_click.as_ref() { handler.call(e); } },
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
      ActionDetails { action }
      if let Some( sub_actions ) = opt_sub_actions {
        for action in sub_actions {
          div { class: "spacer" }
          ActionDetails { action }
        }
      }
      if let Some( keywords ) = keywords_optional {
        if keywords.len() > 0 {
          HorizontalBar {}
          KeywordBlocks { keywords }
        }
      }
    }
  )
}

#[component]
fn ActionDetails( action: Action ) -> Element {
  let activation = action.title();
  let suffix_opt = action.suffix();
  let KeywordCache( ref keyword_cache ) = use_context();
  let keyword_ids = action.keyword_ids.unwrap_or_default();
  let keywords = keyword_cache
    .from_object_ids(&keyword_ids).iter()
    .map(|keyword| keyword.title.clone())
    .collect::<Vec<String>>();
  let keyword_display = keywords.join(", ");
  let (duration, upkeep) = match &action.duration {
    Some(duration) => (Some(duration.base()), duration.upkeep()),
    None => (None, None),
  };
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
        RulesBlockSet { blocks }
      }
    }
    if let Some( cost ) = action.cost {
      PropertyDetail {
        title: "Cost".to_string(),
        "{cost}"
      }
    }
    if let Some( duration ) = duration {
      PropertyDetail {
        title: "Duration".to_string(),
        "{duration}"
      }
    }
    if let Some( upkeep ) = upkeep {
      PropertyDetail {
        title: "Upkeep".to_string(),
        "{upkeep}"
      }
    }
    if let Some( target ) = action.target {
      PropertyDetail {
        title: "Target".to_string(),
        "{target}"
      }
    }
    if let Some( blocks ) = action.refresh {
      PropertyDetail {
        title: "Refresh".to_string(),
        RulesBlockSet { blocks }
      }
    }
    if let Some( stacks ) = action.rules {
      RulesStackDetail { stacks }
    }
  }
}
