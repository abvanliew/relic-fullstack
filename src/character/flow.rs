use std::{cmp::min, fmt};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::ResourcePool;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Flow {
  Innate,
  Resonance,
  Magic,
}

impl fmt::Display for Flow {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "{}", match self {
      Flow::Innate => "Innate",
      Flow::Resonance => "Resonance",
      Flow::Magic => "Magic",
    } )
  }
}

impl Flow {
  pub fn ordered() -> [Flow; 3] { [ Flow::Innate, Flow::Resonance, Flow::Magic, ] }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FlowStat {
  pub flow: Flow,
  pub base: i32,
  pub pools: Vec<ResourceStat>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ResourceStat {
  pub resource: ResourcePool,
  pub base: i32,
}

#[component]
pub fn FlowResourcesBlock( flows: ReadOnlySignal<Vec<FlowStat>>) -> Element {
  rsx!(
    div {
      class: "grid dim-resources",
      for flow in flows.read().iter() {
        FlowBlock { flow: flow.clone() }
      }
    }
  )
}

#[component]
pub fn FlowBlock( flow: ReadOnlySignal<FlowStat>) -> Element {
  let read_flow = flow.read().clone();
  let name = read_flow.flow;
  let flow_value = read_flow.base;
  let pools = read_flow.pools;
  let span = pools.len() + 2;
  return rsx!(
    div { class: "uv-title-flow right highlight", "{name} {flow_value}" }
    div {
      class: "uv-divider thin",
      style: "grid-row: span {span}"
    }
    div { class: "uv-reserves italics", "Reserves" }
    for pool in pools {
      ResourcePoolEntry { pool, flow_value }
    }
    div {
      class: "min-height"
    }
  )
}

#[component]
pub fn ResourcePoolEntry( pool: ReadOnlySignal<ResourceStat>, flow_value: i32 ) -> Element {
  let read_pool = pool.read().clone();
  let resource = read_pool.resource;
  let pool_flow = min( read_pool.base, flow_value );
  let pool_reserves = read_pool.base - pool_flow;
  return rsx!(
    div { class: "uv-title", "{resource}" }
    div {
      class: "uv-flow content-right row-wrap small-gap",
      for _ in 0..pool_flow { div { class: "box" } }
    }
    div {
      class: "uv-reserves row-wrap small-gap",
      for _ in 0..pool_reserves { div { class: "box" } }
    }
  )
}