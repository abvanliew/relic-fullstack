use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrainingClass {
  Adept,
  Endurance,
  Expert,
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
        TrainingClass::Adept => "Adept",
        TrainingClass::Endurance => "Endurance",
        TrainingClass::Expert => "Expert",
        TrainingClass::Innate => "Innate",
        TrainingClass::Resonance => "Resonance",
        TrainingClass::Magic => "Magic",
      }
    )
  }
}

impl TrainingClass {
  pub fn ordered() -> Vec<TrainingClass> {
    return vec![
      TrainingClass::Adept,
      TrainingClass::Endurance,
      TrainingClass::Expert,
      TrainingClass::Innate,
      TrainingClass::Resonance,
      TrainingClass::Magic,
    ];
  }
}
