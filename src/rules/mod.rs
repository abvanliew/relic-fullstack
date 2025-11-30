mod bonus;
pub mod components;
mod dice;
mod roll;
mod rulebook;
mod snippet;
mod stack;
mod stat_block;
mod tier;

pub(crate) mod prelude {
  pub(crate) use super::bonus::Bonus;
  pub(crate) use super::dice::{DiceGroup, DiceGroupEntry};
  pub(crate) use super::rulebook::MainRulesThread;
  pub(crate) use super::snippet::Snippet;
  pub(crate) use super::stack::{
    blurb_to_rules_blocks, snippets_to_rules_blocks, PropertyDetail, RulesBlockSet, RulesBlocks,
    RulesStack, RulesStackDetail,
  };
  pub(crate) use super::tier::Tier;
}

mod internal {
  pub(super) use super::roll::{Outcome, OutcomeDetail, Roll, RollSnippet};
  pub(super) use super::snippet::{RulesSnippets, RulesSpippetDetail};
  pub(super) use super::stat_block::StatBlockSnippet;
}
