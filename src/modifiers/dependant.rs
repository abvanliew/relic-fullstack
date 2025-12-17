use bson::oid::ObjectId;

use super::ModifierClass;

pub enum Dependancy {
  None,
  Skill,
  Path,
}

// pub fn is_mod_dependant( class: &ModifierClass ) -> Dependancy {
//   return match class {
//     ModifierClass::FlowInnate => Dependancy::None,
//     ModifierClass::FlowResonance => Dependancy::None,
//     ModifierClass::FlowMagic => Dependancy::None,
//   }
// }
