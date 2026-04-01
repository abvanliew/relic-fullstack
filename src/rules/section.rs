use bson::oid::ObjectId;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::rules::{internal::*, snippet::rules_block_from_blurb};

pub type RuleSections = Vec<Section>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct Section {
  pub items: Option<Vec<RuleSections>>,
  pub block: Option<RulesBlock>,
}

impl Section {
  pub fn from_blurb(blurb:String)->Self {
    Self::from_block(rules_block_from_blurb(blurb))
  }

  pub fn from_block(block:Option<RulesBlock>) -> Self {
    Self {
      block,
      ..Default::default()
    }
  }

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

pub(crate) fn rule_sections_from_blurb_certain(blurb: String) -> RuleSections {
  vec![Section::from_blurb(blurb)]
}

pub(crate) fn rule_sections_from_blurb(blurb: String) -> Option<RuleSections> {
  Some(rule_sections_from_blurb_certain(blurb))
}

pub(crate) fn rule_sections_from_block( block: RulesBlock ) -> RuleSections {
  vec![Section::from_block(Some(block))]
}

#[component]
pub fn RulesSectionSet(sections: RuleSections) -> Element {
  return rsx!(for section in sections {
    SectionDetail { section }
  });
}

#[component]
pub fn SectionDetail(section: Section) -> Element {
  return rsx!(
    if let Some( items ) = section.items {
      ListSnippet { items }
    }
    if let Some( snippets ) = section.block {
      RulesSpippetDetail { snippets }
    }
  );
}

#[component]
pub fn ListSnippet(items: Vec<RuleSections>) -> Element {
  rsx!(
    ul {
      for sections in items {
        li { RulesSectionSet { sections } }
      }
    }
  )
}
