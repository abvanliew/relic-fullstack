use super::internal::*;

pub fn rules_specific(keywords: Vec<Keyword>) -> Vec<Keyword> {
  keywords
    .into_iter()
    .filter(|keyword| match keyword.class {
      KeywordClass::Classifier | KeywordClass::CoreRule => false,
      _ => true,
    })
    .collect()
}
