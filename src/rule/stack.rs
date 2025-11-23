use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::rule::internal::*;
use crate::rule::stat_block::StatBlock;
use crate::skill::prelude::*;

pub type RulesStack = Vec<Stack>;
pub type RuleBlocks = Vec<Block>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct Stack {
  pub property: Option<Property>,
  pub outcomes: Option<Vec<Outcome>>,
  pub items: Option<Vec<RuleBlocks>>,
  pub block: Option<RulesSnippet>,
  pub stats: Option<StatBlock>,
}

impl Stack {
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

#[component]
pub fn RulesStackDetail(stacks: RulesStack, display: TermDisplay) -> Element {
  rsx! {for stack in stacks {
    StackDetail { stack, display }
  }}
}

#[component]
pub fn StackDetail(stack: Stack, display: TermDisplay) -> Element {
  if let Some(property) = stack.property {
    let title = property.term.title.unwrap_or("undefined".into());
    let details = rsx! {RuleBlockSet { blocks: property.rules, display }};
    return rsx! {PropertyDetail { title, details }};
  }

  rsx!(
    if let Some( outcomes ) = stack.outcomes {
      OutcomeDetail { outcomes, display }
    }
    if let Some(stats) = stack.stats {
      StatBlockSnippet { stats }
    }
    if stack.block.is_some() || stack.items.is_some() {
      div {
        class: "uv-full",
        BlockDetail { block: Block{ items: stack.items, block: stack.block }, display }
      }
    }
  )
}

// #[component]
// pub fn RuleProperty(property: Property, display: TermDisplay) -> Element {

// }

#[derive(PartialEq, Props, Clone)]
pub struct PropertyDetailProps {
  title: String,
  details: Element,
}

#[component]
pub fn PropertyDetail(props: PropertyDetailProps) -> Element {
  let title = props.title;
  let details = props.details;
  return rsx!(
    div { class: "uv-title highlight", "{title}" }
    div { class: "uv-details", { details } }
  );
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct Block {
  pub items: Option<Vec<RuleBlocks>>,
  pub block: Option<RulesSnippet>,
}

impl Block {
  pub fn get_keyword_ids(&self) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    if let Some(items) = &self.items {
      for item in items {
        for rule in item {
          ids.extend(rule.get_keyword_ids());
        }
      }
    }
    if let Some(rules) = &self.block {
      for rule in rules {
        ids.extend(rule.get_keyword_ids());
      }
    }
    return ids;
  }
}

#[component]
pub fn RuleBlockSet(blocks: RuleBlocks, display: TermDisplay) -> Element {
  return rsx!(for block in blocks {
    BlockDetail { block, display }
  });
}

#[component]
pub fn BlockDetail(block: Block, display: TermDisplay) -> Element {
  return rsx!(
    if let Some( items ) = block.items {
      ListSnippet { items, display }
    }
    if let Some( snippets ) = block.block {
      RulesSpippetDetail { snippets, display }
    }
  );
}

#[component]
pub fn ListSnippet(items: Vec<RuleBlocks>, display: TermDisplay) -> Element {
  rsx!(
    ul {
      for blocks in items {
        li { RuleBlockSet { blocks, display } }
      }
    }
  )
}
