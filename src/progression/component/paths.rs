use dioxus::prelude::*;
use std::cmp::max;
use std::collections::HashSet;

use super::SelectionState;
use crate::asset::icon::{IMG_SELECTED, IMG_UNSELECTED};
use crate::common::*;
use crate::path::components::*;
use crate::path::Path;

#[component]
pub fn CharacterPaths(
  path_options: Vec<Path>, path_min: i32, path_max: i32, extra_features_signal: Signal<i32>,
  selected_paths: Signal<HashSet<String>>,
) -> Element {
  let display_path_signal: Signal<Option<String>> = use_signal(|| None);
  let selected_path_count = (selected_paths)().len().try_into().ok().unwrap_or(0);
  let path_only = max(selected_path_count, path_min);
  let feature_max = if path_only >= path_max {
    0
  } else {
    path_max - path_only
  };
  let path_options_used = selected_path_count + extra_features_signal();
  let path_selection_state = if path_options_used == path_max && selected_path_count >= path_min {
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
    div {
      class: "underhang",
      "Each character has one or more paths that they can utilitize. A path grants a character several skills or passive boons for joining, known as the Keystone features. The majority of a path is a collection of Skills and Spells that a character can choose to learn."
    }
    div {
      class: "underhang",
      "While it is encouraged to select multiple different paths to explore the different combinations you can come up with. If you want to travel down a single path (or just not the maximum for your level) you can pick the extra features option to gain more choices from your current path(s)."
    }
    div {
      class: "path-grid",
      ExtraFeatureSelector { feature_max, extra_features_signal, path_selection_state }
      for path in path_options {
        PathSelector { path, selected_paths, path_selection_state, display_path_signal }
      }
      div {
        class: "uv-full",
        div {
          class: "next-button {next_class}",
          "Continue"
        }
      }
    }
  }
}

#[component]
pub fn ExtraFeatureSelector(
  feature_max: i32, extra_features_signal: Signal<i32>, path_selection_state: SelectionState,
) -> Element {
  let conditional_class = match (path_selection_state, extra_features_signal() > 0) {
    (_, true) | (SelectionState::Unfinished, _) => "",
    (SelectionState::Finished | SelectionState::Invalid, false) => "disabled",
  };
  rsx! {
    div {
      class: "path-card row {conditional_class}",
      InputSignal {
        rank: extra_features_signal,
        max_rank: feature_max,
      }
      div { class: "italics", "Extra Features" }
    }
  }
}

#[component]
pub fn PathSelector(
  path: Path, selected_paths: Signal<HashSet<String>>, display_path_signal: Signal<Option<String>>,
  path_selection_state: SelectionState,
) -> Element {
  let title = path.title.clone();
  let id = path.id.to_string();
  let (display, new_display_value) = match display_path_signal() {
    Some(display_id) => match display_id.eq(&id) {
      true => (true, None),
      false => (false, Some(id.clone())),
    },
    _ => (false, Some(id.clone())),
  };
  let selected = selected_paths().contains(&id);
  let img_src = if selected {
    IMG_SELECTED
  } else {
    IMG_UNSELECTED
  };
  let mut conditional_class = match (path_selection_state, selected, display) {
    (_, true, _) | (SelectionState::Unfinished, _, _) => "",
    (SelectionState::Finished | SelectionState::Invalid, false, _) => "disabled",
  }
  .into();
  if display {
    conditional_class = format!("{conditional_class} selected medium-border",);
  }
  let highlight_class = if display { "path-card-highlight" } else { "" };
  return rsx! {
    div {
      class: "row path-card {highlight_class} {conditional_class}",
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
        class: "uv-full column gap-large path-skil-wrapper {conditional_class}",
        PathPanel { path, hide_description: true }
      }
    }
  };
}
