use dioxus::prelude::*;
use crate::server::prelude::*;
use crate::path::Path;
use crate::skill::prelude::SkillTable;

#[component]
pub fn PathList() -> Element {
  let response: Resource<Result<Vec<Path>, ServerFnError>> = use_resource( move || list_path_skills() );
  match &*response.read_unchecked() {
    Some( Ok( paths ) ) => {
      rsx! {
        div {
          class: "grid dim-skill-table",
          for path in paths {
            div { class: "title spacer dim-full", "{path.title}" }
            if let Some( summary ) = &path.summary {
              div { class: "dim-full", "{summary}" }
            }
            if let Some( skills ) = &path.skills {
              SkillTable { skills: skills.to_owned() }
            }
          }
        }
      }
    }
    Some(Err(err)) => {
      rsx! { "An error occurred when loading paths: {err}" }
    }
    None => { rsx! { "Loading paths" } }
  }
}