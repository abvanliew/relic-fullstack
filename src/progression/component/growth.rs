use std::cmp::min;
use std::i32;

use dioxus::prelude::*;

use crate::common::*;
use crate::progression::component::SelectionState;
use crate::progression::{track::GrowthTrack, training::TrainingClass};

#[derive(Debug, Clone, PartialEq)]
pub struct TrainingGrowthSignals {
  pub expert: Signal<i32>,
  pub adept: Signal<i32>,
  pub endurance: Signal<i32>,
  pub innate: Signal<i32>,
  pub resonance: Signal<i32>,
  pub magic: Signal<i32>,
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
  growth_ranks_remaining: i32,
  level: i32,
  #[props(default)] has_innate: bool,
  #[props(default)] has_resonance: bool,
  #[props(default)] has_magic: bool,
) -> Element {
  let display_training_signal: Signal<Option<TrainingClass>> = use_signal(|| None);
  let neg_ranks = growth_ranks_remaining.checked_neg().unwrap_or( i32::MAX );
  let ranks_state = if growth_ranks_remaining == 0 {
    SelectionState::Finished
  } else if growth_ranks_remaining < 0 {
    SelectionState::Invalid
  } else {
    SelectionState::Unfinished
  };
  rsx! {
    div {
      class: "grid dim-growth underhang",
      div { 
        class: "uv-full",
        match ranks_state {
          SelectionState::Finished | SelectionState::Unfinished => rsx!{ "{growth_ranks_remaining} remaining ranks to spend, each category has a maximum rank of {level}." },
          SelectionState::Invalid =>  rsx!{ "Overspent by {neg_ranks} ranks, each category has a maximum rank of {level}." }
        }
      }
      GrowthSelector { 
        training: TrainingClass::Adept, 
        ranks_state,
        enabled: true,
        level, 
        rank: growth_signals.adept, 
        growth_ranks_remaining,
        display_training_signal
      }
      GrowthSelector { 
        training: TrainingClass::Endurance, 
        ranks_state,
        enabled: true,
        level, 
        rank: growth_signals.endurance, 
        growth_ranks_remaining,
        display_training_signal
      }
      GrowthSelector { 
        training: TrainingClass::Expert, 
        ranks_state,
        enabled: true,
        level, 
        rank: growth_signals.expert, 
        growth_ranks_remaining,
        display_training_signal
      }
      GrowthSelector { 
        training: TrainingClass::Innate, 
        ranks_state,
        enabled: has_innate,
        level, 
        rank: growth_signals.innate, 
        growth_ranks_remaining,
        display_training_signal
      }
      GrowthSelector { 
        training: TrainingClass::Resonance,
        ranks_state,
        enabled: has_resonance,
        level, 
        rank: growth_signals.resonance, 
        growth_ranks_remaining,
        display_training_signal
      }
      GrowthSelector { 
        training: TrainingClass::Magic,
        ranks_state,
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
  ranks_state: SelectionState,
  level: i32,
  rank: Signal<i32>,
  growth_ranks_remaining: i32,
  display_training_signal: Signal<Option<TrainingClass>>,
  enabled: bool,
) -> Element {
  let conditional_class = match (ranks_state, enabled, rank() > 0 ) {
    ( SelectionState::Invalid, _, true ) | (_, false, true) => "errored",
    (_, false, _) | (SelectionState::Finished | SelectionState::Invalid, _, false) => "disabled",
    _ => "",
  };
  let max_rank = if enabled { 
    min( level, growth_ranks_remaining + rank() ).max(0)
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
  rsx! {
    div {
      class: "uv-rank growth-selector left-cap {conditional_class}",
      onclick: move |event| {
        display_training_signal.set( new_value );
        event.stop_propagation();
      },
      InputSignal { rank, max_rank, }
    }
    div {
      class: "uv-title growth-selector mid-cap highlight {conditional_class}",
      onclick: move |event| {
        display_training_signal.set( new_value );
        event.stop_propagation();
      },
      "{training}"
    }
    div {
      class: "uv-property growth-selector right-cap small-text {conditional_class}",
      onclick: move |event| {
        display_training_signal.set( new_value );
        event.stop_propagation();
      },
      "{modifiers}"
    }
    if display {
      for row_rank in 1..=18 {
        GrowthRow { training, row_rank, rank, max_rank }
      }
    }
  }
}


#[component]
pub fn GrowthRow(
  training: TrainingClass,
  row_rank: i32,
  rank: Signal<i32>,
  max_rank: i32,
) -> Element {
  let modifiers = GrowthTrack::class_at( &training, row_rank );
  let conditional_class = match row_rank > max_rank {
    true => "disabled",
    false => "",
  };
  return rsx! {
    div{
      class: "uv-title spacer small-text {conditional_class}",
      "{training} {row_rank}"
    }
    div{
      class: "uv-property spacer small-text {conditional_class}",
      "{modifiers}"
    }
  }
}
