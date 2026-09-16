use std::collections::HashSet;

use dioxus::prelude::*;

use super::CharacterBuild;
use super::common::{CounterBadge, FilterButton, SectionBar};

use crate::common::{CollapsibleHeader, StaggeredCell, StaggeredGrid};
use crate::path::prelude::SelectionFilter;
use crate::skill::prelude::*;

use crate::server::prelude::SkillCache;
use crate::skill::component::SkillCard;
use crate::skill::Skill;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConstraintSet {
  pub required_weight: i32,
  pub selected_weight: i32,
  pub overage_total: i32,
  pub leeway: i32,
  pub filters: Vec<SelectionFilter>,
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
  let skill_ranks = skill_ranges
    .ranges
    .into_iter()
    .filter_map(|(id, range)| match skill_map.from_object_id(&id) {
      Some(skill) => Some((skill, range)),
      None => None,
    })
    .filter(
      |(skill, _)| match (include_keystones, &skill.training_cost) {
        (true, TrainingCost::Keystone) => true,
        (false, TrainingCost::Keystone) => false,
        _ => true,
      },
    )
    .filter(|(skill, _)| {
      category_filter.len() == 0
        || (category_filter.len() > 0 && category_filter.contains(&skill.category))
    })
    .collect::<Vec<_>>();
  let partitioned_skill_ranks = partitioned_sorted_skills(&skill_ranks);
  return rsx! {
    SectionBar {
      title: "Features",
      bar: rsx! { for counter in counters { CounterBadge { counter } } },
      div {
        class: "row-wrap align-center",
        div { class:"italics small-text","Filters"}
        FilterButton { title: "Mundane", value: Categorization::Mundane, filter_signal: category_filter_signal }
        FilterButton { title: "Innate", value: Categorization::Innate, filter_signal: category_filter_signal }
        FilterButton { title: "Resonance", value: Categorization::Resonance, filter_signal: category_filter_signal }
        FilterButton { title: "Magic", value: Categorization::Magic, filter_signal: category_filter_signal }
        FilterButton { title: "Show Keystones", value: 1, filter_signal: misc_filter_signal }
      }
      for (training, skill_ranges) in partitioned_skill_ranks {
        if skill_ranges.len() > 0 {
          CollapsibleHeader {
            class: "dotted-underline heavier slightlight",
            header: rsx! { "{training}s" },
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
pub fn SkillSelector(
  mut build_signal: Signal<CharacterBuild>, skill: Skill, min: i32, max: i32, current: i32,
) -> Element {
  let interactive = match skill.training_cost {
    TrainingCost::Inherient | TrainingCost::Keystone => false,
    _ => true,
  };
  let (input, click_event) = match (interactive, skill.is_ranked()) {
    (false, false) => (None, None),
    (false, true) => (
      Some(rsx! {
        div { class: "heavy", "{current} x" }
      }),
      None,
    ),
    (true, false) => (
      None,
      Some(EventHandler::new(move |event: Event<MouseData>| {
        event.stop_propagation();
        if !interactive {
          return;
        }
        let mut new_build = build_signal().clone();
        let new_rank = match (max <= 0, current <= 0) {
          (true, _) | (_, false) => 0,
          (_, true) => 1,
        };
        new_build.set_skill_ranks(&skill.id, new_rank);
        build_signal.set(new_build);
      })),
    ),
    (true, true) => (
      Some(rsx! {
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
      }),
      None,
    ),
  };
  let additional_classes = match (current > 0, max <= 0) {
    (true, _) => Some("selected".into()),
    (_, true) => Some("disabled".into()),
    _ => None,
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
