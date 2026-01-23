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
  pub charge_range: Option<i32>,
  pub size: Option<i32>,
  pub limit: Option<i32>,
  pub suffix: Option<String>,
  pub placed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TargetClass {
  Custom,
  Touch,
  Weapon,
  Range,
  LineOfSight,
  SeeOrHear,
  Cone,
  Burst,
  RadiusCorner,
  RadiusSpace,
  Line,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Eq)]
pub enum Selection {
  Ally,
  Enemy,
  #[default]
  Creature,
  Space,
  Object,
  Affect,
  CreatureSpace,
  CreatureObject,
  ObjectSpace,
  CreatureObjectSpace,
}

impl Target {
  pub fn singular(&self) -> String {
    match (&self.selection, &self.custom_selection) {
      (_, Some(text)) => text.clone(),
      (Some(Selection::Ally), _) => "ally".into(),
      (Some(Selection::Enemy), _) => "enemy".into(),
      (Some(Selection::Creature), _) => "creature".into(),
      (Some(Selection::Space), _) => "space".into(),
      (Some(Selection::Object), _) => "object".into(),
      (Some(Selection::Affect), _) => "affect".into(),
      (Some(Selection::CreatureSpace), _) => "creature or space".into(),
      (Some(Selection::CreatureObject), _) => "creature or object".into(),
      (Some(Selection::ObjectSpace), _) => "object or space".into(),
      (Some(Selection::CreatureObjectSpace), _) => "creature, object, or space".into(),
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
      (Some(Selection::Object), _) => "objects".into(),
      (Some(Selection::Affect), _) => "affects".into(),
      (Some(Selection::CreatureSpace), _) => "creatures and/or spaces".into(),
      (Some(Selection::CreatureObject), _) => "creatures and/or objects".into(),
      (Some(Selection::ObjectSpace), _) => "objects and/or spaces".into(),
      (Some(Selection::CreatureObjectSpace), _) => "creatures, objects, and/or spaces".into(),
      _ => "undefined".into(),
    }
  }

  pub fn article(&self) -> String {
    match (&self.selection, &self.custom_selection) {
      (Some(Selection::Creature) | Some(Selection::Space) | Some(Selection::CreatureObject), _) => {
        "A"
      },
      (Some(Selection::Ally) | Some(Selection::Enemy) | Some(Selection::Object), _) => "An",
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
    let target = match (
      &self.class,
      &self.range,
      &self.size,
      &self.limit,
      &self.placed,
    ) {
      (TargetClass::Touch, _, _, None, _) => {
        format!("{} {} within reach", self.article(), self.singular(),)
      },
      (TargetClass::LineOfSight, _, _, _, _) => format!(
        "{} {} within line of sight",
        self.article(),
        self.singular(),
      ),
      (TargetClass::SeeOrHear, Some(range), _, _, _) => format!(
        "{} {} that can see or hear you within range {}",
        self.article(),
        self.singular(),
        range,
      ),
      (TargetClass::SeeOrHear, _, _, _, _) => format!(
        "{} {} that can see or hear you",
        self.article(),
        self.singular(),
      ),
      (TargetClass::Touch, _, _, Some(limit), _) => format!(
        "Up to {} {} within reach",
        limit,
        if *limit != 0 {
          self.plural()
        } else {
          self.singular()
        },
      ),
      (TargetClass::Weapon, _, _, None, _) => {
        format!("{} {} within weapon reach", self.article(), self.singular(),)
      },
      (TargetClass::Weapon, _, _, Some(limit), _) => format!(
        "Up to {} {} within weapon reach",
        limit,
        if *limit != 0 {
          self.plural()
        } else {
          self.singular()
        }
      ),
      (TargetClass::Range, Some(range), _, None, _) => format!(
        "{} {} within range {}",
        self.article(),
        self.singular(),
        range,
      ),
      (TargetClass::Range, Some(range), _, Some(limit), _) => format!(
        "Up to {} {} within range {}",
        limit,
        if *limit != 0 {
          self.plural()
        } else {
          self.singular()
        },
        range,
      ),
      (TargetClass::Burst, Some(range), _, None, _) => {
        format!("All {} within range {}", self.plural(), range,)
      },
      (TargetClass::Burst, Some(range), _, Some(limit), _) => format!(
        "Up to {limit} {} within range {range}",
        if *limit != 0 {
          self.plural()
        } else {
          self.singular()
        },
      ),
      (TargetClass::Line, _, Some(size), None, _) => {
        format!("All {} in a Line {size} spaces long", self.plural(),)
      },
      (TargetClass::Cone, _, Some(size), None, _) => {
        format!("All {} in a Cone size {size}", self.plural(),)
      },
      (TargetClass::RadiusCorner, Some(range), Some(size), _, Some(true)) => {
        format!("Place a radius {size} area centered on the corner of a space within range {range}")
      },
      (TargetClass::RadiusCorner, Some(range), Some(size), _, _) => format!(
        "All {} within a radius {size} area centered on the corner of a space within range {range}",
        self.plural(),
      ),
      (TargetClass::RadiusSpace, Some(range), Some(size), _, Some(true)) => {
        format!("Place a radius {size} area centered on a space within range {range}")
      },
      (TargetClass::RadiusSpace, Some(range), Some(size), _, _) => format!(
        "All {} within a radius {size} area centered on a space within range {range}",
        self.plural(),
      ),
      _ => format!("undefined"),
    };
    if let Some(suffix) = &self.suffix {
      return write!(f, "{target} {suffix}",);
    }
    return write!(f, "{target}",);
  }
}
