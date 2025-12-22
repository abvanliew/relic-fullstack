use crate::modifiers::ModifierClass;

impl ModifierClass {
  pub fn is_keystone_path_specific(&self) -> bool {
    return match self {
      ModifierClass::PathFeature | ModifierClass::PathMinorFeature |
      ModifierClass::PathCoreFeature | ModifierClass::PathSpell |
      ModifierClass::PathCantrip => true,
      _ => false,
    }
  }
}