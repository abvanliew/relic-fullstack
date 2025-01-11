mod attribute;
mod build;
pub mod component;
pub mod progression;
mod resistance_dodge;
mod resource;
mod sheet;
mod training;

pub use resource::Resource;

pub mod prelude {
  pub use super::attribute::{Capability,Defense,AttributeClass};
  pub use super::resource::{ResourceCost,PoolModifier};
  pub use super::sheet::Character;
}
