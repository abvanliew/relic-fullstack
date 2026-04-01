use super::internal::*;
use crate::rules::prelude::*;

impl Keyword {
  pub fn sections(&self) -> RuleSections {
    match (&self.rules, &self.blurb) {
      (Some(blocks), _) => rule_sections_from_block(blocks.clone()),
      (_, Some(blurb)) => rule_sections_from_blurb_certain(blurb.clone()),
      _ => Vec::new(),
    }
  }

  pub fn class_title(&self) -> String {
    self.class.to_string()
  }
}
