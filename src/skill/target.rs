use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Target {
  pub class: TargetClass,
  pub custom: Option<String>,
  pub selection: Option<Selection>,
  pub range: Option<i32>,
  pub size: Option<i32>,
  pub limit: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TargetClass {
  Custom,
  Touch,
  Weapon,
  Range,
  EachTriggering,
  Cone,
  Burst,
  Area,
  Line,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum Selection {
  #[default]
  Creature,
  Ally,
  Enemy,
}

impl Selection {
  pub fn singular( &self ) -> String {
    match self {
      Selection::Creature => "creature",
      Selection::Ally => "ally",
      Selection::Enemy => "enemy",
    }.into()
  }

  pub fn plural( &self ) -> String {
    match self {
      Selection::Creature => "creatures",
      Selection::Ally => "allies",
      Selection::Enemy => "enemies",
    }.into()
  }
}

impl fmt::Display for Target {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    match (
      &self.class,
      &self.range,
      &self.size,
      &self.limit,
    ) {
      ( TargetClass::Custom, _, _, _, ) => write!( f,
        "{}",
        self.custom.clone().unwrap_or( "missing target".to_string() )
      ),
      ( TargetClass::EachTriggering, _, _, _, ) => write!( f,
        "Each triggering {}.",
        self.selection.clone().unwrap_or_default().singular()
      ),
      ( TargetClass::Touch, _, _, None, ) => write!( f,
        "One {} within reach.",
        self.selection.clone().unwrap_or_default().singular()
      ),
      ( TargetClass::Weapon, _, _, None, ) => write!( f,
        "One {} within weapon reach.",
        self.selection.clone().unwrap_or_default().singular()
      ),
      ( TargetClass::Weapon, _, _, Some( limit ), ) => write!( f,
        "Up to {} {} within weapon reach.",
        limit,
        if *limit != 0 { self.selection.clone().unwrap_or_default().plural() } else { self.selection.clone().unwrap_or_default().singular() }
      ),
      _ => write!( f, "undefined" ),
    }
  }
}
