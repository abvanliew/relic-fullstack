use std::cmp::min;
use std::collections::HashSet;

use bson::oid::ObjectId;
use dioxus::prelude::*;

use crate::common::*;
use crate::keyword::prelude::*;
use crate::path::prelude::*;
use crate::server::prelude::*;
use crate::skill::component::*;
use crate::skill::prelude::*;
use crate::Route;

impl Path {
  pub fn as_chip( &self, paths_as_links: bool, additional_classes: Option<String> ) -> Element {
    let title = &self.title;
    let id = self.id.to_string();
    let addition = additional_classes.unwrap_or("".into());
    rsx! {
      div {
        class: "chip {addition}",
        if paths_as_links {
          Link { to: Route::SinglePath { id }, "{title}" }
        } else {
          div { "{title}" }
        }
      }
    }
  }
}

#[component]
pub fn PathPanelList(paths: Vec<Path>) -> Element {
  return rsx! {
    div {
      class: "column gap-large path-skill-wrapper",
      for path in paths {
        PathPanel {
          path,
          title_as_link: true,
          expandable: true,
          hide_keywords: true,
        }
      }
    }
  };
}

#[component]
pub fn PathPanel(
  path: Path, 
  #[props(default)] hide_description: bool, 
  #[props(default)] title_as_link: bool,
  #[props(default)] expandable: bool,
  #[props(default)] hide_keywords: bool,
) -> Element {
  let mut panel_display = use_signal(|| false);
  let id = path.id.to_string();
  let title = path.title;
  let optional_summary = path.summary;
  let SkillCache(ref skill_cache) = use_context();
  let skill_ids = path.skill_ids.unwrap_or_default();
  let mut skills = skill_cache.from_object_ids(&skill_ids);
  skills.sort();
  let keyword_id_objects = keywords_from_skills(&skills);
  let KeywordCache(ref keyword_cache) = use_context();
  let keywords_all = keyword_cache.from_object_set(&keyword_id_objects);
  let keywords = terms_and_conditions(keywords_all);
  return rsx! {
    if !hide_description {
      div {
        class: "secondary-background thin-border break-before",
        onclick: move |_| {
          if !expandable { return }
          panel_display.set( !panel_display() )
        },
        if title_as_link {
          div { class: "title", Link { to: Route::SinglePath { id }, "{title}" } }
        } else {
          div { class: "title", "{title}" }
        }
        if let Some( summary ) = optional_summary {
          div { "{summary}" }
        }
      }
    }
    if !expandable || panel_display() {
      StaggeredGrid {
        for skill in skills {
          StaggeredCell {
            SkillCard { skill, title_as_link: true, include_path_chips: true }
          }
        }
        if !hide_keywords && keywords.len() > 0 {
          for keyword in keywords {
            StaggeredCell {
              KeywordCard { keyword }
            }
          }
        }
      }
    }
  };
}

#[component]
pub fn PathTile(path: ReadSignal<Path>) -> Element {
  let title = path().title;
  rsx!(
    div { "{title}" }
    if let Some( summary ) = path().summary {
      div { class: "small-text", "{summary}" }
    }
  )
}

#[component]
pub fn PathChipsCard(children: Element) -> Element {
  rsx!( div { class: "chip-card", {children} } )
}

#[component]
pub fn PathChipsLoader(
  path_ids: HashSet<ObjectId>, 
  #[props(default)] paths_as_links: bool,
  additional_classes: Option<String>,
  chip_limit: Option<usize>,
) -> Element {
  let PathCache( path_map ) = use_context::<PathCache>();
  let mut paths = path_map.from_object_set( &path_ids );
  paths.sort();
  return rsx! { PathChips { paths, paths_as_links, additional_classes, chip_limit } };
}

#[component]
pub fn PathChips(
  paths: Vec<Path>, 
  #[props(default)] paths_as_links: bool,
  additional_classes: Option<String>,
  chip_limit: Option<usize>,
) -> Element {
  let expanded_chips = use_signal(|| false);
  let chip_elements: Vec<Element> = paths
    .iter()
    .map(|path| path.as_chip( paths_as_links, additional_classes.clone() ))
    .collect();
  let length = if expanded_chips() { paths.len()} else {chip_limit.map_or(paths.len(), |limit| min(limit,paths.len()))};
  let difference = if length < paths.len() {
    Some( paths.len() - length )
  } else {
    None
  };
  rsx!{
    for i in 0..length { {chip_elements[i].clone()} }
    if let Some( difference ) = difference {
      div { class: "chip no-border", "... {difference} more" }
    }
  }
}
