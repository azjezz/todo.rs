#![macro_use]

#[macro_export]
macro_rules! __enclose__ {
  ( ($( $x:ident ),*) $y:expr ) => {
      {
          $(let $x = $x.clone();)*
          $y
      }
  };
}

// see: https://github.com/rust-lang/rfcs/issues/2407
pub(crate) use __enclose__ as enclose;
