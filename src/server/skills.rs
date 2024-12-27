use dioxus::prelude::*;
use crate::relic::prelude::*;
use bson::doc;

#[cfg(feature = "server")]
use bson::oid::ObjectId;
#[cfg(feature = "server")]
use mongodb::{Client, Collection};
#[cfg(feature = "server")]
use futures::stream::StreamExt;

#[server(ListSkills)]
pub async fn list_skills() -> Result<Vec<Skill>, ServerFnError> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let skills_collection: Collection<Skill> = client.database("relic").collection("skills_paths");
  let mut results = skills_collection
  .find( doc! {}, ).await?;
  let mut skill_list: Vec<Skill> = Vec::new();
  while let Some( result ) = results.next().await {
    let Ok( skill ) = result else { continue; };
    skill_list.push( skill );
  }
  Ok( skill_list )
}

#[server(GetSkill)]
pub async fn get_skill( id: String ) -> Result<Skill, ServerFnError> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let skills_collection: Collection<Skill> = client.database("relic").collection("skills_paths");
  let Ok( id_object ) = ObjectId::parse_str( id ) else {
    return Err( ServerFnError::ServerError( "Invalid Id".into() ) );
  };
  let mut results = skills_collection
  .find( doc! { "_id": id_object }, ).await?;
  while let Some( skill ) = results.next().await {
    return Ok( skill? );
  }
  return Err( ServerFnError::ServerError( "Unable to find skill Id".into() ) )
}
