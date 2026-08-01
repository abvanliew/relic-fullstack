use std::collections::HashSet;

use bson::oid::ObjectId;
use dioxus::prelude::*;

use crate::asset::icon::{IMG_SELECTED, IMG_UNSELECTED};
use crate::common::StaggeredGrid;
use crate::path::components::PathPanel;
use crate::progression::fixed::{MAX_LEVEL, MIN_LEVEL};

use crate::builder::level_selections::{
  CharacterBuild, Counter, SelectionStatus, SelectionValidity,
};
use crate::progression::prelude::{TrainingClass, TrainingTable};
use crate::server::prelude::PathCache;

pub enum Interactible {
  Selectable,
  Deselectable,
  LockedOut,
  LockedIn,
}

pub fn interaction(state: &SelectionStatus, validity: &SelectionValidity) -> Interactible {
  return match (state, validity) {
    (SelectionStatus::SelectedCurrently, _) => Interactible::Deselectable,
    (SelectionStatus::SelectedPreviously, _) => Interactible::LockedIn,
    (SelectionStatus::Unselected, SelectionValidity::Available | SelectionValidity::Minimal) => {
      Interactible::Selectable
    },
    (SelectionStatus::Unselected, SelectionValidity::Full | SelectionValidity::Invalid) => {
      Interactible::LockedOut
    },
  };
}

#[component]
pub fn CharacterBuildPanel() -> Element {
  let build_signal = use_signal(|| CharacterBuild::default());
  // let build_debug = build_signal().clone();
  return rsx! {
    div{ class: "column gap-large",
      // div { "{build_debug:#?}" }
      LevelSelector { build_signal }
      PathSelector { build_signal }
      TrainingSelector { build_signal }
      FeatureSelector {}
      AttributeSelector {}
      EquipmentSelector {}
    }
  };
}

#[component]
pub fn LevelSelector(mut build_signal: Signal<CharacterBuild>) -> Element {
  let level = build_signal().get_level();
  rsx!(
    div {
      class: "row align-center",
      div { "Level" }
      div {
        select {
          autocomplete: "off",
          onchange: move |event| {
            let mut new_build = build_signal().clone();
            let new_level = event.value().parse::<i32>().ok().unwrap_or(MIN_LEVEL).max(MIN_LEVEL).min(MAX_LEVEL);
            if new_level != level {
              new_build.set_level(new_level);
              build_signal.set(new_build);
            }
          },
          for lvl in MIN_LEVEL..=MAX_LEVEL {
            option { value: lvl, label: lvl, selected: level == lvl, }
          }
        }
      }
    }
  )
}

#[component]
pub fn SectionBar(
  title: String, bar: Element, explainer: Element, children: Element,
  #[props(default)] display_default: bool,
) -> Element {
  let mut display_section = use_signal(|| display_default);
  let mut display_explainer = use_signal(|| false);
  return rsx! {
    div {
      class: "section thin-badge secondary-background",
      onclick: move |event| {
        event.stop_propagation();
        display_section.set(!display_section());
      },
      div { class: "heavier", "{title}" }
      div {
        class: "circle",
        onclick: move |event| {
          event.stop_propagation();
          if !display_section() {
            display_explainer.set(true);
            display_section.set(true);
          } else {
            display_explainer.set(!display_explainer());
          }
        },
        "i"
      }
      {bar}
    }
    if display_section() {
      if display_explainer() { {explainer} }
      {children}
    }
  };
}

#[component]
pub fn PathSelector(mut build_signal: Signal<CharacterBuild>) -> Element {
  let build = build_signal();
  let expanded_path: Signal<Option<ObjectId>> = use_signal(|| None);
  let filter_signal = use_signal(|| HashSet::<i32>::new());
  let filter_set = filter_signal();
  let path_cache = use_context::<PathCache>();
  let mut paths = path_cache.get_sorted_paths(false);
  let current_paths = build.get_current_paths();

  if filter_set.len() > 0 {
    paths = paths
      .into_iter()
      .filter(|path| match &path.order {
        Some(order) => filter_set.contains(&order.category),
        None => false,
      })
      .collect();
  }
  let (path_validity, count_required, count_initiate) = build.get_path_validation_status();
  let mut path_titles = path_cache.get_sorted_titles(&current_paths);
  if count_initiate.min < count_initiate.max {
    path_titles.push(format!(
      "+{} Bonus Features",
      count_initiate.max - count_initiate.min
    ));
  }
  let path_title_display = path_titles.join(", ");
  let bar = rsx! {
    CounterBadge { title: "Required", counter: count_required }
    CounterBadge { title: "Optional", counter: count_initiate }
    " {path_title_display}"
  };

  return rsx! {
    SectionBar {
      title: "Paths",
      bar,
      explainer: PathSelectionExplainer(),
      display_default: true,
      div {
        class: "row align-center",
        div { class:"italics small-text","Filters"}
        PathFilter { title: "Mundane", value: 1, filter_signal }
        PathFilter { title: "Innate", value: 2, filter_signal }
        PathFilter { title: "Resonant", value: 3, filter_signal }
        PathFilter { title: "Magic", value: 4, filter_signal }
      }
      div {
        class: "auto-flow-min flow-small",
        for path in paths {
          PathButton { title: path.title.clone(), id: path.id.clone(), build_signal, expanded_path, path_validity }
          match expanded_path() {
            Some(expand_id) => {
              rsx! {
                if expand_id.eq(&path.id) {
                  StaggeredGrid {
                    class: "uv-full stg-large flow-large",
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
pub fn CounterBadge(title: String, counter: Counter) -> Element {
  let min = counter.min;
  let max = counter.max;
  let extra_class = if min < max {
    "bg-warn"
  } else if min == max {
    "bg-info"
  } else {
    "bg-error"
  };
  return rsx! {
    div {
      class: "compact-badge column align-center small-text {extra_class}",
      div { "{title}" }
      div { "{min} / {max}" }
    }
  };
}

#[component]
pub fn PathFilter(title: String, value: i32, mut filter_signal: Signal<HashSet<i32>>) -> Element {
  let filter_set = filter_signal();
  let selected = filter_set.contains(&value);
  return rsx! {
    div {
      class: "row",
      div {
        class: if selected { "medium-border selected" } else { "thin-border" },
        onclick: move |event| {
          event.stop_propagation();
          let mut new_filter = filter_set.clone();
          match selected {
            true => new_filter.remove(&value),
            false => new_filter.insert(value),
          };
          filter_signal.set(new_filter);
        },
        "{title}"
      }
    }
  };
}

#[component]
pub fn PathButton(
  title: String, id: ObjectId, path_validity: SelectionValidity,
  mut build_signal: Signal<CharacterBuild>, mut expanded_path: Signal<Option<ObjectId>>,
) -> Element {
  let build = build_signal();
  let status = build.path_selection_status(&id);
  let expand = match expanded_path() {
    Some(expand_id) => expand_id == id,
    None => false,
  };
  let interactible = interaction(&status, &path_validity);
  let (img_src, extra_classes) = match interactible {
    Interactible::Selectable => (IMG_UNSELECTED, ""),
    Interactible::Deselectable => (IMG_SELECTED, ""),
    Interactible::LockedOut => (IMG_UNSELECTED, "disabled"),
    Interactible::LockedIn => (IMG_SELECTED, "disabled"),
  };
  return rsx! {
    div {
      class: "card-snug row align-center underhang {extra_classes}",
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
            Interactible::Selectable => {
              let mut new_build = build.clone();
              new_build.add_path(id);
              build_signal.set(new_build);
            },
            Interactible::Deselectable => {
              let mut new_build = build.clone();
              new_build.remove_path(&id);
              build_signal.set(new_build);
            },
            _ => (),
          };
        },
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
    div { "Each character has to select at least one path at character creation and when starting a new tier (levels 7 and 13). You can optionally gain additional Paths up to the current path limit at your level." }
  };
}

#[component]
pub fn TrainingSelector(mut build_signal: Signal<CharacterBuild>) -> Element {
  let build = build_signal();
  let expand_signal: Signal<Option<TrainingClass>> = use_signal(|| None);
  let max = build.get_level();
  let previous_training = build.get_previous_trainings();
  let current_training = build.get_current_trainings();
  let summary = current_training.summary();
  let sum = current_training.sum();
  let total = build.get_training_ranks();
  let counter = Counter {
    min: sum,
    max: total,
  };
  let remaining_ranks = total - sum;
  let modifiers = build.get_training_modifiers();

  return rsx! {
    SectionBar {
      title: "Trainings",
      bar: rsx! {
        CounterBadge { title: "Ranks", counter }
        "{summary}"
      },
      explainer: rsx! {},
      div { "Bonuses: {modifiers}" }
      div {
        class: "auto-flow-min flow-xsmall",
        TrainingPicker {
          build_signal,
          training_class: TrainingClass::Adept,
          current: current_training.adept.unwrap_or(0),
          min: previous_training.adept.unwrap_or(0),
          max,
          remaining_ranks,
          expand_signal,
        }
        TrainingPicker {
          build_signal,
          training_class: TrainingClass::Endurance,
          current: current_training.endurance.unwrap_or(0),
          min: previous_training.endurance.unwrap_or(0),
          max,
          remaining_ranks,
          expand_signal,
        }
        TrainingPicker {
          build_signal,
          training_class: TrainingClass::Expert,
          current: current_training.expert.unwrap_or(0),
          min: previous_training.expert.unwrap_or(0),
          max,
          remaining_ranks,
          expand_signal,
        }
        TrainingPicker {
          build_signal,
          training_class: TrainingClass::Innate,
          current: current_training.innate.unwrap_or(0),
          min: previous_training.innate.unwrap_or(0),
          max,
          remaining_ranks,
          expand_signal,
        }
        TrainingPicker {
          build_signal,
          training_class: TrainingClass::Resonance,
          current: current_training.resonant.unwrap_or(0),
          min: previous_training.resonant.unwrap_or(0),
          max,
          remaining_ranks,
          expand_signal,
        }
        TrainingPicker {
          build_signal,
          training_class: TrainingClass::Magic,
          current: current_training.magic.unwrap_or(0),
          min: previous_training.magic.unwrap_or(0),
          max,
          remaining_ranks,
          expand_signal,
        }
      }
    }
  };
}

#[component]
pub fn TrainingPicker(
  mut build_signal: Signal<CharacterBuild>, training_class: TrainingClass, current: i32, min: i32,
  max: i32, remaining_ranks: i32, expand_signal: Signal<Option<TrainingClass>>,
) -> Element {
  let build = build_signal();
  let expanded = match expand_signal() {
    Some(selected_class) => selected_class.eq(&training_class),
    None => false,
  };
  let max_rank = max.min(remaining_ranks + current);
  let disabled = min == max_rank;
  return rsx! {
    div {
      class: if expanded {"medium-border selected underhang"} else {"thin-border underhang"},
      onclick: move |event| {
        expand_signal.set(if expanded {None} else {Some(training_class.clone())});
        event.stop_propagation();
      },
      "{training_class}"
      input {
        class: if disabled {"input disabled"} else {"input"}, type: "number",
        value: current, min, max: max_rank,
        oninput: move |event| {
          let value = event.value().parse::<i32>()
          .unwrap_or_default()
          .min(max_rank).max(min);
          let mut new_build = build.clone();
          new_build.set_training(&training_class, value);
          build_signal.set(new_build);
        },
        onclick: move |event| {
          event.stop_propagation();
        }
      }
    }
    if expanded {
      div {
        class: "uv-full",
        TrainingTable { training_class, highlight_rank: Some( current ) }
      }
    }
  };
}

#[component]
pub fn FeatureSelector() -> Element {
  return rsx! {
    SectionBar {
      title: "Features",
      bar: rsx! {},
      explainer: rsx! {},

    }
  };
}

#[component]
pub fn AttributeSelector() -> Element {
  return rsx! {
    SectionBar {
      title: "Attributes",
      bar: rsx! {},
      explainer: rsx! {},
    }
  };
}

#[component]
pub fn EquipmentSelector() -> Element {
  return rsx! {
    SectionBar {
      title: "Equipment",
      bar: rsx! {},
      explainer: rsx! {},

    }
  };
}
