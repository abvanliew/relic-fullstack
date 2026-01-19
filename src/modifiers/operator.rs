use serde::{Deserialize, Serialize};
use std::{
  cmp::max,
  fmt,
  ops::{Add, AddAssign, Mul},
};

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Eq)]
pub struct Bonus<T>
where
  T: Add + Ord + Clone,
{
  #[serde(default)] pub base: InstanceBonus<T>,
  #[serde(default)] pub bonus: StackingBonus<T>,
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Ord + Default> Bonus<T> {
  pub fn from_bonus(value: T) -> Self {
    Self {
      base: InstanceBonus::default(),
      bonus: StackingBonus::from(value),
    }
  }

  pub fn value(&self) -> T {
    return self.base.as_opt().unwrap_or_default() + self.bonus.as_opt().unwrap_or_default();
  }

  pub fn scalar(&self) -> T {
    return self.bonus.as_opt().unwrap_or_default();
  }

  pub fn multiplier(&mut self, multipler: T) {
    self.bonus = match &self.bonus.0 {
      Some( value ) => StackingBonus::from( value.clone() * multipler ),
      None => StackingBonus( None ),
    }
  }
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Ord + Default + fmt::Display> fmt::Display for Bonus<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let value = self.value();
    let scalar = self.scalar();
    match (value != T::default(), scalar != T::default()) {
      (true, true) => write!(f, "{value} (+{scalar} per additional instance)"),
      (true, false) => write!(f, "{value}"),
      (false, true) => write!(f, "+{scalar} per instance"),
      _ => write!(f, "0"),
    }
  }
}

impl<T: Add<Output = T> + Clone + Ord> Add for Bonus<T> {
  type Output = Bonus<T>;
  fn add(self, rhs: Self) -> Self::Output {
    Bonus {
      bonus: self.bonus + rhs.bonus,
      base: self.base + rhs.base,
    }
  }
}

impl<T: Add<Output = T> + Clone + Ord> AddAssign for Bonus<T>
where
  for<'a> &'a mut Bonus<T>: std::ops::Add<Bonus<T>, Output = Bonus<T>>,
{
  fn add_assign(&mut self, rhs: Self) {
    *self = self.clone() + rhs;
  }
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Eq)]
pub struct StackingBonus<T>(pub Option<T>)
where
  T: Add + Clone;

impl<T: Add<Output = T> + Clone> StackingBonus<T> {
  pub fn from(value: T) -> Self {
    StackingBonus(Some(value))
  }

  pub fn as_opt(&self) -> Option<T> {
    self.0.clone()
  }
}

impl<T: Add<Output = T> + Clone> Add for StackingBonus<T> {
  type Output = StackingBonus<T>;
  fn add(self, rhs: Self) -> Self::Output {
    StackingBonus(match (self.0, rhs.0) {
      (None, None) => None,
      (Some(value), None) => Some(value),
      (None, Some(value)) => Some(value),
      (Some(left), Some(right)) => Some(left + right),
    })
  }
}

impl<T: Add<Output = T> + Clone> AddAssign for StackingBonus<T> {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.clone() + rhs;
  }
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Eq)]
pub struct InstanceBonus<T>(pub Option<T>)
where
  T: Ord + Clone;

impl<T: Ord + Clone> InstanceBonus<T> {
  pub fn base(value: T) -> Self {
    InstanceBonus(Some(value))
  }

  pub fn as_opt(&self) -> Option<T> {
    self.0.clone()
  }
}

impl<T: Ord + Clone> Add for InstanceBonus<T> {
  type Output = InstanceBonus<T>;
  fn add(self, rhs: Self) -> Self::Output {
    InstanceBonus(match (self.0, rhs.0) {
      (None, None) => None,
      (Some(value), None) => Some(value),
      (None, Some(value)) => Some(value),
      (Some(left), Some(right)) => Some(max(left, right)),
    })
  }
}

impl<T: Ord + Clone> AddAssign for InstanceBonus<T> {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.clone() + rhs;
  }
}
