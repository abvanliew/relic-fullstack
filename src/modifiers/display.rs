use std::fmt::Display;

use super::{ModifierSet, ModifierClass};

impl Display for ModifierSet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let ModifierSet( map ) = self;
    let ordered_bonuses = ordered_modifier_class();
    let display_bonuses = ordered_bonuses.iter()
      .filter_map( |class| {
        if let Some( bonus ) = map.get(class) {
          let value = bonus.value();
          Some( match class {
            ModifierClass::HP => format!( "+{value} HP" ),
            ModifierClass::PathFeature => format!( "Learn {value} feature from your paths" ),
            ModifierClass::PathMinorFeature => format!( "Learn {value} minor feature from your paths" ),
            ModifierClass::CapabilityRank => format!( "+{value} Capability Ranks" ),
            ModifierClass::CapabilityMaxRank => format!( "+{value} Max Capability Ranks" ),
            ModifierClass::DefenseRank => format!( "+{value} Defense Ranks" ),
            ModifierClass::DefenseMaxRank => format!( "+{value} Max Defense Ranks" ),
            ModifierClass::ExpertiseRank => format!( "+{value} Expertise Ranks" ),
            ModifierClass::ExpertiseMaxRank => format!( "+{value} Max Expertise Ranks" ),
            ModifierClass::PathMin => format!( "{value} Path Minimum" ),
            ModifierClass::PathMax => format!( "{value} Path Maximum" ),
            ModifierClass::GrowthRanks => format!( "{value} Growth Ranks" ),
            ModifierClass::InnateFlow => format!( "+{value} Innate Flow" ),
            ModifierClass::InnatePool => format!( "+{value} to an Innate Pool" ),
            ModifierClass::InnatePoolAll => format!( "+{value} to all Innate Pools" ),
            ModifierClass::ResonanceFlow => format!( "+{value} Resonance Flow" ),
            ModifierClass::ResonancePool => format!( "+{value} to a Resonance Pool" ),
            ModifierClass::ResonancePoolAll => format!( "+{value} to all Resource Pools" ),
            ModifierClass::MagicFlow => format!( "+{value} Magic Flow" ),
            ModifierClass::MagicPool => format!( "+{value} to a Mana Pool" ),
            ModifierClass::MagicPoolAll => format!( "+{value} to all Mana Pools" ),
          } )
        } else {
          None
        }
      }
    )
    .collect::<Vec<String>>();
    write!(f, "{}",display_bonuses.join(", ") )
  }
}

fn ordered_modifier_class() -> Vec<ModifierClass> {
  return vec![
    ModifierClass::HP,
    ModifierClass::PathFeature,
    ModifierClass::PathMinorFeature,
    ModifierClass::CapabilityRank,
    ModifierClass::CapabilityMaxRank,
    ModifierClass::DefenseRank,
    ModifierClass::DefenseMaxRank,
    ModifierClass::ExpertiseRank,
    ModifierClass::ExpertiseMaxRank,
    ModifierClass::PathMin,
    ModifierClass::PathMax,
    ModifierClass::GrowthRanks,
    ModifierClass::InnatePool,
    ModifierClass::InnatePoolAll,
    ModifierClass::InnateFlow,
    ModifierClass::ResonancePool,
    ModifierClass::ResonancePoolAll,
    ModifierClass::ResonanceFlow,
    ModifierClass::MagicPool,
    ModifierClass::MagicPoolAll,
    ModifierClass::MagicFlow,
  ];
}