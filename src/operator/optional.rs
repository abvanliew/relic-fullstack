use std::cmp::max;

pub fn opt_sum( lhs: Option<i32>, rhs: Option<i32> ) -> Option<i32> {
  return match ( lhs, rhs ) {
    ( None, None ) => None,
    ( Some( value ), None ) => Some( value ),
    ( None, Some( value ) ) => Some( value ),
    ( Some( left ), Some( right ) ) => Some( left + right ),
  };
}

pub fn opt_max( lhs: Option<i32>, rhs: Option<i32> ) -> Option<i32> {
  return match ( lhs, rhs ) {
    ( None, None ) => None,
    ( Some( value ), None ) => Some( value ),
    ( None, Some( value ) ) => Some( value ),
    ( Some( left ), Some( right ) ) => Some( max( left, right ) ),
  };
}


