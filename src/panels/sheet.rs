use dioxus::prelude::*;
use crate::character::prelude::*;
use crate::skill::Skill;
use crate::path::Path;
use crate::server::prelude::*;

#[component]
pub fn CharacterSheetList() -> Element {
  let response_paths: Resource<Result<Vec<Path>, ServerFnError>> = use_resource( move || list_paths() );
  let response_skills: Resource<Result<Vec<Skill>, ServerFnError>> = use_resource( move || list_skills() );
  let response_sheets: Resource<Result<Vec<CharacterSheet>, ServerFnError>> = use_resource( move || list_character_sheets() );
  return match ( &*response_paths.read_unchecked(), &*response_skills.read_unchecked(), &*response_sheets.read_unchecked() ) {
    ( Some( Ok( paths ) ), Some( Ok( skills ) ), Some( Ok( sheets ) ) ) => {
      rsx! {
        div {
          SheetTable { sheets: sheets.to_owned(), paths: paths.to_owned(), skills: skills.to_owned(), }
        }
      }
    },
    ( None, None, None ) => { rsx! { "Loading Character Sheets" } },
    ( path_status, skills_status, sheets_status ) => {
      rsx!(
        div { "Loading Character Sheet Failure" }
        if let Some( Err( err ) ) = path_status {
          div { "An error occurred when loading paths: {err}" }
        }
        if let Some( Err( err ) ) = skills_status {
          div { "An error occurred when loading skills: {err}" }
        }
        if let Some( Err( err ) ) = sheets_status {
          div { "An error occurred when loading character sheets: {err}" }
        }
      )
    },
  }
}
