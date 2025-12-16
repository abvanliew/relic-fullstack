use crate::path::components::*;
use dioxus::prelude::*;

#[component]
pub fn PathsPage() -> Element {
  return rsx! {
    PathListLoader {}
  };
}

#[component]
pub fn SinglePath( id: String ) -> Element {
  return rsx! {
    PathLoader {id}
  };
}
