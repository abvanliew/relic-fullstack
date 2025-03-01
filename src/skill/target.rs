use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Target {
  pub class: TargetClass,
  pub custom: Option<String>,
  pub selection: Option<Selection>,
  pub custom_selection: Option<String>,
  pub range: Option<i32>,
  pub size: Option<i32>,
  pub limit: Option<i32>,
  pub suffix: Option<String>,
}

impl Target {
  pub fn singular( &self ) -> String {
    match ( &self.selection, &self.custom_selection ) {
      ( _, Some( text ) ) => text.clone(),
      ( Some( Selection::Creature ), _ ) => "creature".into(),
      ( Some( Selection::Ally ), _ ) => "ally".into(),
      ( Some( Selection::Enemy ), _ ) => "enemy".into(),
      _ => "undefined".into(),
    }
  }

  pub fn plural( &self ) -> String {
    match ( &self.selection, &self.custom_selection ) {
      ( _, Some( text ) ) => text.clone(),
      ( Some( Selection::Creature ), _ ) => "creatures".into(),
      ( Some( Selection::Ally ), _ ) => "allies".into(),
      ( Some( Selection::Enemy ), _ ) => "enemies".into(),
      _ => "undefined".into(),
    }
  }

  pub fn article( &self ) -> String {
    match &self.selection {
      Some( Selection::Creature ) => "A",
      Some( Selection::Ally ) | Some( Selection::Enemy ) => "An",
      _ => "Some"
    }.into()
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TargetClass {
  Custom,
  Yourself,
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
      ( TargetClass::Yourself, _, _, _ ) => "Yourself".into(),
      ( TargetClass::EachTriggering, _, _, _, ) => format!(
        "Each triggering {}",
        self.singular()
      ),
      ( TargetClass::Touch, _, _, None, ) => format!(
        "{} {} within reach",
        self.article(),
        self.singular(),
      ),
      ( TargetClass::Weapon, _, _, None, ) => format!(
        "{} {} within weapon reach",
        self.article(),
        self.singular(),
      ),
      ( TargetClass::Weapon, _, _, Some( limit ), ) => format!(
        "Up to {} {} within weapon reach",
        limit,
        if *limit != 0 { self.plural() } else { self.singular() }
      ),
      ( TargetClass::Range, Some( range ), _, None, ) => format!(
        "{} {} within range {}",
        self.article(),
        self.singular(),
        range,
      ),
      ( TargetClass::Burst, Some( range ), _, None, ) => format!(
        "All {} within range {}",
        self.plural(),
        range,
      ),
      ( TargetClass::Burst, Some( range ), _, Some( limit ), ) => format!(
        "Up to {} {} within range {}",
        limit,
        if *limit != 0 { self.plural() } else { self.singular() },
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
