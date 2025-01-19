mod attribute;
pub mod component;
mod flow;
mod pool;
mod resistance_dodge;

pub use pool::ResourcePool;

pub mod prelude {
  pub use super::attribute::{Capability, Defense, RankClass};
  pub use super::flow::Flow;
  pub use super::pool::{ResourceCost,PoolModifier};
}
