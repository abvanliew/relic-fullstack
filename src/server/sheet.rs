use std::collections::HashMap;

use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::client::{docs_to_map, get_collection};
#[cfg(feature = "server")]
use mongodb::bson::{doc, Document};

use crate::character::prelude::CharacterSheet;

#[server]
pub async fn get_character_sheet_map() -> Result<HashMap<String, CharacterSheet>, ServerFnError> {
  let collection = get_collection::<Document>("creatures");
  let cursor = collection.await.find(doc! {}).await.map_err(|e| {
    tracing::error!("Unable to find collection {}", e);
    ServerFnError::new(e.to_string())
  })?;
  let map = docs_to_map::<CharacterSheet>(cursor).await?;
  Ok(map)
}
