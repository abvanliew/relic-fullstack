mod keyword;
mod path;
mod sheet;
mod signal;
mod skill;

#[cfg(feature = "server")]
use bson::oid::ObjectId;
#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BasicObject {
  #[serde(rename = "_id")]
  pub id: ObjectId,
}

pub mod prelude {
  pub use super::keyword::get_keyword_map;
  pub use super::path::{list_path_skills, list_paths};
  pub use super::sheet::{get_chracter_sheet, list_character_sheets};
  pub use super::signal::{GameLibrarySignal, ServerRequestSignals};
  pub use super::skill::{get_skill, list_skills};
}
