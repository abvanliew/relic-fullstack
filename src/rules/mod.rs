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
  pub(crate) use super::rulebook::{ProgressChart, WeaponExplainer};
  pub(crate) use super::section::{
    rule_sections_from_block, rule_sections_from_blurb, rule_sections_from_blurb_certain,
    RuleSections, RulesSectionSet,
  };
  pub(crate) use super::snippet::RulesBlock;
  pub(crate) use super::stack::{
    rules_stack_from_blurb, PropertyDetail, RuleStacks, RulesStackDetail, Stack,
  };
  pub(crate) use super::stat_block::{AttributeRanks, CapabilityBlock, DefenseBlock};
  pub(crate) use super::tier::Tier;
}

mod internal {
  pub(super) use super::roll::{Outcome, OutcomeDetail, Roll, RollSnippet};
  pub(super) use super::snippet::{RulesBlock, RulesSpippetDetail};
  pub(super) use super::stat_block::StatBlockSnippet;
}
