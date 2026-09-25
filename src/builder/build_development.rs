use dioxus::prelude::*;

use super::common::{Counter, CounterBadge, SectionBar};
use super::CharacterBuild;

use crate::progression::prelude::{TrainingClass, DevelopmentTable};

#[derive(Debug, Clone, Default)]
pub struct Development {
  pub adept: Option<i32>,
  pub endurance: Option<i32>,
  pub expert: Option<i32>,
  pub innate: Option<i32>,
  pub resonant: Option<i32>,
  pub magic: Option<i32>,
}

impl Development {
  pub fn extend(&mut self, other: &Self) {
    self.adept = optional_max(&self.adept, &other.adept);
    self.endurance = optional_max(&self.endurance, &other.endurance);
    self.expert = optional_max(&self.expert, &other.expert);
    self.innate = optional_max(&self.innate, &other.innate);
    self.resonant = optional_max(&self.resonant, &other.resonant);
    self.magic = optional_max(&self.magic, &other.magic);
  }

  pub fn get(&self, class: &TrainingClass) -> i32 {
    return match class {
      TrainingClass::Adept => self.adept.unwrap_or(0),
      TrainingClass::Endurance => self.endurance.unwrap_or(0),
      TrainingClass::Expert => self.expert.unwrap_or(0),
      TrainingClass::Innate => self.innate.unwrap_or(0),
      TrainingClass::Resonance => self.resonant.unwrap_or(0),
      TrainingClass::Magic => self.magic.unwrap_or(0),
    };
  }

  pub fn set(&mut self, class: &TrainingClass, value: i32) {
    match class {
      TrainingClass::Expert => self.expert = Some(value),
      TrainingClass::Adept => self.adept = Some(value),
      TrainingClass::Endurance => self.endurance = Some(value),
      TrainingClass::Innate => self.innate = Some(value),
      TrainingClass::Resonance => self.resonant = Some(value),
      TrainingClass::Magic => self.magic = Some(value),
    }
  }

  pub fn sum(&self) -> i32 {
    return self.adept.unwrap_or(0)
      + self.endurance.unwrap_or(0)
      + self.expert.unwrap_or(0)
      + self.innate.unwrap_or(0)
      + self.resonant.unwrap_or(0)
      + self.magic.unwrap_or(0);
  }

  pub fn summary(&self) -> String {
    let items: Vec<String> = vec![
      option_formater("Adept".into(), &self.adept),
      option_formater("Endurance".into(), &self.endurance),
      option_formater("Expert".into(), &self.expert),
      option_formater("Innate".into(), &self.innate),
      option_formater("Resonnance".into(), &self.resonant),
      option_formater("Magic".into(), &self.magic),
    ]
    .into_iter()
    .flatten()
    .collect();
    return items.join(", ");
  }
}

fn optional_max(lhs: &Option<i32>, rhs: &Option<i32>) -> Option<i32> {
  return match (lhs, rhs) {
    (None, None) => None,
    (Some(value), None) => Some(*value),
    (None, Some(value)) => Some(*value),
    (Some(left_value), Some(right_value)) => Some(*left_value.max(right_value)),
  };
}

fn option_formater(title: String, value: &Option<i32>) -> Option<String> {
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
  let build = build_signal();
  let expand_signal: Signal<Option<TrainingClass>> = use_signal(|| None);
  let max = build.get_level();
  let previous_training = build.get_previous_development();
  let current_training = build.get_current_development();
  let summary = current_training.summary();
  let sum = current_training.sum();
  let total = build.get_training_ranks();
  let counter = Counter {
    title: "Points".into(),
    current: sum,
    max: total,
    ..Default::default()
  };
  let remaining_ranks = total - sum;
  let modifiers: crate::modifiers::prelude::ModifierSet = build.get_training_modifiers();
  build.get_path_constraints();
  return rsx! {
    SectionBar {
      title: "Development",
      bar: rsx! {
        CounterBadge { counter }
        "{summary}"
      },
      explainer: rsx! {
        div { "Each level characters gain development points which can be spent in one of six categories. These development points scale up hit points, attributes, specializations, flows and resource pools. You cannot spend more points in a give development than your current level. You can click on each one to see the table of bonuses they provide." }
      },
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
        DevelopmentTable { training_class, highlight_rank: Some( current ) }
      }
    }
  };
}
