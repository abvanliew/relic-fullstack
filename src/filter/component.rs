use std::collections::HashSet;

use dioxus::prelude::*;

use crate::server::prelude::*;
use crate::keyword::prelude::*;
use crate::skill::component::*;
// use crate::path::components::*;

use super::sets::get_common_classifier_ids;

#[derive(Debug, Clone, PartialEq)]
pub struct SkillFilter {
  pub category: Signal<HashSet<String>>,
}

impl Default for SkillFilter {
  fn default() -> Self {
    let category = use_signal( || HashSet::new() );
    Self { category }
  }
}

#[component]
pub fn SkillSearch() -> Element {
  let SkillCache( ref skill_cache ) = use_context();
  let KeywordCache( ref keyword_cache ) = use_context();
  let filters = SkillFilter::default();
  let current_classifiers = (filters.category)();
  let classifier_ids = get_common_classifier_ids();
  let classifiers = classifier_ids.into_iter()
  .filter_map( |id| {
    match keyword_cache.from_id( &id ) {
      Some( keyword ) => {
        let title = keyword.title;
        let checked = current_classifiers.contains( &id );
        let mut category_copy = filters.category.clone();
        let mut classifier_copy = current_classifiers.clone();
        Some( rsx! {
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
        } )
      },
      None => None
    }
  } )
  .collect::<Vec<Element>>();
  let mut skills = skill_cache.into_vec();
  if current_classifiers.len() > 0 {
    skills = skills.into_iter().filter( |skill| {
      let objects = skill.get_keyword_ids();
      let ids = objects.iter()
      .map( |object| object.to_string() )
      .collect::<HashSet<String>>();
      ids.intersection(&current_classifiers).count() > 0
    } ).collect()
  }
  skills.sort();
  return rsx! {
    div { "Test" }
    div { "{current_classifiers:?}" }
    for classifier in classifiers {
      {classifier}
    }
    SkillCardList { skills, display: TermDisplay::Embeded, title_as_link: true }
  }
}
