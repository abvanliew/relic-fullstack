use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrainingClass {
  Expert,
  Adept,
  Endurance,
  Innate,
  Resonance,
  Magic,
}

impl fmt::Display for TrainingClass {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        TrainingClass::Expert => "Expert",
        TrainingClass::Adept => "Adept",
        TrainingClass::Endurance => "Endurance",
        TrainingClass::Innate => "Innate",
        TrainingClass::Resonance => "Resonance",
        TrainingClass::Magic => "Magic",
      }
    )
  }
}
