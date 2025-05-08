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
  pub use super::path::{list_paths, list_path_skills};
  pub use super::skill::{list_skills, get_skill};
  pub use super::sheet::{list_character_sheets, get_chracter_sheet};
  pub use super::keyword::list_keywords;
  pub use super::signal::{GameLibrarySignal,SignalStatus};
}
