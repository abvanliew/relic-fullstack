use dioxus::prelude::*;

#[cfg(feature = "server")]
use bson::oid::ObjectId;
#[cfg(feature = "server")]
use bson::doc;
#[cfg(feature = "server")]
use mongodb::{Client, Collection};
#[cfg(feature = "server")]
use futures::stream::StreamExt;

use crate::path::Path;

#[server(ListKeywords)]
pub async fn list_keywords() -> Result<Vec<Path>, ServerFnError> {
  let client = Client::with_uri_str( "mongodb://localhost:27017" ).await?;
  let keyword_collection: Collection<Path> = client.database( "relic" ).collection( "keywords" );
  let mut results = keyword_collection.find( doc! {}, ).await?;
  let mut keyword_list: Vec<Path> = Vec::new();
  while let Some( result ) = results.next().await {
    let Ok( path ) = result else { tracing::error!( "Unable to load keyword {:?}", result ); continue; };
    keyword_list.push( path );
  }
  Ok( keyword_list )
}

// #[server(SelectKeywords)]
// pub async fn select_keywords( ids: Vec<ObjectId> ) -> Result<Vec<Path>, ServerFnError> {
//   let client = Client::with_uri_str( "mongodb://localhost:27017" ).await?;
//   let keyword_collection: Collection<Path> = client.database( "relic" ).collection( "keywords" );
//   let mut results = keyword_collection.find( doc! {}, ).await?;
//   let mut keyword_list: Vec<Path> = Vec::new();
//   while let Some( result ) = results.next().await {
//     let Ok( path ) = result else { tracing::error!( "Unable to load keyword {:?}", result ); continue; };
//     keyword_list.push( path );
//   }
//   Ok( keyword_list )
// }
