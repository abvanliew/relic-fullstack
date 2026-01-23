use std::collections::HashMap;

use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::client::{docs_to_map, get_collection};
#[cfg(feature = "server")]
use mongodb::bson::{doc, Document};

use crate::skill::Skill;

#[server]
pub async fn get_skill_map() -> Result<HashMap<String, Skill>, ServerFnError> {
  let collection = get_collection::<Document>("skills_display");
  let cursor = collection.await.find(doc! {}).await.map_err(|e| {
    tracing::error!("Unable to find collection {}", e);
    ServerFnError::new(e.to_string())
  })?;
  let map = docs_to_map::<Skill>(cursor).await?;
  Ok(map)
}
