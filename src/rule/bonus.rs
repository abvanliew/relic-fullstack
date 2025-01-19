use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};
use std::fmt;
use crate::operator::{opt_max, opt_sum};

#[derive( Serialize, Deserialize, Debug, Clone, PartialEq )]
pub struct Bonus {
  pub base: Option<i32>,
  pub bonus: Option<i32>,
}

impl Default for Bonus {
  fn default() -> Self {
    Self { base: Some( 0 ), bonus: None }
  }
}

impl fmt::Display for Bonus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!( f, "{}", self.value() )
  }
}

impl Add for Bonus {
  type Output = Self;

  fn add( self, rhs: Self ) -> Self::Output {
    Self { base: opt_max( self.base, rhs.base ), bonus: opt_sum( self.bonus, rhs.bonus) }
  }
}

impl AddAssign for Bonus {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.clone() + rhs;
  }
}

impl Bonus {
  pub fn value( &self ) -> i32  {
    return match ( self.base, self.bonus ) {
      ( Some( base ), Some( bonus ) ) => base + bonus,
      ( Some( base ), None ) => base,
      ( None, Some( bonus ) ) => bonus,
      _ => 0,
    };
  }
}
