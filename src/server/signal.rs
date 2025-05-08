use std::collections::HashMap;
use dioxus::prelude::*;
use crate::{path::Path, server::prelude::list_keywords, skill::{prelude::Keyword, Skill}};

use super::{path::get_path_map, skill::get_skill_map};

#[derive(Debug, Clone)]
pub enum SignalStatus {
  Loading,
}

fn status_from<T: Clone> ( resource: Resource<Result<T,ServerFnError>> ) -> Result<T,SignalStatus> {
  return match &*resource.read_unchecked() {
    Some( Ok( data ) ) => {
      Ok( data.clone() )
    },
    _ => { Err( SignalStatus::Loading ) },
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GameLibrarySignal {
  keywords_response: Resource<Result<HashMap<String,Keyword>, ServerFnError>>,
  // keywords_signal: Signal<Result<HashMap<String,Keyword>,SignalStatus>>,
  skills_response: Resource<Result<HashMap<String,Skill>, ServerFnError>>,
  paths_response: Resource<Result<HashMap<String,Path>, ServerFnError>>,
}

impl GameLibrarySignal {
  pub fn use_context_provider()-> Self {
    let keywords_response = use_resource( move || list_keywords() );
    // let keywords_signal = use_signal( || Signal::new(Err(SignalStatus::Loading)) )();
    let skills_response = use_resource( move || get_skill_map() );
    let paths_response = use_resource( move || get_path_map() );
    use_context_provider( || Self {
      keywords_response,
      skills_response, paths_response,
    } )
  }

  pub fn get_keyword( &self ) -> Result<HashMap<String,Keyword>,SignalStatus> {
    return status_from( self.keywords_response );
  }

  pub fn get_skills( &self ) -> Result<HashMap<String,Skill>,SignalStatus> {
    return status_from( self.skills_response );
  }

  // pub fn get_paths( &self ) -> SignalStatus<HashMap<String,Path>,ServerFnError> {
  //   return status_from( self.paths_response );
  // }
}
