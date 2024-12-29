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
  pub suffix: Option<String>,
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
    if let Some( custom ) = &self.custom {
      return write!( f, "{custom}" );
    }
    let target = match (
      &self.class,
      &self.range,
      &self.size,
      &self.limit,
    ) {
      ( TargetClass::EachTriggering, _, _, _, ) => format!(
        "Each triggering {}",
        self.selection.clone().unwrap_or_default().singular()
      ),
      ( TargetClass::Touch, _, _, None, ) => format!(
        "One {} within reach",
        self.selection.clone().unwrap_or_default().singular()
      ),
      ( TargetClass::Weapon, _, _, None, ) => format!(
        "One {} within weapon reach",
        self.selection.clone().unwrap_or_default().singular()
      ),
      ( TargetClass::Weapon, _, _, Some( limit ), ) => format!(
        "Up to {} {} within weapon reach",
        limit,
        if *limit != 0 { self.selection.clone().unwrap_or_default().plural() } else { self.selection.clone().unwrap_or_default().singular() }
      ),
      ( TargetClass::Burst, Some( range ), _, None, ) => format!(
        "All {} within range {}",
        self.selection.clone().unwrap_or_default().plural(),
        range,
      ),
      _ => format!( "undefined" ),
    };
    if let Some( suffix ) = &self.suffix {
      return write!( f, "{target} {suffix}.",  )
    }
    return write!( f, "{target}.",  )
  }
}
