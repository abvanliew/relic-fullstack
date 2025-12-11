use dioxus::prelude::*;

use crate::path::Path;
use crate::server::prelude::*;
use crate::skill::prelude::*;
use crate::skill::component::*;
use crate::keyword::prelude::*;

#[component]
pub fn PathListLoader() -> Element {
  let PathCache( path_cache ) = use_context::<PathCache>();
  let path_results = path_cache.into_result_vec();
  match path_results {
    Some( mut paths ) => {
      paths.sort();
      return rsx! {
        PathPanelList { paths }
      }
    },
    None => return rsx! {
      div { "Loading Paths ..." }
    },
  }
}

#[component]
pub fn PathLoader(id: String) -> Element {
  let PathCache( path_cache ) = use_context::<PathCache>();
  match path_cache.get(&id) {
    Some( path ) => return rsx! {
      div {
        class: "column gap-large",
        PathPanel { path }
      }
    },
    None => return rsx! {
      div { "Path not found" }
    },
  }
}

#[component]
pub fn PathPanelList(paths: ReadOnlySignal<Vec<Path>>) -> Element {
  let paths = paths();
  return rsx! {
    div {
      class: "column gap-large",
      for path in paths {
        PathPanel { path }
      }
    }
  }
}


#[component]
pub fn PathPanel(path: ReadOnlySignal<Path>) -> Element {
  let path = path();
  let title = path.title;
  let optional_summary = path.summary;

  let SkillCache( skill_cache ) = use_context::<SkillCache>();
  let KeywordCache( keyword_cache ) = use_context::<KeywordCache>();
  let skill_ids = path.skill_ids.unwrap_or_default();
  let skills = skill_cache.from_object_ids(&skill_ids);
  let keyword_ids = keywords_from_skills(&skills);
  let (
    keystones,
    features,
    minor_features,
  ) = partion_skills_by_cost( skills );
  let keywords = rules_specific( keyword_cache.from_object_ids(&keyword_ids) );
  return rsx! {
    div {
      div { class: "title", "{title}" }
      if let Some( summary ) = optional_summary {
        div { "{summary}" }
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
        KeywordCardList { keywords }
      }
    }
  }
}


#[component]
pub fn PathTile(path: ReadOnlySignal<Path>) -> Element {
  let title = path().title;
  rsx!(
    div { "{title}" }
    if let Some( summary ) = path().summary {
      div { class: "small-text", "{summary}" }
    }
  )
}
