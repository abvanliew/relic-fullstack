use dioxus::prelude::*;

use crate::{character::prelude::{BlankSheet, SheetDetails}, server::prelude::CharacterSheetCache};

#[component]
pub fn CharacterSheetsPage() -> Element {
  CharacterSheetCache::use_context_provider();
  let CharacterSheetCache(ref sheet_cache) = use_context();
  let sheets = sheet_cache.into_vec();
  return rsx! {
    for sheet in sheets {
      SheetDetails { sheet, named_url: true }
    }
  }
}

#[component]
pub fn SingleCharacterSheetPage(id: String) -> Element {
  CharacterSheetCache::use_context_provider();
  let CharacterSheetCache(ref sheet_cache) = use_context();
  let Some( sheet ) = sheet_cache.from_id(&id) else {
    return rsx! {
      div { "Character sheet not found" }
    }
  };
  return rsx! {
    SheetDetails { sheet }
  }
}

#[component]
pub fn BlankSheetPage() -> Element {
  return rsx! {
    BlankSheet {}
  }
}
