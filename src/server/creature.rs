use dioxus::prelude::*;
use bson::doc;

#[cfg(feature = "server")]
use bson::oid::ObjectId;
#[cfg(feature = "server")]
use mongodb::{Client, Collection};
#[cfg(feature = "server")]
use futures::stream::StreamExt;

use crate::character::prelude::Character;

#[server(ListCreatures)]
pub async fn list_creatures() -> Result<Vec<Character>, ServerFnError> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let character_collection: Collection<Character> = client.database("relic").collection("creatures");
  let mut results = character_collection
  .find( doc! {}, ).await?;
  let mut character_list: Vec<Character> = Vec::new();
  while let Some( result ) = results.next().await {
    let Ok( character ) = result else { continue; };
    // tracing::debug!( "{character:?}" );
    character_list.push( character );
  }
  Ok( character_list )
}
