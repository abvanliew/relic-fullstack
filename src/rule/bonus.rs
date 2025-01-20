use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};
use std::fmt::{Display, Formatter, Result};
use crate::operator::{opt_max, opt_sum};
use std::cmp::Ord;

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct Bonus<Integer>
where Integer: Clone + Ord + Add<Output = Integer> + Display + Default + Copy {
  pub base: Option<Integer>,
  pub bonus: Option<Integer>,
}

impl<Integer> Default for Bonus<Integer>
where Integer: Clone + Ord + Add<Output = Integer> + Display + Default + Copy {
  fn default() -> Self {
    Self { base: Some( Integer::default() ), bonus: None }
  }
}

impl<Integer> Display for Bonus<Integer>
where Integer: Clone + Ord + Add<Output = Integer> + Display + Default + Copy {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!( f, "{}", self.value() )
  }
}

impl<Integer> Add for Bonus<Integer>
where Integer: Clone + Ord + Add<Output = Integer> + Display + Default + Copy {
  type Output = Bonus<Integer>;

  fn add( self, rhs: Self ) -> Self::Output {
    Self { base: opt_max( self.base, rhs.base ), bonus: opt_sum( self.bonus, rhs.bonus) }
  }
}

impl<Integer> AddAssign for Bonus<Integer>
where Integer: Clone + Ord + Add<Output = Integer> + Display + Default + Copy {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.clone() + rhs;
  }
}

impl<Integer> Bonus<Integer>
where Integer: Clone + Ord + Add<Output = Integer> + Display + Default + Copy {
  pub fn value( &self ) -> Integer  {
    return match ( &self.base, &self.bonus ) {
      ( Some( base ), Some( bonus ) ) => *base + *bonus,
      ( Some( base ), None ) => *base,
      ( None, Some( bonus ) ) => *bonus,
      _ => Integer::default(),
    };
  }
}
