use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt;

use bson::oid::ObjectId;
use dioxus::prelude::*;

use crate::asset::icon::{IMG_SELECTED, IMG_UNSELECTED};
use crate::path::Path;
use crate::path::components::*;
use crate::progression::component::builder::{SelectionState};
use crate::progression::fixed::MIN_LEVEL;
use crate::progression::track::TrackContext;
use crate::progression::training::TrainingClass;
use crate::server::prelude::*;
use crate::skill::prelude::*;

#[component]
pub fn CharacterPaths(
  path_min: u32,
  path_max: u32,
  path_feature_count_signal: Signal<u32>,
  selected_paths: Signal<HashSet<String>>,
) -> Element {
  let PathCache(path_cache) = use_context::<PathCache>();
  let mut paths: Vec<Path> = path_cache.into_vec().into_iter().filter(
    |path|
    match path.inherient {
      Some(true ) => false,
      _ => true,
    }
  ).collect();
  paths.sort();
  let display_path_signal: Signal<Option<String>> = use_signal(|| None);
  let selected_path_count = (selected_paths)().len().try_into().ok().unwrap_or(0);
  let path_only = max(selected_path_count, path_min);
  let feature_max = if path_only >= path_max { 0 } else { path_max - path_only };
  let path_options_used = selected_path_count + path_feature_count_signal();
  let path_selection_state = 
  if path_options_used == path_max && selected_path_count >= path_min {
    SelectionState::Finished
  } else if path_options_used > path_max {
    SelectionState::Invalid
  } else {
    SelectionState::Unfinished
  };
  let next_class = match path_selection_state {
    SelectionState::Unfinished => "disabled",
    SelectionState::Finished => "",
    SelectionState::Invalid => "errored",
  };
  rsx! {
    div { "You must select at least one path, then you can choose up to X additional paths or gain additional features in your existing paths" }
    div {
      class: "path-grid",
      ExtraFeatureSelector { feature_max, path_feature_count_signal, path_selection_state }
      for path in paths {
        PathSelector { path, selected_paths, path_selection_state, display_path_signal }
      }
    }
    div {
      class: "next-button {next_class}",
      "Continue"
    }
  }
}

#[component]
pub fn ExtraFeatureSelector(
  feature_max: u32,
  path_feature_count_signal: Signal<u32>,
  path_selection_state: SelectionState,
) -> Element {
  let conditional_class = match (path_selection_state, path_feature_count_signal() > 0) {
    (_, true) | (SelectionState::Unfinished, _ ) => "",
    (SelectionState::Finished | SelectionState::Invalid, false) => "disabled",
  };
  rsx! {
    div {
      class: "path-card row {conditional_class}",
      input {
        class: "input",
        type: "number",
        value: path_feature_count_signal(),
        min: 0,
        max: feature_max,
        oninput: move |event| {
          let value = event.value().parse::<u32>().unwrap_or(0).min(feature_max);
          path_feature_count_signal.set(value);
        },
        onclick: move |event| {
          event.stop_propagation();
        }
      }
      div { class: "italics", "Extra Features" }
    }
  }
}

#[component]
pub fn PathSelector(
  path: Path,
  selected_paths: Signal<HashSet<String>>,
  display_path_signal: Signal<Option<String>>,
  path_selection_state: SelectionState,
) -> Element {
  let title = path.title.clone();
  let id = path.id.to_string();
  let (display, new_display_value) = match display_path_signal() {
    Some( display_id ) => match display_id.eq(&id) {
      true => (true, None),
      false => (false, Some(id.clone()))
    },
    _ => (false, Some(id.clone())),
  };
  let selected = selected_paths().contains(&id);
  let img_src = if selected { IMG_SELECTED } else { IMG_UNSELECTED };
  let conditional_class = match (path_selection_state, selected) {
    (_, true) | (SelectionState::Unfinished, _ ) => "",
    (SelectionState::Finished | SelectionState::Invalid, false) => "disabled",
  };
  return rsx! {
    div {
      class: "row path-card {conditional_class}",
      onclick: move |_| {
        display_path_signal.set(new_display_value.to_owned())
      },
      div {
        class: "path-checkbox-wrapper",
        onclick: move |evt: Event<MouseData>| {
          let mut cloned = selected_paths().clone();
          match (path_selection_state, selected) {
            (_, true) => cloned.remove(&id),
            (SelectionState::Unfinished, false) => cloned.insert(id.clone()),
            _ => false,
          };
          selected_paths.set(cloned);
          evt.stop_propagation();
        },
        img { src: "{img_src}" }
      }
      div { class: "path-title", "{title}" }
    }
    if display {
      div {
        class: "uv-full column gap-large {conditional_class}",
        PathPanel { path, hide_description: true }
      }
    }
  };
}