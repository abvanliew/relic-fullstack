use std::cmp::min;

use dioxus::prelude::*;

use crate::progression::{track::GrowthTrack, training::TrainingClass};

#[derive(Debug, Clone, PartialEq)]
pub struct TrainingGrowthSignals {
  pub expert: Signal<u32>,
  pub adept: Signal<u32>,
  pub endurance: Signal<u32>,
  pub innate: Signal<u32>,
  pub resonance: Signal<u32>,
  pub magic: Signal<u32>,
}

impl Default for TrainingGrowthSignals {
  fn default() -> Self {
    Self {
      expert: use_signal(|| 0),
      adept: use_signal(|| 0),
      endurance: use_signal(|| 0),
      innate: use_signal(|| 0),
      resonance: use_signal(|| 0),
      magic: use_signal(|| 0),
    }
  }
}

#[component]
pub fn CharacterGrowth(
  growth_signals: TrainingGrowthSignals,
  growth_ranks_remaining: u32,
  level: u32,
  #[props(default)] has_innate: bool,
  #[props(default)] has_resonance: bool,
  #[props(default)] has_magic: bool,
) -> Element {
  let display_training_signal: Signal<Option<TrainingClass>> = use_signal(|| None);
  rsx! {
    div {
      class: "grid dim-growth",
      div { 
        class: "uv-full",
        "{growth_ranks_remaining} Remaining Ranks to spend"
      }
      GrowthSelector { 
        training: TrainingClass::Adept, 
        enabled: true,
        level, 
        rank: growth_signals.adept, 
        growth_ranks_remaining,
        display_training_signal
      }
      GrowthSelector { 
        training: TrainingClass::Endurance, 
        enabled: true,
        level, 
        rank: growth_signals.endurance, 
        growth_ranks_remaining,
        display_training_signal
      }
      GrowthSelector { 
        training: TrainingClass::Expert, 
        enabled: true,
        level, 
        rank: growth_signals.expert, 
        growth_ranks_remaining,
        display_training_signal
      }
      GrowthSelector { 
        training: TrainingClass::Innate, 
        enabled: has_innate,
        level, 
        rank: growth_signals.innate, 
        growth_ranks_remaining,
        display_training_signal
      }
      GrowthSelector { 
        training: TrainingClass::Resonance,
        enabled: has_resonance,
        level, 
        rank: growth_signals.resonance, 
        growth_ranks_remaining,
        display_training_signal
      }
      GrowthSelector { 
        training: TrainingClass::Magic,
        enabled: has_magic,
        level,
        rank: growth_signals.magic,
        growth_ranks_remaining,
        display_training_signal
      }
    }
  }
}

#[component]
pub fn GrowthSelector(
  training: TrainingClass,
  level: u32,
  rank: Signal<u32>,
  growth_ranks_remaining: u32,
  display_training_signal: Signal<Option<TrainingClass>>,
  enabled: bool,
) -> Element {
  let max_ranks = 
  if enabled { 
    min( level, growth_ranks_remaining + rank() )
  } else {
    0
  };
  let modifiers = GrowthTrack::class_at( &training, rank() );
  let ( display, new_value ) = match display_training_signal() {
    Some( class ) => {
      match class.eq(&training) {
        true => (true, None),
        false => (false, Some( training.clone() )),
      }
    },
    None => (false, Some( training.clone() )),
  };
  let conditional_class = match enabled {
    true => "",
    false => "disabled",
  };
  rsx! {
    div {
      class: "uv-rank left-cap {conditional_class}",
      onclick: move |event| {
        display_training_signal.set( new_value );
        event.stop_propagation();
      },
      input {
        class: "input",
        type: "number",
        value: rank(),
        min: 0,
        max: max_ranks,
        oninput: move |event| {
          let value = match enabled {
            true => event.value().parse::<u32>().unwrap_or(0).min(max_ranks),
            false => 0,
          };
          rank.set(value);
        },
        onclick: move |event| {
          event.stop_propagation();
        }
      }
    }
    div {
      class: "uv-title mid-cap highlight {conditional_class}",
      onclick: move |event| {
        display_training_signal.set( new_value );
        event.stop_propagation();
      },
      "{training}"
    }
    div {
      class: "uv-property right-cap {conditional_class}",
      onclick: move |event| {
        display_training_signal.set( new_value );
        event.stop_propagation();
      },
      "{modifiers}"
    }
    if display {
      for row_rank in 1..=18 {
        GrowthRow { training, row_rank, rank, max_ranks }
      }
    }
  }
}


#[component]
pub fn GrowthRow(
  training: TrainingClass,
  row_rank: u32,
  rank: Signal<u32>,
  max_ranks: u32,
) -> Element {
  let modifiers = GrowthTrack::class_at( &training, row_rank );
  let conditional_class = match row_rank > max_ranks {
    true => "disabled",
    false => "",
  };
  return rsx! {
    div{
      class: "uv-title spacer {conditional_class}",
      "{training} {row_rank}"
    }
    div{
      class: "uv-property spacer {conditional_class}",
      "{modifiers}"
    }
  }
}
