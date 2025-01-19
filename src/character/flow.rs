use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Flow {
  Innate,
  Resonance,
  Magic,
}

impl fmt::Display for Flow {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      Flow::Innate => "Innate",
      Flow::Resonance => "Resonance",
      Flow::Magic => "Magic",
    } )
  }
}

impl Flow {
  pub fn ordered() -> [Flow; 3] { [ Flow::Innate, Flow::Resonance, Flow::Magic, ] }
}
