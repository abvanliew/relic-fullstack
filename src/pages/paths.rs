use crate::path::components::*;
use crate::server::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn PathsPage() -> Element {
  let PathCache( ref path_cache ) = use_context();
  let path_results = path_cache.into_result_vec();
  match path_results {
    Some(mut paths) => {
      paths.sort();
      return rsx! {
        PathPanelList { paths }
      };
    }
    None => {
      return rsx! {
        div { "Loading Paths ..." }
      }
    }
  }
}

#[component]
pub fn SinglePath(id: String) -> Element {
  let PathCache( ref path_cache ) = use_context();
  match path_cache.from_id(&id) {
    Some(path) => {
      return rsx! {
        div {
          class: "column gap-large",
          PathPanel { path }
        }
      }
    }
    None => {
      return rsx! {
        div { "Path not found" }
      }
    }
  }
}
