use std::collections::HashMap;

use dioxus::prelude::*;

#[cfg(feature = "server")]
use mongodb::bson::{Document, doc};
#[cfg(feature = "server")]
use super::client::{get_collection, docs_to_map};

use crate::keyword::prelude::*;

#[server]
pub async fn get_keyword_map() -> Result<HashMap<String, Keyword>, ServerFnError> {
  let collection = get_collection::<Document>("keywords_display");
  let cursor = collection.await.find(doc! {}).await
    .map_err( |e| {
      tracing::error!( "Unable to find collection {}", e );
      ServerFnError::new( e.to_string() )
    } )?;
  let map = docs_to_map::<Keyword>(cursor).await?;
  Ok(map)
}
