use dioxus::prelude::*;

#[component]
pub fn HorizontalBar() -> Element {
  rsx! { div { class: "uv-full horizontal-bar" } }
}
