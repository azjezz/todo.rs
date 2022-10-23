#![macro_use]

// see: https://github.com/rust-lang/rfcs/issues/2407

#[macro_export]
macro_rules! enclose {
  ( $( $x:ident ),* , $y:expr ) => {
      {
          $(let $x = $x.clone();)*
          move || $y
      }
  };
}
