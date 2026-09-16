mod input;
mod sized;
mod wrapper;

use dioxus::prelude::*;

pub use input::{NumericInput, NumericRange};
pub use sized::{StaggeredCell, StaggeredGrid};
pub use wrapper::{CollapsibleHeader};

#[component]
pub fn HorizontalBar() -> Element {
  rsx! { div { class: "uv-full horizontal-bar" } }
}
