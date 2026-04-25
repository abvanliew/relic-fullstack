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
  // let (keystones, features, minor_features) = partition_skills_by_cost(skills);
  return rsx! {
    if !hide_description {
      div {
        class: "thin-border break-before",
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
            SkillCard { skill, title_as_link: true }
          }
        }
        // if keystones.len() > 0 {
        //   // div {
        //   //   class: "uv-full underhang keep-after",
        //   //   div { class: "small-text dotted-underline", "Keystones" }
        //   // }
        //   for skill in keystones {
        //     StaggeredCell {
        //       SkillCard { skill, title_as_link: true }
        //     }
        //   }
        // }
        // if features.len() > 0 {
        //   // div {
        //   //   class: "uv-full underhang keep-after",
        //   //   div { class: "small-text dotted-underline", "Features" }
        //   // }
        //   for skill in features {
        //     StaggeredCell {
        //       SkillCard { skill, title_as_link: true }
        //     }
        //   }
        // }
        // if minor_features.len() > 0 {
        //   // div {
        //   //   class: "uv-full underhang keep-after",
        //   //   div { class: "small-text dotted-underline", "Minor Features" }
        //   // }
        //   for skill in minor_features {
        //     StaggeredCell {
        //       SkillCard { skill, title_as_link: true }
        //     }
        //   }
        // }
        if keywords.len() > 0 {
          // div {
          //   class: "uv-full keep-after",
          //   div {
          //     class: "underhang",
          //     div { class: "small-text dotted-underline", "Rules Refence" }
          //   }
          // }
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
  #[props(default)] additional_classes: Option<String>,
) -> Element {
  let PathCache( path_map ) = use_context::<PathCache>();
  let mut paths = path_map.from_object_set( &path_ids );
  paths.sort();
  return rsx! { PathChips { paths, paths_as_links, additional_classes } };
}

#[component]
pub fn PathChips(
  paths: Vec<Path>, 
  #[props(default)] paths_as_links: bool,
  #[props(default)] additional_classes: Option<String>,
) -> Element {
  let chip_elements: Vec<Element> = paths
    .iter()
    .map(|path| path.as_chip( paths_as_links, additional_classes.clone() ))
    .collect();
  rsx!(
    for chip in chip_elements { {chip} }
  )
}
