use std::collections::HashSet;

use dioxus::prelude::*;

use super::CharacterBuild;

use super::common::{CounterBadge, FilterButton, SectionBar};
use crate::character::prelude::Flow;
use crate::common::{CollapsibleHeader, StaggeredCell, StaggeredGrid};

use crate::modifiers::ModifierClass;
use crate::path::prelude::{Constraint, SelectionFilter, SkillFilter};
use crate::server::prelude::SkillCache;
use crate::skill::component::SkillCard;
use crate::skill::prelude::*;
use crate::skill::Skill;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConstraintSet {
  pub required_weight: i32,
  pub selected_weight: i32,
  pub overage_total: i32,
  pub leeway: i32,
  pub filters: Vec<SelectionFilter>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureCounter {
  pub path_name: String,
  pub skill_filter: SkillFilter,
  pub weight: i32,
}

impl FeatureCounter {
  pub fn from_constraint(constraint: &Constraint, path_name: &String) -> Self {
    return Self {
      path_name: match &constraint.filter.skill_filter {
        SkillFilter::Features | SkillFilter::CoreFeatures => format!("{path_name}"),
        SkillFilter::MinorFeatures | SkillFilter::CoreMinorFeatures => {
          format!("{path_name} - Minors")
        },
        SkillFilter::Cantrips => format!("{path_name} - Cantrips"),
        SkillFilter::Spells => format!("{path_name} - Spells"),
      },
      skill_filter: constraint.filter.skill_filter.clone(),
      weight: constraint.required_weight,
    };
  }
}

impl CharacterBuild {
  pub fn get_current_flows(&self) -> (i32, i32, i32) {
    let modifiers = self.get_current_modifiers();
    return (
      modifiers.get(&ModifierClass::InnateFlow),
      modifiers.get(&ModifierClass::ResonanceFlow),
      modifiers.get(&ModifierClass::MagicFlow),
    );
  }
}

#[component]
pub fn FeatureGroup(mut build_signal: Signal<CharacterBuild>) -> Element {
  let SkillCache(ref skill_map) = use_context();
  let build = build_signal();
  let category_filter_signal = use_signal(|| HashSet::<Categorization>::new());
  let category_filter = category_filter_signal();
  let misc_filter_signal = use_signal(|| HashSet::<i32>::new());
  let include_keystones = misc_filter_signal().contains(&1);
  let show_overcosted = misc_filter_signal().contains(&2);
  let flows = build.get_current_flows();

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
    .filter(|(skill, _)| {
      match (show_overcosted, skill.minimum_resource_cost(), skill.resource_cost_flow()) {
        (true, _, _ ) | (_, _, None) => true,
        (_, cost, Some( Flow::Innate )) => cost <= flows.0,
        (_, cost, Some( Flow::Resonance )) => cost <= flows.1,
        (_, cost, Some( Flow::Magic )) => cost <= flows.2,
      }
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
        FilterButton { title: "Show Overcosted", value: 2, filter_signal: misc_filter_signal }
      }
      for (training, skill_ranges) in partitioned_skill_ranks {
        if skill_ranges.len() > 0 {
          CollapsibleHeader {
            class: "thin-border minimal-background heavier slightlight",
            header: rsx! { "{training}s" },
            StaggeredGrid {
              class: "indent stg-large",
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
  let (input, onclick) = match (interactive, skill.is_ranked()) {
    (false, false) => (None, None),
    (false, true) => (
      Some(rsx! {
        div { class: "heavy", "{current} x" }
      }),
      None,
    ),
    (true, false) => (
      None,
      Some(Callback::new(move |_: MouseEvent| {
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
      }))
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
      onclick,
      input,
      additional_classes,
      include_path_chips: true
    }
  };
}
