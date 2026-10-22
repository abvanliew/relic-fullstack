use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DevelopmentClass {
  Adept,
  Endurance,
  Expert,
  Innate,
  Resonance,
  Magic,
}

impl Display for DevelopmentClass {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self)
  }
}

impl DevelopmentClass {
  pub fn iter<'a>() -> impl Iterator<Item = &'a Self> {
    return [
      DevelopmentClass::Adept,
      DevelopmentClass::Endurance,
      DevelopmentClass::Expert,
      DevelopmentClass::Innate,
      DevelopmentClass::Resonance,
      DevelopmentClass::Magic,
    ].iter();
  }
}
