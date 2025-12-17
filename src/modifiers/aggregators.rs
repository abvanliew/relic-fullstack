use crate::modifiers::ModifierSet;
use crate::skill::Skill;

pub fn modifiers_from_skills(skills: &Vec<Skill>) -> ModifierSet {
  let mut new_set = ModifierSet::default();
  for skill in skills {
    let Some(ref modifiers) = skill.modifiers else {
      continue;
    };
    new_set.append(modifiers);
  }
  return new_set;
}
