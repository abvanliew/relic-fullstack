use std::str::FromStr;

use bson::oid::ObjectId;

use crate::rules::prelude::*;
use crate::skill::prelude::*;

pub fn ranked_boon() -> Skill {
  let action = Action {
    class: Activation::Boon,
    keyword_ids: Some( vec! [
      ObjectId::from_str( "68a394e242060f16e214d04a" ).unwrap_or_default()
    ] ),
    rules: Some( vec! [
      Stack::from_blurb( "Gain a +4 to something, then for each additional rank beyond the first gain another +1 to that something".into() )
    ] ),
    ..Default::default()
  };

  return Skill { 
    title: "Something Bonus".into(),
    training_cost: TrainingCost::Full, 
    ranked: Some( true ),
    action, 
    ..Default::default()
  }
}

pub fn awesome_spell() -> Skill {
  let action = Action {
    initial: Some( true ),
    class: Activation::ComplexAction,
    keyword_ids: Some( vec! [
      ObjectId::from_str( "691401e931fbdfcd5247c9ab" ).unwrap_or_default()
    ] ),
    condition: rule_sections_from_blurb( "When you want to do something cool".into() ),
    cost: Some(ResourceCost::from_mana(1)),
    duration: Some(Duration::five_min()),
    target: Some(Target{
      class: TargetClass::Burst,
      range: Some( 16 ),
      selection: Some( Selection::CreatureObject),
      custom_selection: Some( "As many things as you want".into() ),
      ..Default::default()
    }),
    rules: rules_stack_from_blurb( "This spell has an amazing affect!".into() ),
    ..Default::default()
  };

  return Skill {
    title: "Awesome Spell".into(),
    training_cost: TrainingCost::Spell, 
    action, 
    ..Default::default()
  }
}

pub fn secret_handshake() -> Skill {
  let action = Action {
    class: Activation::Action,
    keyword_ids: Some( vec! [
      ObjectId::from_str( "68a394e242060f16e214d04a" ).unwrap_or_default()
    ] ),
    target: Some( Target {
      class: TargetClass::Touch,
      selection: Some( Selection::Ally ),
      ..Default::default()
    } ),
    rules: Some( vec! [
      Stack::from_blurb( "Clap your target's hand three times. Then you can choose to do either of the following actions.".into() )
    ] ),
    ..Default::default()
  };
  let sub_actions = Some(vec! [
    Action {
      class: Activation::Action,
      sub_title: Some( "Fake and Snap".into() ),
      keyword_ids: Some( vec! [
        ObjectId::from_str( "68a394e242060f16e214d04a" ).unwrap_or_default()
      ] ),
      target: Some( Target {
        class: TargetClass::Touch,
        selection: Some( Selection::Ally ),
        ..Default::default()
      } ),
      rules: Some( vec! [
        Stack::from_blurb( "Pretend to start to shake your target's hand but pull away and snap.".into() )
      ] ),
      ..Default::default()
    },
    Action {
      class: Activation::Action,
      sub_title: Some( "The Spartan Grasp".into() ),
      keyword_ids: Some( vec! [
        ObjectId::from_str( "68a394e242060f16e214d04a" ).unwrap_or_default()
      ] ),
      target: Some( Target {
        class: TargetClass::Touch,
        selection: Some( Selection::Ally ),
        ..Default::default()
      } ),
      rules: Some( vec! [
        Stack::from_blurb( "Grasp the forearm of your target, allowing them to grasp yours. Lock eyes and do a single serious handshake.".into() )
      ] ),
      ..Default::default()
    },
  ]);

  return Skill { 
    title: "Secret Handshake".into(),
    training_cost: TrainingCost::Keystone,
    action, 
    sub_actions,
    ..Default::default()
  }
}