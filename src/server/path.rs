use std::collections::HashMap;

use dioxus::prelude::*;

#[cfg(feature = "server")]
use mongodb::bson::{Document, doc};
#[cfg(feature = "server")]
use super::client::{get_collection, docs_to_map};

use crate::path::Path;

#[server]
pub async fn get_path_map() -> Result<HashMap<String, Path>, ServerFnError> {
  let collection = get_collection::<Document>("paths_display");
  let cursor = collection.await.find(doc! {}).await
    .map_err( |e| {
      tracing::error!( "Unable to find collection {}", e );
      ServerFnError::new( e.to_string() )
    } )?;
  let map = docs_to_map::<Path>(cursor).await?;
  Ok(map)
}
