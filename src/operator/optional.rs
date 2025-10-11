use std::{
  cmp::max,
  ops::{Add, AddAssign},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StackingBonus<T>(Option<T>)
where
  T: Add + Clone;

impl<T: Add<Output = T> + Clone> StackingBonus<T> {
  pub fn plus(value: T) -> Self {
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct InstanceBonus<T>(Option<T>)
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

pub fn opt_sum<Integer: std::ops::Add<Output = Integer>>(
  lhs: Option<Integer>,
  rhs: Option<Integer>,
) -> Option<Integer> {
  return match (lhs, rhs) {
    (None, None) => None,
    (Some(value), None) => Some(value),
    (None, Some(value)) => Some(value),
    (Some(left), Some(right)) => Some(left + right),
  };
}

pub fn opt_max<Integer: std::cmp::Ord>(
  lhs: Option<Integer>,
  rhs: Option<Integer>,
) -> Option<Integer> {
  return match (lhs, rhs) {
    (None, None) => None,
    (Some(value), None) => Some(value),
    (None, Some(value)) => Some(value),
    (Some(left), Some(right)) => Some(max(left, right)),
  };
}
