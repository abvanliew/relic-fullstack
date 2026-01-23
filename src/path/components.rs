use std::collections::HashSet;

use bson::oid::ObjectId;
use dioxus::prelude::*;

use crate::common::*;
use crate::keyword::prelude::*;
use crate::path::Path;
use crate::server::prelude::*;
use crate::skill::component::*;
use crate::skill::prelude::*;
use crate::Route;

#[component]
pub fn PathPanelList(paths: Vec<Path>) -> Element {
  return rsx! {
    div {
      class: "column gap-large path-skil-wrapper",
      for path in paths {
        PathPanel {
          path,
          title_as_link: true
        }
      }
    }
  };
}

#[component]
pub fn PathPanel(
  path: Path, #[props(default)] hide_description: bool, #[props(default)] title_as_link: bool,
) -> Element {
  let id = path.id.to_string();
  let title = path.title;
  let optional_summary = path.summary;
  let SkillCache(ref skill_cache) = use_context();
  let skill_ids = path.skill_ids.unwrap_or_default();
  let skills = skill_cache.from_object_ids(&skill_ids);
  let keyword_id_objects = keywords_from_skills(&skills);
  let KeywordCache(ref keyword_cache) = use_context();
  let keywords_all = keyword_cache.from_object_set(&keyword_id_objects);
  let mut keywords = rules_specific(keywords_all);
  keywords.sort();
  let (keystones, features, minor_features) = partition_skills_by_cost(skills);
  return rsx! {
    if !hide_description {
      div {
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
    if keystones.len() > 0 {
      div {
        div { class: "small-text dotted-underline underhang", "Keystones" }
        SkillCardList { skills: keystones, title_as_link: true }
      }
    }
    if features.len() > 0 {
      div {
        div { class: "small-text dotted-underline underhang", "Features" }
        SkillCardList { skills: features, title_as_link: true }
      }
    }
    if minor_features.len() > 0 {
      div {
        div { class: "small-text dotted-underline underhang", "Minor Features" }
        SkillCardList { skills: minor_features, title_as_link: true }
      }
    }
    if keywords.len() > 0 {
      div {
        div { class: "small-text dotted-underline underhang", "Rules Refence" }
        div {
          class: "staggered-grid",
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
pub fn PathChipsLoader(
  path_ids: HashSet<ObjectId>, #[props(default)] paths_as_links: bool,
  #[props(default)] additional_classes: Option<String>,
) -> Element {
  let PathCache(ref path_cache) = use_context::<PathCache>();
  let mut paths = path_cache.from_object_set(&path_ids);
  paths.sort();
  return rsx! { PathChips { paths, paths_as_links, additional_classes } };
}

#[component]
pub fn PathChips(
  paths: Vec<Path>, #[props(default)] paths_as_links: bool,
  #[props(default)] additional_classes: Option<String>,
) -> Element {
  let extra_class = additional_classes.unwrap_or("".into());
  let chip_elements: Vec<Element> = paths
    .iter()
    .map(|path| {
      let title = path.title.clone();
      let id = path.id.to_string();
      rsx! {
        div {
          class: "chip {extra_class}",
          if paths_as_links {
            Link { to: Route::SinglePath { id }, "{title}" }
          } else {
            div { "{title}" }
          }
        }
      }
    })
    .collect();
  rsx!(
    div {
      class: "chip-card",
      for chip in chip_elements {
        {chip}
      }
    }
  )
}
