mod sized;

use dioxus::prelude::*;

pub use sized::{StaggeredCell, StaggeredGrid};

#[component]
pub fn HorizontalBar() -> Element {
  rsx! { div { class: "uv-full horizontal-bar" } }
}

#[component]
pub fn InputSignal(
  mut rank: Signal<i32>, #[props(default)] min_rank: i32, max_rank: i32,
) -> Element {
  rsx! {
    input {
      class: "input", type: "number",
      value: rank(), min: min_rank, max: max_rank,
      oninput: move |event| {
        let value = event.value().parse::<i32>()
        .unwrap_or_default()
        .min(max_rank).max(min_rank);
        rank.set(value);
      },
      onclick: move |event| {
        event.stop_propagation();
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumericRange {
  pub min: i32, pub value: i32, pub max: i32,
}

impl NumericRange {
  pub fn new(min: i32, value: i32, max: i32) -> Self {
    Self {min, value, max}
  }

  pub fn decomp(&self) -> (i32, i32, i32) {
    return (self.min, self.value, self.max);
  }
}

#[component]
pub fn NumericInput(
  range: NumericRange,
  value_handler: Callback<i32>,
  #[props(default, into)] class: String,
) -> Element {
  let (min, value, max) = range.decomp();
  rsx! {
    input {
      type: "number",
      class, value, min, max,
      oninput: move |event| { 
        let value = event.value().parse::<i32>()
        .unwrap_or_default()
        .min(max).max(min);
        value_handler.call(value); },
      onclick: move |event| { event.stop_propagation(); }
    }
  }
}

