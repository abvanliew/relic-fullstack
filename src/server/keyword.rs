use std::collections::HashMap;

use dioxus::prelude::*;

#[cfg(feature = "server")]
use bson::oid::ObjectId;
#[cfg(feature = "server")]
use bson::doc;
#[cfg(feature = "server")]
use mongodb::{Client, Collection};
#[cfg(feature = "server")]
use futures::stream::StreamExt;

use crate::skill::prelude::Keyword;

#[server(ListKeywords)]
pub async fn get_keyword_map() -> Result<HashMap<String,Keyword>, ServerFnError> {
  let client = Client::with_uri_str( "mongodb://localhost:27017" ).await?;
  let keyword_collection: Collection<Keyword> = client.database( "relic" ).collection( "keywords_display" );
  let mut results = keyword_collection.find( doc! {}, ).await?;
  let mut keyword_map: HashMap<String,Keyword> = HashMap::new();
  while let Some( result ) = results.next().await {
    let Ok( keyword ) = result else { tracing::error!( "Unable to load keyword {:?}", result ); continue; };
    keyword_map.insert( keyword.id.to_string(), keyword );
  }
  Ok( keyword_map )
}
