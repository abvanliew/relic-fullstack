use dioxus::prelude::*;
use bson::doc;

#[cfg(feature = "server")]
use bson::oid::ObjectId;
#[cfg(feature = "server")]
use mongodb::{Client, Collection};
#[cfg(feature = "server")]
use futures::stream::StreamExt;

use crate::path::Path;

#[server(ListPathSkills)]
pub async fn list_path_skills() -> Result<Vec<Path>, ServerFnError> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let path_collection: Collection<Path> = client.database("relic").collection("path_skills");
  let mut results = path_collection
  .find( doc! {}, ).await?;
  let mut path_list: Vec<Path> = Vec::new();
  while let Some( result ) = results.next().await {
    let Ok( path ) = result else { tracing::error!( "Unable to load path {:?}", result ); continue; };
    path_list.push( path );
  }
  Ok( path_list )
}
