use dioxus::prelude::*;
use bson::doc;

use crate::character::prelude::CharacterSheet;

#[cfg(feature = "server")]
use bson::oid::ObjectId;
#[cfg(feature = "server")]
use mongodb::{Client, Collection};
#[cfg(feature = "server")]
use futures::stream::StreamExt;

#[server(ListCharacterSheets)]
pub async fn list_character_sheets() -> Result<Vec<CharacterSheet>, ServerFnError> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let characters_collection: Collection<CharacterSheet> = client.database("relic").collection("creatures");
  let mut results = characters_collection.find( doc! {}, ).await?;
  let mut character_list: Vec<CharacterSheet> = Vec::new();
  while let Some( result ) = results.next().await {
    let Ok( character ) = result else { tracing::error!( "Unable to load character {:?}", result ); continue; };
    character_list.push( character );
  }
  Ok( character_list )
}
