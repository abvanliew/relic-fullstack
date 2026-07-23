use bson::oid::ObjectId;

#[derive(Debug, Clone, Default)]
pub struct CharacterBuild {
  levels: Vec<LevelSelections>
}

#[derive(Debug, Clone, Default)]
pub struct LevelSelections {
  path_features: Vec<PathFeatureSelection>,
}

#[derive(Debug, Clone, Default)]
pub enum PathFeatureSelection {
  #[default]
  Unselected,
  Path(Option<ObjectId>),
  Features(Vec<ObjectId>),
}
