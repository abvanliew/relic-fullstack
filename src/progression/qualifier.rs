use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{character::ResourcePool, rule::prelude::Bonus};

#[derive(Debug, Clone)]
pub struct PathQualifier {
  pub paths: Signal<u8>,
  pub paths_optional: Signal<u8>,
}

impl PathQualifier {
  pub fn new_signal() -> Self {
    let paths: Signal<u8> = use_signal( || 0 );
    let paths_optional: Signal<u8> = use_signal( || 0 );
    return Self { paths, paths_optional };
  }

  pub fn set( &mut self, ( paths, paths_optional ): ( u8, u8 ) ) {
    self.paths.set( paths );
    self.paths_optional.set( paths_optional );
  }
}

#[derive(Debug, Clone, Default)]
pub struct FlowPoolQualifier {
  pub flow: Bonus<u8>,
  pub pools: HashMap<ResourcePool, Bonus<u8>>,
}

// impl FlowPoolQualifier {
//   // pub fn new_signal() -> Self {
//   //   let paths: Signal<u8> = use_signal( || 0 );
//   //   let paths_optional: Signal<u8> = use_signal( || 0 );
//   //   return Self { paths, paths_optional };
//   // }

//   // pub fn set( &mut self, ( paths, paths_optional ): ( u8, u8 ) ) {
//   //   self.paths.set( paths );
//   //   self.paths_optional.set( paths_optional );
//   // }
// }
