use dioxus::prelude::*;
use crate::path::components::PathDescription;
use crate::server::prelude::*;
use crate::path::Path;

#[component]
pub fn PathList() -> Element {
  let response: Resource<Result<Vec<Path>, ServerFnError>> = use_resource( move || list_path_skills() );
  match &*response.read_unchecked() {
    Some( Ok( paths ) ) => {
      rsx! {
        div {
          class: "grid dim-paths",
          for path in paths {
            PathDescription { path: path.to_owned() }
          }
        }
      }
    }
    Some( Err(err) ) => {
      rsx! { "An error occurred when loading paths: {err}" }
    }
    None => { rsx! { "Loading paths" } }
  }
}