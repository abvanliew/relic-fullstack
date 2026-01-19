use std::fmt::Display;

use super::{ModifierSet, ModifierClass, Bonus};

impl ModifierClass {
  pub fn display_with_value( &self, bonus: &Bonus<i32> ) -> String {
    let value = bonus.value();
    match self {
      ModifierClass::HP => format!( "+{value} HP" ),
      ModifierClass::Constituion => format!( "+{value} Consitution" ),
      ModifierClass::WalkingSpeed => format!( "+{value} Speed" ),
      ModifierClass::DashSpeed => format!( "+{value} Dash" ),
      ModifierClass::RankMax => format!( "+{value} Maximum Attribute Rank" ),
      ModifierClass::Feature => format!( "Learn {value} features" ),
      ModifierClass::MinorFeature => format!( "Learn {value} minor features" ),
      ModifierClass::AttributeRank => format!( "+{value} Attribute Ranks" ),
      ModifierClass::CapabilityMaxRank => format!( "+{value} Max Capability Ranks" ),
      ModifierClass::DefenseMaxRank => format!( "+{value} Max Defense Ranks" ),
      ModifierClass::ExpertiseRank => format!( "+{value} Expertise Ranks" ),
      ModifierClass::ExpertiseMaxRank => format!( "+{value} Max Expertise Ranks" ),
      ModifierClass::InitiatePathMin => format!( "{value} Path Minimum" ),
      ModifierClass::InitiatePathMax => format!( "{value} Path Maximum" ),
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
      ModifierClass::ManaPoolMinor => format!( "+{value} Minor Mana Pool" ),
      ModifierClass::ManaPoolModerate => format!( "+{value} Moderate Mana Pool" ),
      ModifierClass::ManaPoolMajor => format!( "+{value} Major Mana Pool" ),
    }
  }
}

impl Display for ModifierSet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let ModifierSet( ref map ) = self;
    let mut bonuses: Vec<(&ModifierClass, &Bonus<i32>)> = map.into_iter().collect();
    bonuses.sort_by( |(lhs, _), (rhs, _)| { lhs.cmp( rhs ) } );
    let display_bonuses: Vec<String> = bonuses.iter().map( | (class, bonus) | {
      class.display_with_value( bonus )
    } ).collect();
    write!( f, "{}", display_bonuses.join(", ") )
  }
}
