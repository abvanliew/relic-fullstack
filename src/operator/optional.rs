use std::cmp::max;

pub fn opt_sum<Integer: std::ops::Add<Output = Integer>>( lhs: Option<Integer>, rhs: Option<Integer> ) -> Option<Integer> {
  return match ( lhs, rhs ) {
    ( None, None ) => None,
    ( Some( value ), None ) => Some( value ),
    ( None, Some( value ) ) => Some( value ),
    ( Some( left ), Some( right ) ) => Some( left + right ),
  };
}

pub fn opt_max<Integer: std::cmp::Ord>( lhs: Option<Integer>, rhs: Option<Integer> ) -> Option<Integer> {
  return match ( lhs, rhs ) {
    ( None, None ) => None,
    ( Some( value ), None ) => Some( value ),
    ( None, Some( value ) ) => Some( value ),
    ( Some( left ), Some( right ) ) => Some( max( left, right ) ),
  };
}


