use std::collections::HashMap;

use dioxus::prelude::*;

use super::build_common::{Counter, CounterBadge, SectionBar};
use super::CharacterBuild;

use crate::common::{NumericInput, NumericRange};
use crate::modifiers::ModifierSet;
use crate::progression::prelude::{DevelopmentClass, DevelopmentTable, GrowthTrack};

#[derive(Debug, Clone, Default)]
pub struct Development {
  pub development_points: HashMap<DevelopmentClass, i32>,
}

impl Development {
  pub fn extend(&mut self, other: &Self) {
    self.development_points.extend(other.development_points.clone());
  }

  pub fn get(&self, class: &DevelopmentClass) -> i32 {
    return *self.development_points.get(class).unwrap_or(&0);
  }

  pub fn set(&mut self, class: &DevelopmentClass, points: i32) {
    self.development_points.insert(class.clone(), points);
  }

  pub fn unset(&mut self, class: &DevelopmentClass) {
    self.development_points.remove(class);
  }

  pub fn sum(&self) -> i32 {
    return DevelopmentClass::iter()
    .map(|class| self.development_points.get(class).unwrap_or(&0))
    .sum::<i32>()
  }

  pub fn summary(&self) -> String {
    let items: Vec<String> = DevelopmentClass::iter()
    .map(|class| option_formater(class.to_string(), self.development_points.get(class)))
    .flatten()
    .collect();
    return items.join(", ");
  }

  pub fn get_modifiers(&self) -> ModifierSet {
    let mut modifiers = ModifierSet::default();
    for class in DevelopmentClass::iter() {
      modifiers.append(&GrowthTrack::class_at(&class, self.get(&class)));
    }
    return modifiers;
  }
}

impl CharacterBuild {
  pub fn set_development(&mut self, class: &DevelopmentClass, value: i32) {
    let current = self.get_current_mut();
    current.development.set(class, value)
  }

  pub fn unset_development(&mut self, class: &DevelopmentClass) {
    let current = self.get_current_mut();
    current.development.unset(class)
  }

  pub fn get_previous_development(&self) -> Development {
    let mut previous_trainings = Development::default();
    for selections in self.previous_level_selections() {
      previous_trainings.extend(&selections.development);
    }
    return previous_trainings;
  }

  pub fn get_current_development(&self) -> Development {
    let mut previous_training = self.get_previous_development();
    let Some(level_selection) = self.current_selection_ref() else {
      return previous_training;
    };
    previous_training.extend(&level_selection.development);
    return previous_training;
  }

  pub fn get_training_modifiers(&self) -> ModifierSet {
    let trainings = self.get_current_development();
    let mut modifiers = ModifierSet::default();
    for class in DevelopmentClass::iter() {
      modifiers.append(&GrowthTrack::class_at(&class, trainings.get(&class)));
    }
    return modifiers;
  }

  pub fn development_constraints(&self) -> (
    Vec<(DevelopmentClass, NumericRange)>,
    Vec<Counter>,
  ) {
    let (previous_selections, current_selections) = self.get_selection_pair();
    let previous_development = previous_selections.development;
    let current_development = current_selections.development;
    let max = self.get_level();
    let sum = current_development.sum();
    let total = self.get_training_ranks();
    let remaining_ranks = total - sum;
    let development_ranges = DevelopmentClass::iter()
    .map(|class| {
      let previous = previous_development.get(class);
      let current = current_development.get(class);
      (
        class.clone(),
        NumericRange::new(previous, current, max.min(remaining_ranks + current))
      )
    })
    .collect();
    let counter = vec![ Counter {
      title: "Points".into(),
      current: sum,
      max: total,
      ..Default::default()
    }];
    return (development_ranges, counter)
  }
}

fn option_formater(title: String, value: Option<&i32>) -> Option<String> {
  return match value {
    Some(value) => {
      if value.eq(&0) {
        None
      } else {
        Some(format!("{title} {value}"))
      }
    },
    None => None,
  };
}

#[component]
pub fn GrowthGroup(mut build_signal: Signal<CharacterBuild>) -> Element {
  let expand_signal: Signal<Option<DevelopmentClass>> = use_signal(|| None);
  let build = build_signal();
  let (development_ranges, counters) = build.development_constraints();
  let current_development = build.get_current_development();
  let summary = current_development.summary();
  let modifiers = current_development.get_modifiers();
  build.get_path_constraints();
  return rsx! {
    SectionBar {
      title: "Development",
      bar: rsx! {
        for counter in counters {
          CounterBadge { counter }
        }
        "{summary}"
      },
      explainer: rsx! {
        div { "Each level characters gain development points which can be spent in one of six categories. These development points scale up hit points, attributes, specializations, flows and resource pools. You cannot spend more points in a give development than your current level. You can click on each one to see the table of bonuses they provide." }
      },
      div { "Bonuses: {modifiers}" }
      div {
        class: "auto-flow-min flow-xsmall",
        for (development_class, range) in development_ranges {
          GrowthSelector {
            build_signal, development_class, range, expand_signal
          }
        }
      }
    }
  };
}

#[component]
pub fn GrowthSelector(
  mut build_signal: Signal<CharacterBuild>,
  development_class: DevelopmentClass,
  range: NumericRange,
  expand_signal: Signal<Option<DevelopmentClass>>,
) -> Element {
  let expanded: bool = match expand_signal() {
    Some(selected_class) => selected_class.eq(&development_class),
    None => false,
  };
  let min = range.min;
  let current = range.value;
  return rsx! {
    div {
      class: if expanded {
        "medium-border selected underhang no-select align-center"
      } else {
        "thin-border minimal-background underhang no-select align-center"
      },
      onclick: move |event| {
        event.stop_propagation();
        expand_signal.set(if expanded {None} else {Some(development_class.clone())});
      },
      NumericInput {
        class: "input big-text bumper",
        range,
        value_handler: move |value: i32| {
          let mut new_build = build_signal().clone();
          if value == min {
            new_build.unset_development(&development_class);
          } else {
            new_build.set_development(&development_class, value);
          }
          build_signal.set(new_build);
        },
      }
      " {development_class}"
    }
    if expanded {
      div {
        class: "uv-full",
        DevelopmentTable { development_class, highlight_rank: Some( current ) }
      }
    }
  }
}

#[component]
pub fn TrainingSelector(
  mut build_signal: Signal<CharacterBuild>, training_class: DevelopmentClass, 
  current: i32, min: i32, max: i32, remaining_ranks: i32, 
  expand_signal: Signal<Option<DevelopmentClass>>,
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
      class: if expanded {"medium-border selected underhang no-select align-center"} else {"thin-border minimal-background underhang no-select align-center"},
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
          new_build.set_development(&training_class, value);
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
        DevelopmentTable { development_class: training_class, highlight_rank: Some( current ) }
      }
    }
  }
}
