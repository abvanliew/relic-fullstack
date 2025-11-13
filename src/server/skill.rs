use std::collections::HashMap;

use bson::doc;
use dioxus::prelude::*;

use crate::skill::Skill;

#[cfg(feature = "server")]
use bson::oid::ObjectId;
#[cfg(feature = "server")]
use futures::stream::StreamExt;
#[cfg(feature = "server")]
use mongodb::{Client, Collection};

#[server(ListSkills)]
pub async fn list_skills() -> Result<Vec<Skill>, ServerFnError> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let skills_collection: Collection<Skill> = client.database("relic").collection("skills_paths");
  let mut results = skills_collection.find(doc! {}).await?;
  let mut skill_list: Vec<Skill> = Vec::new();
  let mut count: u32 = 0;
  while let Some(result) = results.next().await {
    let Ok(skill) = result else {
      tracing::error!("Unable to load skill [{}] {:?}", count, result);
      continue;
    };
    skill_list.push(skill);
    count += 1;
  }
  Ok(skill_list)
}

#[server(GetSkill)]
pub async fn get_skill(id: String) -> Result<Skill, ServerFnError> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let skills_collection: Collection<Skill> = client.database("relic").collection("skills_paths");
  let Ok(id_object) = ObjectId::parse_str(id) else {
    return Err(ServerFnError::ServerError("Invalid Id".into()));
  };
  let mut results = skills_collection.find(doc! { "_id": id_object }).await?;
  while let Some(skill) = results.next().await {
    return Ok(skill?);
  }
  return Err(ServerFnError::ServerError("Unable to find skill Id".into()));
}

#[server(GetSkillMap)]
pub async fn get_skill_map() -> Result<HashMap<String, Skill>, ServerFnError> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let skills_collection: Collection<Skill> = client.database("relic").collection("skills_display");
  let mut results = skills_collection.find(doc! {}).await?;
  let mut skill_map: HashMap<String, Skill> = HashMap::new();
  let mut count: u32 = 0;
  let mut previous: Option<String> = None;
  while let Some(result) = results.next().await {
    let Ok(skill) = result else {
      tracing::error!("Unable to load skill [{}] {:?} previous {:?}", count, result, previous);
      continue;
    };
    previous = Some(skill.id.to_string());
    skill_map.insert(skill.id.to_string(), skill);
    count += 1;
  }
  Ok(skill_map)
}
