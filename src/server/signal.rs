use crate::keyword::prelude::*;
use crate::{path::Path, server::prelude::get_keyword_map, skill::Skill};
use bson::oid::ObjectId;
use dioxus::prelude::*;
use std::collections::HashMap;

use super::{path::get_path_map, skill::get_skill_map};

#[derive(Debug, PartialEq, Clone)]
pub enum ResourceStatus {
  Loading,
  Errored(ServerFnError),
  Ready,
}

fn resource_data<T: Clone>(resource: Resource<Result<T, ServerFnError>>) -> Option<T> {
  return match &*resource.read_unchecked() {
    Some(Ok(data)) => Some(data.clone()),
    _ => None,
  };
}

#[derive(Debug, PartialEq, Clone)]
pub struct MapCache<T: Clone + 'static> {
  resource: Resource<Result<HashMap<String, T>, ServerFnError>>,
}

impl<T> MapCache<T>
where
  T: Clone + 'static,
{
  pub fn status(&self) -> ResourceStatus {
    match (self.resource)() {
      Some(Ok(_)) => ResourceStatus::Ready,
      None => ResourceStatus::Loading,
      Some(Err(err)) => ResourceStatus::Errored(err),
    }
  }

  pub fn status_element(&self,) -> Option<Element> {
    return match self.status() {
      ResourceStatus::Loading => {
        Some( rsx! {
          div { "Loading ..." }
        } )
      },
      ResourceStatus::Errored( error ) => {
        Some( rsx! {
          div { "Error Loading: {error}" }
        } )
      },
      ResourceStatus::Ready => None,
    }
  }

  pub fn into_result_vec(&self) -> Option<Vec<T>> {
    let data = resource_data(self.resource);
    match data {
      Some(map) => Some(map.iter().map(|(_, path)| path.clone()).collect()),
      None => None,
    }
  }

  pub fn into_vec(&self) -> Vec<T> {
    self.into_result_vec().unwrap_or_default()
  }

  pub fn from_id(&self, id: &String) -> Option<T> {
    let data = resource_data(self.resource);
    let Some(map) = data else {
      return None;
    };
    return map.get(id).cloned();
  }

  pub fn from_object_id(&self, object_id: &ObjectId) -> Option<T> {
    return self.from_id(&object_id.to_string());
  }

  pub fn from_object_ids(&self, object_ids: &Vec<ObjectId>) -> Vec<T> {
    object_ids
    .iter()
    .filter_map(|id| self.from_id(&id.to_string()))
    .collect()
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SkillCache(pub MapCache<Skill>);

impl SkillCache {
  pub fn use_context_provider() -> Self {
    let resource = use_resource(move || get_skill_map());
    use_context_provider(|| SkillCache(MapCache { resource }))
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeywordCache(pub MapCache<Keyword>);

impl KeywordCache {
  pub fn use_context_provider() -> Self {
    let resource = use_resource(move || get_keyword_map());
    use_context_provider(|| KeywordCache(MapCache { resource }))
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PathCache(pub MapCache<Path>);

impl PathCache {
  pub fn use_context_provider() -> Self {
    let resource = use_resource(move || get_path_map());
    use_context_provider(|| PathCache(MapCache { resource }))
  }
}