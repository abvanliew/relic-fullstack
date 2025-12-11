use crate::keyword::prelude::*;
use crate::{path::Path, server::prelude::get_keyword_map, skill::Skill};
use dioxus::prelude::*;
use std::collections::HashMap;
use bson::oid::ObjectId;

use super::{path::get_path_map, skill::get_skill_map};

fn resource_data<T: Clone>(resource: Resource<Result<T, ServerFnError>>) -> Option<T> {
  return match &*resource.read_unchecked() {
    Some(Ok(data)) => Some(data.clone()),
    _ => None,
  };
}

#[derive(Debug, PartialEq, Clone)]
pub struct MapCache<T: Clone + 'static> {
  resource: Resource<Result<HashMap<String, T>, ServerFnError>>
}

impl<T> MapCache<T> where T: Clone + 'static {
  pub fn into_result_vec(&self) -> Option<Vec<T>> {
    let data = resource_data(self.resource);
    match data {
      Some( map ) => Some( 
        map.iter()
        .map( |(_, path)| path.clone() )
        .collect()
      ),
      None => None
    }
  }

  pub fn into_vec(&self) -> Vec<T> {
    self.into_result_vec().unwrap_or_default()
  }

  pub fn get(&self, id: &String) -> Option<T> {
    let data = resource_data(self.resource);
    let Some( map ) = data else { return None; };
    return map.get(id).cloned();
  }

  pub fn from_object_ids(&self, object_ids: &Vec<ObjectId> ) -> Vec<T> {
    object_ids.iter().filter_map(|id| self.get(&id.to_string()) ).collect()
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SkillCache(pub MapCache<Skill>);

impl SkillCache {
  pub fn use_context_provider() -> Self {
    let resource = 
      use_resource(move || get_skill_map() );
    use_context_provider(|| SkillCache( MapCache{ resource } ) )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeywordCache(pub MapCache<Keyword>);

impl KeywordCache {
  pub fn use_context_provider() -> Self {
    let resource = 
      use_resource(move || get_keyword_map() );
    use_context_provider(|| KeywordCache( MapCache{ resource } ) )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PathCache(pub MapCache<Path>);

impl PathCache {
  pub fn use_context_provider() -> Self {
    let resource = 
      use_resource(move || get_path_map() );
    use_context_provider(|| PathCache( MapCache{ resource } ) )
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ServerRequestSignals {
  keywords_response: Resource<Result<HashMap<String, Keyword>, ServerFnError>>,
  skills_response: Resource<Result<HashMap<String, Skill>, ServerFnError>>,
  paths_response: Resource<Result<HashMap<String, Path>, ServerFnError>>,
}

impl ServerRequestSignals {
  pub fn use_context_provider() -> Self {
    let keywords_response = use_resource(move || get_keyword_map());
    let skills_response = use_resource(move || get_skill_map());
    let paths_response = use_resource(move || get_path_map());
    use_context_provider(|| Self {
      keywords_response,
      skills_response,
      paths_response,
    })
  }

  pub fn get_keywords(&self) -> Option<HashMap<String, Keyword>> {
    return resource_data(self.keywords_response);
  }

  pub fn get_skills(&self) -> Option<HashMap<String, Skill>> {
    return resource_data(self.skills_response);
  }

  pub fn get_paths(&self) -> Option<HashMap<String, Path>> {
    return resource_data(self.paths_response);
  }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GameLibrarySignal {
  pub keywords: Signal<Option<HashMap<String, Keyword>>>,
  pub skills: Signal<Option<HashMap<String, Skill>>>,
  pub paths: Signal<Option<HashMap<String, Path>>>,
}

impl GameLibrarySignal {
  pub fn use_context_provider() -> Self {
    let requests = ServerRequestSignals::use_context_provider();
    let keywords = use_signal(move || resource_data(requests.keywords_response));
    let skills = use_signal(move || resource_data(requests.skills_response));
    let paths = use_signal(move || resource_data(requests.paths_response));
    use_context_provider(|| Self {
      keywords,
      skills,
      paths,
    })
  }
}
