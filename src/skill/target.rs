use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TargetClass {
  Custom,
  Yourself,
  Touch,
  Weapon,
  Range,
  LineOfSight,
  SeeOrHear,
  Triggering,
  EachTriggering,
  Cone,
  Burst,
  RadiusCorner,
  RadiusSpace,
  Line,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Eq)]
pub enum Selection {
  #[default]
  Creature,
  Ally,
  Enemy,
  Space,
}

impl Target {
  pub fn singular(&self) -> String {
    match (&self.selection, &self.custom_selection) {
      (_, Some(text)) => text.clone(),
      (Some(Selection::Ally), _) => "ally".into(),
      (Some(Selection::Enemy), _) => "enemy".into(),
      (Some(Selection::Creature), _) => "creature".into(),
      (Some(Selection::Space), _) => "space".into(),
      _ => "undefined".into(),
    }
  }

  pub fn plural(&self) -> String {
    match (&self.selection, &self.custom_selection) {
      (_, Some(text)) => text.clone(),
      (Some(Selection::Ally), _) => "allies".into(),
      (Some(Selection::Enemy), _) => "enemies".into(),
      (Some(Selection::Creature), _) => "creatures".into(),
      (Some(Selection::Space), _) => "spaces".into(),
      _ => "undefined".into(),
    }
  }

  pub fn article(&self) -> String {
    match (&self.selection, &self.custom_selection) {
      (Some(Selection::Creature) | Some(Selection::Space), _) => "A",
      (Some(Selection::Ally) | Some(Selection::Enemy), _) => "An",
      (_, Some(_)) => "",
      _ => "Some",
    }
    .into()
  }
}

impl fmt::Display for Target {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(custom) = &self.custom {
      return write!(f, "{custom}");
    }
    let target = match (&self.class, &self.range, &self.size, &self.limit) {
      (TargetClass::Yourself, _, _, _) => "Yourself".into(),
      (TargetClass::Triggering, _, _, _) => format!("Triggering {}", self.singular()),
      (TargetClass::EachTriggering, _, _, _) => {
        format!("Each triggering {}", self.singular())
      }
      (TargetClass::Touch, _, _, None) => {
        format!("{} {} within reach", self.article(), self.singular(),)
      }
      (TargetClass::LineOfSight, _, _, _) => format!(
        "{} {} within line of sight",
        self.article(),
        self.singular(),
      ),
      (TargetClass::SeeOrHear, _, _, Some(limit)) => format!(
        "{} {} that can see or hear you within {}",
        self.article(),
        self.singular(),
        limit,
      ),
      (TargetClass::SeeOrHear, _, _, _) => format!(
        "{} {} that can see or hear you",
        self.article(),
        self.singular(),
      ),
      (TargetClass::Touch, _, _, Some(limit)) => format!(
        "Up to {} {} within reach",
        limit,
        if *limit != 0 {
          self.plural()
        } else {
          self.singular()
        },
      ),
      (TargetClass::Weapon, _, _, None) => {
        format!("{} {} within weapon reach", self.article(), self.singular(),)
      }
      (TargetClass::Weapon, _, _, Some(limit)) => format!(
        "Up to {} {} within weapon reach",
        limit,
        if *limit != 0 {
          self.plural()
        } else {
          self.singular()
        }
      ),
      (TargetClass::Range, Some(range), _, None) => format!(
        "{} {} within range {}",
        self.article(),
        self.singular(),
        range,
      ),
      (TargetClass::Range, Some(range), _, Some(limit)) => format!(
        "Up to {} {} within range {}",
        limit,
        if *limit != 0 {
          self.plural()
        } else {
          self.singular()
        },
        range,
      ),
      (TargetClass::Burst, Some(range), _, None) => {
        format!("All {} within range {}", self.plural(), range,)
      }
      (TargetClass::Burst, Some(range), _, Some(limit)) => format!(
        "Up to {limit} {} within range {range}",
        if *limit != 0 {
          self.plural()
        } else {
          self.singular()
        },
      ),
      (TargetClass::Line, _, Some(size), None) => {
        format!("All {} in a Line {size} spaces long", self.plural(),)
      }
      (TargetClass::Cone, _, Some(size), None) => {
        format!("All {} in a Cone size {size}", self.plural(),)
      }
      (TargetClass::RadiusCorner, Some(range), Some(size), _) => format!(
        "All {} within a radius {size} area centered on a corner within {range} spaces",
        self.plural(),
      ),
      (TargetClass::RadiusSpace, Some(range), Some(size), _) => format!(
        "All {} within a radius {size} area centered on a space within {range} spaces",
        self.plural(),
      ),
      _ => format!("undefined"),
    };
    if let Some(suffix) = &self.suffix {
      return write!(f, "{target} {suffix}.",);
    }
    return write!(f, "{target}.",);
  }
}
