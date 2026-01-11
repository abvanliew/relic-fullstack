mod sized;

use dioxus::prelude::*;

pub use sized::StaggeredCell;

#[component]
pub fn HorizontalBar() -> Element {
  rsx! { div { class: "uv-full horizontal-bar" } }
}

#[component]
pub fn InputSignal(
  mut rank: Signal<i32>,
  #[props(default)]
  min_rank: i32,
  max_rank: i32,
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
