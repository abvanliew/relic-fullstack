pub mod components;
mod dice;
mod roll;
mod rulebook;
mod section;
mod snippet;
mod stack;
mod stat_block;
mod tier;

pub(crate) mod prelude {
  pub(crate) use super::dice::{DiceGroup, DiceGroupEntry};
  pub(crate) use super::rulebook::MainRulesThread;
  pub(crate) use super::section::{RulesSectionSet, RuleSections, rule_sections_from_blurb_certain, rule_sections_from_blurb, rule_sections_from_block};
  pub(crate) use super::snippet::{RulesBlock};
  pub(crate) use super::stack::{Stack, PropertyDetail, RuleStacks, RulesStackDetail,rules_stack_from_blurb};
  pub(crate) use super::tier::Tier;
}

mod internal {
  pub(super) use super::roll::{Outcome, OutcomeDetail, Roll, RollSnippet};
  pub(super) use super::snippet::{RulesBlock, RulesSpippetDetail};
  pub(super) use super::stat_block::StatBlockSnippet;
}
