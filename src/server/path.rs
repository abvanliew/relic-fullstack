use std::collections::HashMap;

use bson::doc;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use bson::oid::ObjectId;
#[cfg(feature = "server")]
use futures::stream::StreamExt;
#[cfg(feature = "server")]
use mongodb::{Client, Collection};

use crate::path::Path;

#[server(ListPathSkills)]
pub async fn list_path_skills() -> Result<Vec<Path>, ServerFnError> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let path_collection: Collection<Path> = client.database("relic").collection("path_skills");
  let mut results = path_collection.find(doc! {}).await?;
  let mut path_list: Vec<Path> = Vec::new();
  let mut count: i32 = 0;
  while let Some(result) = results.next().await {
    let Ok(path) = result else {
      tracing::error!("Unable to load path [{}] {:?}", count, result);
      continue;
    };
    path_list.push(path);
    count += 1;
  }
  Ok(path_list)
}

#[server(GetPathMap)]
pub async fn get_path_map() -> Result<HashMap<String, Path>, ServerFnError> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let path_collection: Collection<Path> = client.database("relic").collection("paths_display");
  let mut results = path_collection.find(doc! {}).await?;
  let mut path_map: HashMap<String, Path> = HashMap::new();
  let mut count: i32 = 0;
  while let Some(result) = results.next().await {
    let Ok(path) = result else {
      tracing::error!("Unable to load path [{}] {:?}", count, result);
      continue;
    };
    path_map.insert(path.id.to_string(), path);
    count += 1;
  }
  Ok(path_map)
}

#[server(ListPaths)]
pub async fn list_paths() -> Result<Vec<Path>, ServerFnError> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let path_collection: Collection<Path> = client.database("relic").collection("paths");
  let mut results = path_collection.find(doc! {}).await?;
  let mut path_list: Vec<Path> = Vec::new();
  while let Some(result) = results.next().await {
    let Ok(path) = result else {
      tracing::error!("Unable to load path {:?}", result);
      continue;
    };
    path_list.push(path);
  }
  Ok(path_list)
}
