use std::hash::Hash;
use std::collections::HashSet;


use bson::oid::ObjectId;
use dioxus::prelude::*;

use crate::asset::icon::{IMG_SELECTED, IMG_UNSELECTED};
use crate::common::{StaggeredCell, StaggeredGrid};
use crate::path::components::PathPanel;
use crate::progression::fixed::{MAX_LEVEL, MIN_LEVEL};
use crate::skill::prelude::*;

use crate::builder::character_build::{
  CharacterBuild, Counter, SelectionStatus, SelectionValidity,
};
use crate::progression::prelude::{LevelTable, TrainingClass, TrainingTable};
use crate::server::prelude::{PathCache, SkillCache};
use crate::skill::component::SkillCard;
use crate::skill::Skill;

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
      CharacterGroup { build_signal }
      PathGroup { build_signal }
      TrainingGroup { build_signal }
      FeatureGroup { build_signal }
      AttributeSelector { build_signal }
      EquipmentSelector { build_signal }
    }
  };
}

#[component]
pub fn CharacterGroup(mut build_signal: Signal<CharacterBuild>) -> Element {
  let level = build_signal().get_level();
  rsx! {
    SectionBar {
      title: "Character",
      bar: rsx! {
        "Level"
        select {
          class: "big-text",
          autocomplete: "off",
          onclick: move |event| { event.stop_propagation(); },
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
      },
      explainer: rsx! {},
      LevelTable { highlight_level: level }
    }
  }
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
      div { class: "section", {bar} }
    }
    if display_section() {
      if display_explainer() { {explainer} }
      {children}
    }
  };
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
    CounterBadge { title: "Required", counter: count_required }
    CounterBadge { title: "Optional", counter: count_initiate }
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
  let (value, remainder, max) = counter.effective();
  let term = match ( value == 0, remainder == 0 ) {
    ( true, true ) => format!("0"),
    ( true, false ) => format!("\u{00BD}"),
    ( false, true ) => format!("{value}"),
    ( false, false ) => format!("{value} \u{00BD}"),
  };
  let extra_class = match counter.valid() {
    SelectionValidity::Available | SelectionValidity::Minimal => "bg-warn",
    SelectionValidity::Full => "bg-info",
    SelectionValidity::Invalid => "bg-error",
  };
  return rsx! {
    div {
      class: "compact-badge column align-center small-text {extra_class}",
      div { "{title}" }
      div { "{term} / {max}" }
    }
  };
}

#[component]
pub fn FilterButton<T: 'static>(title: String, value: T, mut filter_signal: Signal<HashSet<T>>) -> Element where T: Eq, T: Clone + PartialEq + Eq + Hash {
  let filter_set = filter_signal();
  let selected = filter_set.contains(&value);
  return rsx! {
    div {
      class: "row",
      div {
        class: if selected { "medium-border selected" } else { "thin-border minimal-background" },
        onclick: move |event| {
          event.stop_propagation();
          let mut new_filter = filter_set.clone();
          match selected {
            true => new_filter.remove(&value),
            false => new_filter.insert(value.clone()),
          };
          filter_signal.set(new_filter);
        },
        "{title}"
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
    Interactible::Selectable => (IMG_UNSELECTED, "", ""),
    Interactible::Deselectable => (IMG_SELECTED, "", "selected-filter"),
    Interactible::LockedOut => (IMG_UNSELECTED, "disabled", ""),
    Interactible::LockedIn => (IMG_SELECTED, "disabled", ""),
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
    div { "Each character has to select at least one path at character creation and when starting a new tier (levels 7 and 13). You can optionally gain additional Paths up to the current path limit at your level." }
  };
}

#[component]
pub fn TrainingGroup(mut build_signal: Signal<CharacterBuild>) -> Element {
  let build = build_signal();
  let expand_signal: Signal<Option<TrainingClass>> = use_signal(|| None);
  let max = build.get_level();
  let previous_training = build.get_previous_trainings();
  let current_training = build.get_current_trainings();
  let summary = current_training.summary();
  let sum = current_training.sum();
  let total = build.get_training_ranks();
  let counter = Counter {
    current: sum,
    max: total,
    ..Default::default()
  };
  let remaining_ranks = total - sum;
  let modifiers = build.get_training_modifiers();
  build.get_path_constraints();
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
        for class in TrainingClass::ordered() {
          TrainingSelector {
            build_signal,
            training_class: class.clone(),
            current: current_training.get(&class),
            min: previous_training.get(&class),
            max,
            remaining_ranks,
            expand_signal,
          }
        }
      }
    }
  };
}

#[component]
pub fn TrainingSelector(
  mut build_signal: Signal<CharacterBuild>, training_class: TrainingClass, current: i32, min: i32,
  max: i32, remaining_ranks: i32, expand_signal: Signal<Option<TrainingClass>>,
) -> Element {
  let build = build_signal();
  let expanded: bool = match expand_signal() {
    Some(selected_class) => selected_class.eq(&training_class),
    None => false,
  };
  let max_rank = max.min(remaining_ranks + current);
  let disabled = min == max_rank;
  return rsx! {
    div {
      class: if expanded {"medium-border selected underhang align-center"} else {"thin-border minimal-background underhang align-center"},
      onclick: move |event| {
        expand_signal.set(if expanded {None} else {Some(training_class.clone())});
        event.stop_propagation();
      },
      input {
        class: if disabled {"input disabled big-text bumper"} else {"input big-text bumper"}, type: "number",
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
      " {training_class}"
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
pub fn FeatureGroup(mut build_signal: Signal<CharacterBuild>) -> Element {
  let SkillCache(ref skill_map) = use_context();
  let build = build_signal();
  let category_filter_signal = use_signal(|| HashSet::<Categorization>::new());
  let category_filter = category_filter_signal();
  let misc_filter_signal = use_signal(|| HashSet::<i32>::new());
  let include_keystones = misc_filter_signal().contains(&1);

  let (skill_ranges, counters) = build.get_skill_ranges();
  let skill_ranks = skill_ranges.ranges.into_iter()
    .filter_map(|(id, range)| {
      match skill_map.from_object_id(&id) {
        Some(skill) => Some((skill, range)),
        None => None,
      }
    })
    .filter(|(skill, _)|
      match (include_keystones, &skill.training_cost) {
        ( true, TrainingCost::Keystone ) => true,
        ( false, TrainingCost::Keystone ) => false,
        _ => true,
      })
    .filter(|(skill, _)|
      category_filter.len() == 0 || ( category_filter.len() > 0 && category_filter.contains(&skill.category) )
    )
    .collect::<Vec<_>>();
  let partitioned_skill_ranks = partitioned_sorted_skills(&skill_ranks);
  return rsx! {
    SectionBar {
      title: "Features",
      bar: rsx! {
        for counter in counters {
          FeatureSelectionBadge { counter }
        }
      },
      explainer: rsx! {},
      div {
        class: "row align-center",
        div { class:"italics small-text","Filters"}
        FilterButton { title: "Mundane", value: Categorization::Mundane, filter_signal: category_filter_signal }
        FilterButton { title: "Innate", value: Categorization::Innate, filter_signal: category_filter_signal }
        FilterButton { title: "Resonance", value: Categorization::Resonance, filter_signal: category_filter_signal }
        FilterButton { title: "Magic", value: Categorization::Magic, filter_signal: category_filter_signal }
        FilterButton { title: "Show Keystones", value: 1, filter_signal: misc_filter_signal }
      }
      for (training, skill_ranges) in partitioned_skill_ranks {
        if skill_ranges.len() > 0 {
          CollapsibleSection {
            class: "dotted-underline heavier slightlight",
            section: rsx! { "{training}s" },
            StaggeredGrid {
              class: "stg-large",
              for (skill, range) in skill_ranges {
                StaggeredCell {
                  SkillSelector {
                    build_signal,
                    skill, current: range.current,
                    min: range.min,
                    max: range.max,
                  }
                }
              }
            }
          }
        }
      }
    }
  };
}

#[component]
pub fn FeatureSelectionBadge(counter: Counter) -> Element {
  let title = counter.title.clone().unwrap_or("undefined".into());
  return rsx! {
    CounterBadge { title, counter }
  };
}

#[component]
pub fn CollapsibleSection(class: String, section: Element, children: Element) -> Element {
  let mut display = use_signal(|| true);
  return rsx! {
    div {
      class,
      onclick: move |event| {
        event.stop_propagation();
        display.set(!display());
      },
      {section}
    }
    if display() {
      {children}
    }
  }
}

#[component]
pub fn SkillSelector(mut build_signal: Signal<CharacterBuild>, skill: Skill, min: i32, max: i32, current: i32 ) -> Element {
  let interactive = match skill.training_cost {
    TrainingCost::Inherient | TrainingCost::Keystone => false,
    _ => true,
  };
  let (input, click_event) = match (interactive, skill.is_ranked()) {
    (false, false) => (None, None),
    (false, true) => (
      Some( rsx! {
        div { class: "heavy", "{current} x" }
      } ),
      None,
    ),
    (true, false) => (
      None, 
      Some( EventHandler::new(
        move |event: Event<MouseData>| {
          event.stop_propagation();
          if !interactive { return; }
          let mut new_build = build_signal().clone();
          let new_rank = match (max <= 0, current <=0 ) {
            (true, _) | (_, false) => 0,
            (_, true) => 1,
          };
          new_build.set_skill_ranks(&skill.id, new_rank);
          build_signal.set(new_build);
        } 
      ) ),
    ),
    (true, true) => (
      Some( rsx! {
        input {
          class: "input", type: "number",
          value: current, min: min, max: max,
          oninput: move |event| {
            let value = event.value().parse::<i32>()
            .unwrap_or_default()
            .min(max).max(min);
            let mut new_build = build_signal().clone();
            new_build.set_skill_ranks(&skill.id, value);
            build_signal.set(new_build);
          },
          onclick: move |event| {
            event.stop_propagation();
          }
        }
      } ), 
      None
    ),
  };
  let additional_classes =  match ( current > 0, max <= 0 ) {
    (true, _) => Some("selected".into()),
    (_, true) => Some("disabled".into()),
    _ => None
  };
  return rsx! {
    SkillCard {
      skill,
      click_event,
      input,
      additional_classes,
      include_path_chips: true
    }
  };
}

#[component]
pub fn AttributeSelector(mut build_signal: Signal<CharacterBuild>) -> Element {
  return rsx! {
    SectionBar {
      title: "Attributes",
      bar: rsx! {},
      explainer: rsx! {},
    }
  };
}

#[component]
pub fn EquipmentSelector(mut build_signal: Signal<CharacterBuild>) -> Element {
  return rsx! {
    SectionBar {
      title: "Equipment",
      bar: rsx! {},
      explainer: rsx! {},

    }
  };
}
