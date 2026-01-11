use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::common::HorizontalBar;
use crate::rules::internal::*;
use crate::rules::snippet::Snippet;
use crate::rules::stat_block::StatBlock;
use crate::skill::prelude::*;

pub type RulesStack = Vec<Stack>;
pub type RulesBlocks = Vec<Block>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct Stack {
  pub property: Option<Property>,
  pub outcomes: Option<Vec<Outcome>>,
  pub items: Option<Vec<RulesBlocks>>,
  pub block: Option<RulesSnippets>,
  pub stats: Option<StatBlock>,
  pub hr: Option<bool>,
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
pub fn RulesStackDetail(
  stacks: RulesStack,
  display: SkillTermDisplay
) -> Element {
  rsx! {
    for stack in stacks {
      StackDetail { stack, display }
    }
  }
}

#[component]
pub fn StackDetail(
  stack: Stack,
  display: SkillTermDisplay
) -> Element {
  if let Some( property ) = stack.property {
    let ( title, blocks ) = property.get_title_and_blocks();
    let block = property.block.unwrap_or_default();
    return rsx! {
      PropertyDetail { 
        title,
        block,
        RulesBlockSet { blocks }
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
        BlockDetail { block: Block{ items: stack.items, block: stack.block } }
      }
    }
    if stack.hr.unwrap_or(false) {
      HorizontalBar {}
    }
  )
}

#[component]
pub fn PropertyDetail(
  title: String,
  #[props(default)] block: bool,
  children: Element,
) -> Element {
  return  rsx! {
    if block {
      div { class: "uv-full highlight", "{title}" }
      div { class: "uv-full indent", {children} }
    } else {
      div { class: "uv-title highlight", "{title}" }
      div { class: "uv-details", {children} }
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct Block {
  pub items: Option<Vec<RulesBlocks>>,
  pub block: Option<RulesSnippets>,
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

pub(crate) fn blurb_to_rules_blocks(blurb: String) -> RulesBlocks {
  snippets_to_rules_blocks(blurb_to_snippets(blurb))
}

pub(crate) fn snippets_to_rules_blocks(snippets: Vec<Snippet>) -> RulesBlocks {
  vec![Block {
    block: Some(snippets),
    ..Default::default()
  }]
}

pub(crate) fn blurb_to_snippets(blurb: String) -> Vec<Snippet> {
  vec![Snippet {
    text: Some(blurb),
    ..Default::default()
  }]
}

#[component]
pub fn RulesBlockSet(blocks: RulesBlocks) -> Element {
  return rsx!(for block in blocks {
    BlockDetail { block }
  });
}

#[component]
pub fn BlockDetail(block: Block) -> Element {
  return rsx!(
    if let Some( items ) = block.items {
      ListSnippet { items }
    }
    if let Some( snippets ) = block.block {
      RulesSpippetDetail { snippets }
    }
  );
}

#[component]
pub fn ListSnippet(items: Vec<RulesBlocks>) -> Element {
  rsx!(
    ul {
      for blocks in items {
        li { RulesBlockSet { blocks } }
      }
    }
  )
}
