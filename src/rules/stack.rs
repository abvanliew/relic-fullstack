use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::common::*;
use crate::rules::internal::*;
use crate::rules::section::{RuleSections, RulesSectionSet, Section, SectionDetail};
use crate::rules::snippet::rules_block_from_blurb;
use crate::rules::stat_block::StatBlock;
use crate::skill::prelude::*;

pub type RuleStacks = Vec<Stack>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct Stack {
  pub property: Option<Property>,
  pub outcomes: Option<Vec<Outcome>>,
  pub stats: Option<StatBlock>,
  pub hr: Option<bool>,
  pub items: Option<Vec<RuleSections>>,
  pub block: Option<RulesBlock>,
}

impl Stack {
  pub fn from_blurb( blurb: String ) -> Self {
    Self {
      block: rules_block_from_blurb( blurb ),
      ..Default::default()
    }
  }

  pub fn get_keyword_ids(&self) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    if let Some(property) = &self.property {
      ids.extend(property.get_keyword_ids());
    }
    if let Some(outcomes) = &self.outcomes {
      for outcome in outcomes {
        ids.extend(outcome.get_keyword_ids());
      }
    }
    if let Some(items) = &self.items {
      for item in items {
        for block in item {
          ids.extend(block.get_keyword_ids());
        }
      }
    }
    if let Some(block) = &self.block {
      for snippet in block {
        ids.extend(snippet.get_keyword_ids());
      }
    }
    return ids;
  }
}

pub(crate) fn rules_stack_from_blurb(blurb: String) -> Option<RuleStacks> {
  Some(vec![
    Stack::from_blurb(blurb)
  ])
}

#[component]
pub fn RulesStackDetail(stacks: RuleStacks) -> Element {
  rsx! {
    for stack in stacks {
      StackDetail { stack }
    }
  }
}

#[component]
pub fn StackDetail(stack: Stack) -> Element {
  if let Some(property) = stack.property {
    let (title, sections) = property.get_title_and_sections();
    let block = property.block.unwrap_or_default();
    return rsx! {
      PropertyDetail {
        title,
        block,
        RulesSectionSet { sections }
      }
    };
  }
  rsx!(
    if let Some( outcomes ) = stack.outcomes {
      OutcomeDetail { outcomes }
    }
    if let Some(stats) = stack.stats {
      StatBlockSnippet { stats }
    }
    if stack.block.is_some() || stack.items.is_some() {
      div {
        class: "uv-full",
        SectionDetail { section: Section{ items: stack.items, block: stack.block } }
      }
    }
    if stack.hr.unwrap_or(false) {
      HorizontalBar {}
    }
  )
}

#[component]
pub fn PropertyDetail(
  title: String, #[props(default)] block: bool, children: Element, title_override: Option<Element>,
) -> Element {
  return rsx! {
    if block {
      div { class: "uv-full highlight", "{title}" }
      div { class: "uv-full indent", {children} }
    } else {
      div { class: "uv-title highlight", "{title}" }
      div { class: "uv-details", {children} }
    }
  };
}
