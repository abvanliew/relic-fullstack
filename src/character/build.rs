use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

use super::attribute::AttributeSet;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CharacterBuild {
  level: Option<i32>,
  attributes: AttributeSet,
  growths: GrowthSet,
  selections: Vec<Selection>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Selection {
  pub class: Option<SelectionClass>,
  pub id: Option<ObjectId>,
  pub sub_selection: Option<Vec<Selection>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum SelectionClass {
  Path,
  FullFeature,
  HalfFeature,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Growth {
  Expert,
  Adept,
  Endurance,
  Resonance,
  Innate,
  Magic,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GrowthSet {
  expert: Option<i32>,
  adept: Option<i32>,
  endurance: Option<i32>,
  resonance: Option<i32>,
  innate: Option<i32>,
  magic: Option<i32>,
}
