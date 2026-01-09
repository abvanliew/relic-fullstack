use super::internal::*;
use crate::rules::prelude::*;

impl Keyword {
  pub fn blocks(&self) -> RulesBlocks {
    match (&self.rules, &self.blurb) {
      (Some(blocks), _) => snippets_to_rules_blocks(blocks.clone()),
      (_, Some(blurb)) => blurb_to_rules_blocks(blurb.clone()),
      _ => Vec::new(),
    }
  }

  pub fn class_title(&self) -> String {
    self.class.to_string()
  }
}
