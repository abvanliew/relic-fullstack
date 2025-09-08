use std::collections::HashMap;
use dioxus::prelude::*;
use crate::{path::Path, server::prelude::get_keyword_map, skill::{prelude::Keyword, Skill}};

use super::{path::get_path_map, skill::get_skill_map};

fn status_from<T: Clone> ( resource: Resource<Result<T,ServerFnError>> ) -> Option<T> {
  return match &*resource.read_unchecked() {
    Some( Ok( data ) ) => {
      Some( data.clone() )
    },
    _ => { None },
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GameLibrarySignal {
  keywords_response: Resource<Result<HashMap<String,Keyword>, ServerFnError>>,
  skills_response: Resource<Result<HashMap<String,Skill>, ServerFnError>>,
  paths_response: Resource<Result<HashMap<String,Path>, ServerFnError>>,
}

impl GameLibrarySignal {
  pub fn use_context_provider()-> Self {
    let keywords_response = use_resource( move || get_keyword_map() );
    let skills_response = use_resource( move || get_skill_map() );
    let paths_response = use_resource( move || get_path_map() );
    use_context_provider( || Self {
      keywords_response, skills_response, paths_response,
    } )
  }

  pub fn get_keyword( &self ) -> Option<HashMap<String,Keyword>> {
    return status_from( self.keywords_response );
  }

  pub fn get_skills( &self ) -> Option<HashMap<String,Skill>> {
    return status_from( self.skills_response );
  }

  pub fn get_paths( &self ) -> Option<HashMap<String,Path>> {
    return status_from( self.paths_response );
  }
}
