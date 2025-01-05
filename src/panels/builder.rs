use dioxus::prelude::*;

use crate::server::prelude::list_path_skills;
use crate::path::Path;
use crate::character::component::*;

#[component]
pub fn CharacterBuilder() -> Element {
  let response: Resource<Result<Vec<Path>, ServerFnError>> = use_resource( move || list_path_skills() );
  match &*response.read_unchecked() {
    Some( Ok( paths ) ) => {
      rsx! {
        CharacterProgression { paths: paths.to_owned() }
      }
    }
    Some(Err(err)) => {
      rsx! { "An error occurred when loading character builder: {err}" }
    }
    None => { rsx! { "Loading character builder" } }
  }
}
