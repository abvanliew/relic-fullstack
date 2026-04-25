use std::collections::HashSet;

use dioxus::prelude::*;

use crate::common::{StaggeredCell, StaggeredGrid};
use crate::keyword::prelude::*;
use crate::server::prelude::*;
use crate::skill::component::*;

use super::sets::get_common_classifier_ids;

#[derive(Debug, Clone, PartialEq)]
pub struct SkillFilter {
  pub category: Signal<HashSet<String>>,
  pub path: Signal<HashSet<String>>,
  pub mono_skill: Signal<bool>,
}

impl Default for SkillFilter {
  fn default() -> Self {
    let category = use_signal(|| HashSet::new());
    let path = use_signal(|| HashSet::new());
    let mono_skill = use_signal(|| false);
    Self { category, path, mono_skill }
  }
}

#[component]
pub fn SkillSearch() -> Element {
  let SkillCache(ref skill_cache) = use_context();
  let KeywordCache(ref keyword_cache) = use_context();
  let PathCache(ref path_cache) = use_context();
  let mut filters = SkillFilter::default();
  let current_classifiers = (filters.category)();
  let classifier_ids = get_common_classifier_ids();
  let classifiers = classifier_ids
    .into_iter()
    .filter_map(|id| match keyword_cache.from_id(&id) {
      Some(keyword) => {
        let title = keyword.title;
        let checked = current_classifiers.contains(&id);
        let mut category_copy = filters.category.clone();
        let mut classifier_copy = current_classifiers.clone();
        Some(rsx! {
          div {
            class: "row",
            input {
              r#type: "checkbox",
              checked,
              oninput: move |_| {
                match checked {
                  true => { classifier_copy.remove( &id ) }
                  false => { classifier_copy.insert( id.clone() ) }
                };
                category_copy.set( classifier_copy.clone() )
              }
            }
            div { "{title}" }
          }
        })
      },
      None => None,
    })
    .collect::<Vec<Element>>();
  let current_paths = (filters.path)();
  let mut paths = path_cache.into_vec();
  paths.sort();
  let path_classifiers = paths
    .into_iter()
    .map(|path| {
      let title = path.title;
      let id = path.id.to_string();
      let checked = current_paths.contains(&id);
      let mut path_signal = filters.path.clone();
      let mut current_path_copy = current_paths.clone();
      rsx! {
        div {
          class: "row",
          input {
            r#type: "checkbox",
            checked,
            oninput: move |_| {
              match checked {
                true => { current_path_copy.remove( &id ) }
                false => { current_path_copy.insert( id.clone() ) }
              };
              path_signal.set( current_path_copy.clone() )
            }
          }
          div { "{title}" }
        }
      }
    })
    .collect::<Vec<Element>>();
  let mut skills = skill_cache.into_vec();
  skills.sort();
  let skill_elements = skills
  .iter()
  .map(
    |skill| {
      let mut display: bool = true;
      display = if display && (filters.mono_skill)() {
        match &skill.paths {
          Some(paths) => paths.len() == 1,
          None => false,
        }
      } else { display };
      display = if display && current_classifiers.len() > 0 {
        let keyword_object_ids = skill.get_keyword_ids();
        let keyword_id_set = keyword_object_ids
          .iter()
          .map(|object_id| object_id.to_string())
          .collect::<HashSet<String>>();
        keyword_id_set.intersection(&current_classifiers).count() > 0
      } else { display };
      display = if display && current_paths.len() > 0 {
        let path_id_set = skill.paths
          .clone()
          .unwrap_or_default()
          .iter()
          .map(|object_id| object_id.to_string())
          .collect::<HashSet<String>>();
        path_id_set.intersection(&current_paths).count() > 0
      } else { display };
      rsx! {
        StaggeredCell {
          additional_classes: if display { None } else { Some( "hidden".into() ) },
          SkillCard {
            skill: skill.clone(),
            display: TermDisplay::Embeded,
            title_as_link: true,
            include_path_chips: true
          }
        }
      }
    }
  )
  .collect::<Vec<Element>>();
  let path_titles = current_paths.iter()
    .filter_map(|path_id| path_cache.from_id(path_id))
    .map(|path| path.title)
    .collect::<Vec<String>>()
  ;
  let display_titles = path_titles.len() > 0;
  let joined_titles = path_titles.join(", ");
  return rsx! {
    div { class: "no-print row selector-gap underhang",
      input {
        r#type: "checkbox",
        checked: (filters.mono_skill)(),
        oninput: move |_| {
          filters.mono_skill.set( !(filters.mono_skill)() )
        }
      }
      div { "Unique Skill" }
    }
    div { class: "no-print underhang", "Classifier" }
    div {
      class: "no-print underhang row-wrap selector-gap",
      for classifier in classifiers {
        {classifier}
      }
    }
    div { class: "no-print underhang", "Paths" }
    div {
      class: "no-print underhang row-wrap selector-gap",
      for path_classifier in path_classifiers {
        {path_classifier}
      }
    }
    if display_titles {
      div { class: "title underhang", "{joined_titles}" }
    }
    StaggeredGrid {
      for skill_element in skill_elements {
        {skill_element}
      }
    }
  };
}
