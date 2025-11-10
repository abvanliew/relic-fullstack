use crate::{
  path::Path,
  server::prelude::get_keyword_map,
  skill::{prelude::Keyword, Skill},
};
use dioxus::prelude::*;
use std::{collections::HashMap, future::Future};
use tracing::info;

use super::{path::get_path_map, skill::get_skill_map};

pub static KEYWORDS: GlobalSignal<Option<HashMap<String, Keyword>>> = Signal::global(|| None);

pub async fn global_server_load() {
  info!("Start keyword load");
  *KEYWORDS.write() = parse_server_response(get_keyword_map()).await;
  info!("Done with keyword load");
  // *KEYWORDS.write() = parse_server_response(get_keyword_map()).await;
  // *KEYWORDS.write() = parse_server_response(get_keyword_map()).await;
}

pub async fn parse_server_response<T: Clone>(
  response: impl Future<Output = Result<T, ServerFnError>>,
) -> Option<T> {
  let result = response.await;
  return match &result {
    Ok(data) => Some(data.to_owned()),
    _ => None,
  };
}

fn resource_data<T: Clone>(resource: Resource<Result<T, ServerFnError>>) -> Option<T> {
  return match &*resource.read_unchecked() {
    Some(Ok(data)) => Some(data.clone()),
    _ => None,
  };
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
