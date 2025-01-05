mod attribute;
mod build;
pub mod component;
pub mod progression;
mod resistance_dodge;
mod resource;
mod sheet;

pub use resource::Resource;

pub mod prelude {
  pub use super::attribute::{Capability,Defense};
  pub use super::resource::ResourceCost;
  pub use super::sheet::Character;
}
