use dioxus::prelude::*;
use crate::path::components::PathListLoader;


#[component]
pub fn PathsPage() -> Element {
  return rsx! {
    PathListLoader {}
  }
}
