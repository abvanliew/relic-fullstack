use std::collections::HashSet;

use bson::oid::ObjectId;
use dioxus::prelude::*;

use crate::builder::CharacterBuildPanel;
use crate::character::prelude::{
  BodyStats, CharacterSheet, FillableSheet, SheetDetails, TrainingRanks,
};
use crate::rules::prelude::{AttributeRanks, ProgressChart};
use crate::server::prelude::CharacterSheetCache;

#[component]
pub fn CharacterSheetsPage() -> Element {
  CharacterSheetCache::use_context_provider();
  let CharacterSheetCache(ref sheet_cache) = use_context();
  let sheets = sheet_cache.into_vec();
  return rsx! {
    for sheet in sheets {
      SheetDetails { sheet, named_url: true }
    }
  };
}

#[component]
pub fn SingleCharacterSheetPage(id: String) -> Element {
  CharacterSheetCache::use_context_provider();
  let CharacterSheetCache(ref sheet_cache) = use_context();
  let Some(sheet) = sheet_cache.from_id(&id) else {
    return rsx! {
      div { "Character sheet not found" }
    };
  };
  return rsx! {
    SheetDetails { sheet }
  };
}

#[component]
pub fn BlankSheetPage() -> Element {
  let character_sheet = Some(CharacterSheet {
    id: ObjectId::default(),
    name: "".into(),
    level: 1,
    attributes: AttributeRanks {
      ..AttributeRanks::default()
    },
    training: TrainingRanks::default(),
    body: BodyStats {
      hp: 30,
      constitution: 4,
      speed: 6,
    },
    paths: HashSet::new(),
    skills: Vec::new(),
    flows: None,
    armor: None,
    weapons: None,
    resistances: None,
    expertise: None,
  });
  return rsx! {
    FillableSheet { character_sheet }
  };
}

#[component]
pub fn LevelingProgressionPage() -> Element {
  rsx! {
    ProgressChart {}
  }
}

#[component]
pub fn CharacterBuildPage() -> Element {
  rsx! {
    CharacterBuildPanel {}
  }
}
