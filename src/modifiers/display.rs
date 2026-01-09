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
            ModifierClass::RankMax => format!( "Maximum Attribute Rank {value}" ),
            ModifierClass::Feature => format!( "Learn {value} features" ),
            ModifierClass::MinorFeature => format!( "Learn {value} minor features" ),
            ModifierClass::AttributeRank => format!( "+{value} Attribute Ranks" ),
            ModifierClass::CapabilityMaxRank => format!( "+{value} Max Capability Ranks" ),
            ModifierClass::DefenseMaxRank => format!( "+{value} Max Defense Ranks" ),
            ModifierClass::ExpertiseRank => format!( "+{value} Expertise Ranks" ),
            ModifierClass::ExpertiseMaxRank => format!( "+{value} Max Expertise Ranks" ),
            ModifierClass::PathMin => format!( "{value} Path Minimum" ),
            ModifierClass::PathMax => format!( "{value} Path Maximum" ),
            ModifierClass::GrowthRanks => format!( "{value} Growth Ranks" ),
            ModifierClass::InnateFlow => format!( "+{value} Innate Flow" ),
            ModifierClass::InnatePool => format!( "+{value} to an Innate Pool" ),
            ModifierClass::InnatePoolAll => format!( "+{value} to all Innate Pools" ),
            ModifierClass::AnointmentPool => format!( "+{value} to the Anointment Resource Pool" ),
            ModifierClass::AnimalismPool => format!( "+{value} to the Animalism Resource Pool" ),
            ModifierClass::SanguinePool => format!( "+{value} to the Sanguine Resource Pool" ),
            ModifierClass::RagePool => format!( "+{value} to the Rage Resource Pool" ),
            ModifierClass::ResonanceFlow => format!( "+{value} Resonance Flow" ),
            ModifierClass::ResonancePool => format!( "+{value} to a Resonance Pool" ),
            ModifierClass::ResonancePoolAll => format!( "+{value} to all Resource Pools" ),
            ModifierClass::MasteryPool => format!( "+{value} to the Mastery Resource Pool" ),
            ModifierClass::ChannelPool => format!( "+{value} to the Channel Resource Pool" ),
            ModifierClass::KiPool => format!( "+{value} to the Ki Resource Pool" ),
            ModifierClass::VirtuosoPool => format!( "+{value} to the Virtuoso Resource Pool" ),
            ModifierClass::MagicFlow => format!( "+{value} Magic Flow" ),
            ModifierClass::MagicPool => format!( "+{value} to a Mana Pool" ),
            ModifierClass::MagicPoolAll => format!( "+{value} to all Mana Pools" ),
            ModifierClass::MinorManaPool => format!( "+{value} to the Minor Mana Pool" ),
            ModifierClass::ModerateManaPool => format!( "+{value} to the Moderate Mana Pool" ),
            ModifierClass::MajorManaPool => format!( "+{value} to the Major Mana Pool" ),
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
    ModifierClass::RankMax,
    ModifierClass::Feature,
    ModifierClass::MinorFeature,
    ModifierClass::AttributeRank,
    ModifierClass::CapabilityMaxRank,
    ModifierClass::DefenseMaxRank,
    ModifierClass::ExpertiseRank,
    ModifierClass::ExpertiseMaxRank,
    ModifierClass::PathMin,
    ModifierClass::PathMax,
    ModifierClass::GrowthRanks,
    ModifierClass::InnatePool,
    ModifierClass::InnatePoolAll,
    ModifierClass::InnateFlow,
    ModifierClass::AnointmentPool,
    ModifierClass::AnimalismPool,
    ModifierClass::SanguinePool,
    ModifierClass::RagePool,
    ModifierClass::ResonancePool,
    ModifierClass::ResonancePoolAll,
    ModifierClass::ResonanceFlow,
    ModifierClass::MasteryPool,
    ModifierClass::ChannelPool,
    ModifierClass::KiPool,
    ModifierClass::VirtuosoPool,
    ModifierClass::MinorManaPool,
    ModifierClass::ModerateManaPool,
    ModifierClass::MajorManaPool,
    ModifierClass::MagicPool,
    ModifierClass::MagicPoolAll,
    ModifierClass::MagicFlow,
  ];
}
