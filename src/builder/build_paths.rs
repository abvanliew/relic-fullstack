use std::collections::HashSet;

use bson::oid::ObjectId;
use dioxus::prelude::*;

use super::common::{CounterBadge, FilterButton, SectionBar, Interactable, interaction};
use super::CharacterBuild;

use crate::asset::icon::{IMG_SELECTED, IMG_UNSELECTED};
use crate::common::StaggeredGrid;
use crate::path::components::PathPanel;
use crate::skill::prelude::*;
use crate::builder::character_build::{
 SelectionStatus, SelectionValidity,
};
use crate::server::prelude::PathCache;

impl CharacterBuild {
  pub fn path_selection_status(&self, path_id: &ObjectId) -> SelectionStatus {
    let level_selection = self.current_selection();
    return match (level_selection.paths.get(path_id), self.get_previous_paths().get(path_id)) {
      (Some(_), _) =>SelectionStatus::SelectedCurrently,
      (_, Some(_)) => SelectionStatus::SelectedPreviously,
      _ => SelectionStatus::Unselected,
    };
  }
}

#[component]
pub fn PathGroup(mut build_signal: Signal<CharacterBuild>) -> Element {
  let build = build_signal();
  let expanded_path: Signal<Option<ObjectId>> = use_signal(|| None);
  let filter_signal = use_signal(|| HashSet::<Categorization>::new());
  let filter_set = filter_signal();
  let path_cache = use_context::<PathCache>();
  let mut paths = path_cache.get_sorted_paths(false);
  let current_paths = build.get_current_path_ids();

  if filter_set.len() > 0 {
    paths = paths
      .into_iter()
      .filter(|path| filter_set.contains(&path.category))
      .collect();
  }
  let (path_validity, count_required, count_initiate) = build.get_path_validation_status();
  let mut path_titles = path_cache.get_sorted_titles(&current_paths);
  if count_initiate.current < count_initiate.max {
    path_titles.push(format!(
      "+{} Bonus Features",
      count_initiate.max - count_initiate.current
    ));
  }
  let path_title_display = path_titles.join(", ");
  let bar = rsx! {
    CounterBadge { counter: count_required }
    CounterBadge { counter: count_initiate }
    " {path_title_display}"
  };

  return rsx! {
    SectionBar {
      title: "Paths",
      bar,
      explainer: PathSelectionExplainer(),
      div {
        class: "row align-center",
        div { class:"italics small-text","Filters"}
        FilterButton { title: "Mundane", value: Categorization::Mundane, filter_signal }
        FilterButton { title: "Innate", value: Categorization::Innate, filter_signal }
        FilterButton { title: "Resonance", value: Categorization::Resonance, filter_signal }
        FilterButton { title: "Magic", value: Categorization::Magic, filter_signal }
      }
      div {
        class: "auto-flow-min flow-small",
        for path in paths {
          PathSelector { title: path.title.clone(), id: path.id.clone(), build_signal, expanded_path, path_validity }
          match expanded_path() {
            Some(expand_id) => {
              rsx! {
                if expand_id.eq(&path.id) {
                  StaggeredGrid {
                    class: "uv-full spacer stg-large flow-large",
                    PathPanel { path, hide_description: true }
                  }
                }
              }
            },
            None => rsx! {},
          }
        }
      }
    }
  };
}


#[component]
pub fn PathSelector(
  title: String, id: ObjectId, path_validity: SelectionValidity,
  mut build_signal: Signal<CharacterBuild>, mut expanded_path: Signal<Option<ObjectId>>,
) -> Element {
  let build = build_signal();
  let status = build.path_selection_status(&id);
  let expand = match expanded_path() {
    Some(expand_id) => expand_id == id,
    None => false,
  };
  let more_classes = if expand { "selected" } else { "" };
  let interactible = interaction(&status, &path_validity);
  let (img_src, extra_classes, img_class) = match interactible {
    Interactable::Selectable => (IMG_UNSELECTED, "", ""),
    Interactable::Deselectable => (IMG_SELECTED, "", "selected-filter"),
    Interactable::LockedOut => (IMG_UNSELECTED, "disabled", ""),
    Interactable::LockedIn => (IMG_SELECTED, "disabled", ""),
  };
  return rsx! {
    div {
      class: "card-snug minimal-background row align-center underhang {more_classes} {extra_classes}",
      onclick: move |event| {
        event.stop_propagation();
        if expand {
          expanded_path.set(None);
        } else {
          expanded_path.set(Some(id.clone()));
        }
      },
      img {
        onclick: move |event| {
          event.stop_propagation();
          match &interactible {
            Interactable::Selectable => {
              let mut new_build = build.clone();
              new_build.add_path(id);
              build_signal.set(new_build);
            },
            Interactable::Deselectable => {
              let mut new_build = build.clone();
              new_build.remove_path(&id);
              build_signal.set(new_build);
            },
            _ => (),
          };
        },
        class: "{img_class}",
        src: "{img_src}"
      }
      span { "{title}" }
    }
  };
}

#[component]
pub fn PathSelectionExplainer() -> Element {
  return rsx! {
    div { "Paths are central to each character, they provide a number of Keystone features that are granted to anyone on the path and then provide a pool of features you can choose from. These features range from Skills, Spells or additional flat bonuses." }
    div { "Each character has to select at least one path at character creation and when starting a new tier (levels 7 and 13). You can gain additional paths up to a maximum value listed on the level chart. If you do not select optional paths you gain more features to spend on your other paths." }
  };
}