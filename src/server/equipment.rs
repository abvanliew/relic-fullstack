use std::collections::HashMap;

use dioxus::prelude::*;

use crate::equipment::base::Equipment;

#[cfg(feature = "server")]
use super::client::{docs_to_map, get_collection};
#[cfg(feature = "server")]
use mongodb::bson::{doc, Document};


#[server]
pub async fn get_equipment_map() -> Result<HashMap<String, Equipment>, ServerFnError> {
  let collection = get_collection::<Document>("equipment_display");
  let cursor = collection.await.find(doc! {}).await.map_err(|e| {
    tracing::error!("Unable to find collection {}", e);
    ServerFnError::new(e.to_string())
  })?;
  let map = docs_to_map::<Equipment>(cursor).await?;
  Ok(map)
}
