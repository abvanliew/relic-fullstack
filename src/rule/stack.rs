use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use std::collections::HashSet;

use crate::rule::internal::*;
use crate::skill::prelude::*;

pub type RulesStack = Vec<Stack>;
pub type RuleBlocks = Vec<Block>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Stack {
  pub property: Option<Property>,
  pub outcomes: Option<Vec<Outcome>>,
  pub items: Option<Vec<RuleBlocks>>,
  pub block: Option<RulesSnippet>,
}

impl Stack {
  pub fn get_keyword_ids( &self ) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    if let Some( outcomes ) = &self.outcomes {
      for outcome in outcomes {
        ids.extend( outcome.get_keyword_ids() );
      }
    }
    if let Some( outcomes ) = &self.outcomes {
      for outcome in outcomes {
        ids.extend( outcome.get_keyword_ids() );
      }
    }
    return ids;
  }
}

#[component]
pub fn RulesStackDetail( stacks: RulesStack ) -> Element {
  rsx!(
    for stack in stacks {
      StackDetail { stack }
    }
  )
}

#[component]
pub fn StackDetail( stack: Stack ) -> Element {
  rsx!(
    if let Some( property ) = stack.property {
      PropertyDetail { title: property.title, blocks: property.rules }
    }
    if let Some( outcomes ) = stack.outcomes {
      OutcomeDetail { outcomes }
    }
    if stack.block.is_some() || stack.items.is_some() {
    div {
      class: "uv-full",
      BlockDetail { block: Block{ items: stack.items, block: stack.block } }
    }
    }
  )
}

#[component]
pub fn PropertyDetail( title: String, blocks: RuleBlocks ) -> Element {
  return rsx!(
    div { class: "uv-title highlight", "{title}" }
    div { class: "uv-details", RuleBlockSet { blocks } }
  )
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Block {
  pub items: Option<Vec<RuleBlocks>>,
  pub block: Option<RulesSnippet>,
}

impl Block {
  pub fn get_keyword_ids( &self ) -> HashSet<ObjectId> {
    let mut ids: HashSet<ObjectId> = HashSet::new();
    if let Some( items ) = &self.items {
      for item in items {
        for rule in item {
          ids.extend(rule.get_keyword_ids());
        }
      }
    }
    if let Some( rules ) = &self.block {
      for rule in rules {
        ids.extend(rule.get_keyword_ids());
      }
    }
    return ids;
  }
}

#[component]
pub fn RuleBlockSet( blocks: RuleBlocks ) -> Element {
  return rsx!(
    for block in blocks {
      BlockDetail { block }
    }
  )
}

#[component]
pub fn BlockDetail( block: Block ) -> Element {
  return rsx!(
    if let Some( items ) = block.items {
      ListSnippet { items }
    }
    if let Some( snippets ) = block.block {
      RulesSpippetDetail { snippets }
    }
  )
}

#[component]
pub fn ListSnippet( items: Vec<RuleBlocks> ) -> Element {
  rsx!(
    ul {
      for blocks in items {
        li { RuleBlockSet { blocks } }
      }
    }
  )
}
