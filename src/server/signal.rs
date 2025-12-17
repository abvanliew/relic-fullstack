use crate::keyword::prelude::*;
use crate::path::Path;
use crate::server::prelude::get_keyword_map;
use crate::skill::Skill;
use bson::oid::ObjectId;
use dioxus::prelude::*;
use std::collections::HashMap;

use super::path::get_path_map;
use super::skill::get_skill_map;

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

fn elements_by_status(status: &ResourceStatus, title: &str) -> Option<Element> {
  return match status {
    ResourceStatus::Loading => Some(rsx! {
      div { "Loading {title} ..." }
    }),
    ResourceStatus::Errored(error) => Some(rsx! {
      div { "Cannot load {title} recieved error: {error}" }
    }),
    ResourceStatus::Ready => None,
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
      Some(Err(error)) => ResourceStatus::Errored(error),
    }
  }

  pub fn status_element(&self) -> Option<Element> {
    return elements_by_status(&self.status(), "");
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
pub struct KeywordCache(pub MapCache<Keyword>);

impl KeywordCache {
  pub fn use_context_provider() -> Self {
    let resource = use_resource(move || get_keyword_map());
    use_context_provider(|| KeywordCache(MapCache { resource }))
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
pub struct PathCache(pub MapCache<Path>);

impl PathCache {
  pub fn use_context_provider() -> Self {
    let resource = use_resource(move || get_path_map());
    use_context_provider(|| PathCache(MapCache { resource }))
  }

  pub fn get_sorted_paths(&self, include_inherent: bool) -> Vec<Path> {
    let PathCache(cache) = &self;
    let mut paths: Vec<Path> = cache
      .into_vec()
      .into_iter()
      .filter(|path| match path.inherient {
        Some(true) => include_inherent,
        _ => true,
      })
      .collect();
    paths.sort();
    return paths;
  }
}

pub fn status_element_paths_skills_keywords() -> Option<Element> {
  let PathCache(path_cache) = use_context::<PathCache>();
  let SkillCache(skill_cache) = use_context::<SkillCache>();
  let KeywordCache(keyword_cache) = use_context::<KeywordCache>();
  let mut errors: Vec<ServerFnError> = Vec::new();
  let mut loading: bool = false;
  match path_cache.status() {
    ResourceStatus::Errored(error) => {
      errors.push(error);
    }
    ResourceStatus::Loading => {
      loading = true;
    }
    _ => (),
  }
  match skill_cache.status() {
    ResourceStatus::Errored(error) => {
      errors.push(error);
    }
    ResourceStatus::Loading => {
      loading = true;
    }
    _ => (),
  }
  match keyword_cache.status() {
    ResourceStatus::Errored(error) => {
      errors.push(error);
    }
    ResourceStatus::Loading => {
      loading = true;
    }
    _ => (),
  }
  return match (errors.len() > 0, loading) {
    (true, _) => Some(rsx! {
      div { "Error when loading Paths, Skills, and Keywords ..." }
      for error in errors {
        div { "{error}" }
      }
    }),
    (_, true) => Some(rsx! {
      div { "Loading Paths, Skills, and Keywords ..." }
    }),
    _ => None,
  };
}
