use std::collections::HashMap;

use crate::character::prelude::*;
use crate::path::Path;
use crate::server::prelude::*;
use crate::skill::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn CharacterSheetList() -> Element {
  let response_paths: Resource<Result<Vec<Path>, ServerFnError>> =
    use_resource(move || list_paths());
  let response_skills: Resource<Result<Vec<Skill>, ServerFnError>> =
    use_resource(move || list_skills());
  let response_sheets: Resource<Result<Vec<CharacterSheet>, ServerFnError>> =
    use_resource(move || list_character_sheets());
  let response_keywords: Resource<Result<HashMap<String, Keyword>, ServerFnError>> =
    use_resource(move || get_keyword_map());
  return match (
    &*response_paths.read_unchecked(),
    &*response_skills.read_unchecked(),
    &*response_sheets.read_unchecked(),
    &*response_keywords.read_unchecked(),
  ) {
    (Some(Ok(paths)), Some(Ok(skills)), Some(Ok(sheets)), Some(Ok(keywords))) => {
      rsx! {
        div {
          class: "column gap-2xlarge",
          SheetTable {
            sheets: sheets.to_owned(),
            paths: paths.to_owned(),
            skills: skills.to_owned(),
            keywords: keywords.to_owned(),
            named_url: true
          }
        }
      }
    }
    (None, None, None, None) => {
      rsx! { "Loading Character Sheets" }
    }
    (path_status, skills_status, sheets_status, keyword_status) => {
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
        if let Some( Err( err ) ) = keyword_status {
          div { "An error occurred when loading keywords: {err}" }
        }
      )
    }
  };
}

#[component]
pub fn SingleChracterSheet(id: String) -> Element {
  let response_paths: Resource<Result<Vec<Path>, ServerFnError>> =
    use_resource(move || list_paths());
  let response_skills: Resource<Result<Vec<Skill>, ServerFnError>> =
    use_resource(move || list_skills());
  let response_sheet: Resource<Result<CharacterSheet, ServerFnError>> =
    use_resource(move || get_chracter_sheet(id.clone()));
  let response_keywords: Resource<Result<HashMap<String, Keyword>, ServerFnError>> =
    use_resource(move || get_keyword_map());
  return match (
    &*response_paths.read_unchecked(),
    &*response_skills.read_unchecked(),
    &*response_sheet.read_unchecked(),
    &*response_keywords.read_unchecked(),
  ) {
    (Some(Ok(paths)), Some(Ok(skills)), Some(Ok(sheet)), Some(Ok(keywords))) => {
      rsx! {
        div {
          class: "column gap-2xlarge",
          SheetDetails {
            sheet: sheet.to_owned(),
            paths: paths.to_owned(),
            skills: skills.to_owned(),
            keywords: keywords.to_owned(),
            named_url: false,
          }
        }
      }
    }
    (None, None, None, None) => {
      rsx! { "Loading Character Sheets" }
    }
    (path_status, skills_status, sheets_status, keyword_status) => {
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
        if let Some( Err( err ) ) = keyword_status {
          div { "An error occurred when loading keywords: {err}" }
        }
      )
    }
  };
}
