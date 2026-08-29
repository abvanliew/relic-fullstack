use std::collections::HashSet;

use crate::common::*;
use crate::keyword::prelude::*;
use crate::path::components::{PathChipsCard, PathChipsLoader};
use crate::rules::prelude::*;
use crate::skill::prelude::*;
use crate::Route;

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub enum TermDisplay {
  #[default]
  Standard,
  Embeded,
}

#[component]
pub fn SkillCardElements(
  skills: Vec<Skill>, #[props(default)] display: TermDisplay,
  #[props(default)] title_as_link: bool, #[props(default)] include_path_chips: bool,
  #[props(default)] collapsed: bool,
) -> Element {
  rsx! {
    for skill in skills {
      StaggeredCell {
        SkillCard { skill, display, title_as_link, include_path_chips, collapsed }
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
  #[props(default)] click_event: Option<EventHandler<MouseEvent>>,
  #[props(default)] additional_classes: Option<String>, 
  #[props(default)] include_path_chips: bool,
  #[props(default)] collapsed: bool,
) -> Element {
  let id = skill.id.to_string();
  let title = skill.title.clone();
  let training_requirements = skill.training_requirements();
  let opt_description = skill.description.clone();
  let action = skill.action.clone();
  let sub_actions = skill.sub_actions.clone();
  let path_ids = match (include_path_chips, collapsed) {
    (true, false) => skill.paths.clone().unwrap_or_default(),
    _ => HashSet::new(),
  };
  let activation_element = action.activation_element();
  let extra_class = match additional_classes {
    Some(class) => class,
    None => "".into(),
  };
  rsx!(
    div {
      class: "card grid dim-keywords {extra_class}",
      onclick: move |e| { if let Some(handler) = click_event.as_ref() { handler.call(e); } },
      div {
        class: "uv-title-property align-center gap",
        if let Some( input_element ) = input {
          {input_element}
        }
        div {
          class: "title",
          if title_as_link {
            Link { to: Route::SingleSkillPage { id }, "{title}" }
          } else {
            "{title}"
          }
        }
      }
      div { class: "uv-property",
        if collapsed {
          {activation_element}
        } else {
          div { class: "nowrap italics", "{training_requirements}" }
        }
      }
      if let Some( description ) = opt_description {
        div { class: "uv-full", "{description}" }
      }
      if !collapsed {
        ActionDetails { action }
        if sub_actions.len() > 0 {
          for action in sub_actions {
            div { class: "spacer" }
            ActionDetails { action }
          }
        }
      }
    }
    if path_ids.len() > 0 {
      PathChipsCard {
        PathChipsLoader {
          path_ids,
          additional_classes: Some( extra_class.clone() ),
          chip_limit: 4,
        }
      }
    }
  )
}

#[component]
pub fn ActionDetails(action: RelicAction) -> Element {
  let activation_element = action.activation_element();
  let keyword_ids = action.keyword_ids.unwrap_or_default();
  let keyword_display = display_keywords(&keyword_ids);
  let (duration, upkeep) = match &action.duration {
    Some(duration) => (Some(duration.base()), duration.upkeep()),
    None => (None, None),
  };
  rsx! {
    if let Some( sub_title ) = action.sub_title {
      div { class: "uv-full subtitle", "{sub_title}" }
    }
    div { class: "uv-full inline",
      {activation_element}
      if let Some( keyword_display ) = keyword_display {
        span {class: "italics", " - {keyword_display}"}
      }
    }
    if let Some( sections ) = action.condition {
      PropertyDetail {
        title: "Condition",
        RulesSectionSet { sections }
      }
    }
    if let Some( cost ) = action.cost {
      PropertyDetail {
        title: "Cost",
        "{cost}"
      }
    }
    if let Some( duration ) = duration {
      PropertyDetail {
        title: "Duration",
        "{duration}"
      }
    }
    if let Some( upkeep ) = upkeep {
      PropertyDetail {
        title: "Upkeep",
        "{upkeep}"
      }
    }
    if let Some( target ) = action.target {
      PropertyDetail {
        title: "Target",
        "{target}"
      }
    }
    if let Some( sections ) = action.refresh {
      PropertyDetail {
        title: "Refresh",
        RulesSectionSet { sections }
      }
    }
    if let Some( stacks ) = action.rules {
      RulesStackDetail { stacks }
    }
  }
}
