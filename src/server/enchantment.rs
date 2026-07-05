use std::collections::HashMap;

use dioxus::prelude::*;

use crate::equipment::enchantment::Enchantment;

#[cfg(feature = "server")]
use super::client::{docs_to_map, get_collection};
#[cfg(feature = "server")]
use mongodb::bson::{doc, Document};


#[server]
pub async fn get_enchantment_map() -> Result<HashMap<String, Enchantment>, ServerFnError> {
  let collection = get_collection::<Document>("enchantments_display");
  let cursor = collection.await.find(doc! {}).await.map_err(|e| {
    tracing::error!("Unable to find collection {}", e);
    ServerFnError::new(e.to_string())
  })?;
  let map = docs_to_map::<Enchantment>(cursor).await?;
  Ok(map)
}
