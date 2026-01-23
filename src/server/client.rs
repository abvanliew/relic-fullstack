#[cfg(feature = "server")]
use dioxus::prelude::ServerFnError;
#[cfg(feature = "server")]
use dioxus::prelude::*;
#[cfg(feature = "server")]
use futures::StreamExt;
#[cfg(feature = "server")]
use mongodb::bson::{from_document, Bson, Document};
#[cfg(feature = "server")]
use mongodb::{Client, Collection, Cursor};
#[cfg(feature = "server")]
use serde::de::DeserializeOwned;
#[cfg(feature = "server")]
use std::collections::HashMap;
#[cfg(feature = "server")]
use std::marker::{Send, Sync};
#[cfg(feature = "server")]
use tokio::sync::OnceCell;

#[cfg(feature = "server")]
static MONGO_CLIENT: OnceCell<Client> = OnceCell::const_new();

#[cfg(feature = "server")]
pub async fn get_mongo_client() -> &'static Client {
  MONGO_CLIENT
    .get_or_init(|| async {
      let uri = std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

      Client::with_uri_str(&uri)
        .await
        .expect("Failed to initialize MongoDB client")
    })
    .await
}

#[cfg(feature = "server")]
pub async fn get_collection<T>(name: &str) -> Collection<T>
where
  T: DeserializeOwned + Unpin + Send + Sync,
{
  let client = get_mongo_client().await;
  client.database("relic").collection::<T>(name)
}

#[cfg(feature = "server")]
pub async fn docs_to_map<T>(
  mut cursor: Cursor<Document>,
) -> Result<HashMap<String, T>, ServerFnError>
where
  T: DeserializeOwned,
{
  let mut map = HashMap::new();
  while let Some(result) = cursor.next().await {
    let doc = result.map_err(|e| ServerFnError::new(e.to_string()))?;
    let id_bson = doc
      .get("_id")
      .ok_or_else(|| ServerFnError::new("Document missing _id"))?;
    let id_str = match id_bson {
      Bson::ObjectId(oid) => oid.to_hex(),
      other => other.to_string(),
    };
    let parsed: T = from_document(doc).map_err(|e| ServerFnError::new(e.to_string()))?;
    map.insert(id_str, parsed);
  }
  Ok(map)
}
