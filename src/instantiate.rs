use std::{
  convert::From,
  default::Default,
};

use num_traits::{
  One,
  Zero,
};
use polyx::{
  polynomial,
  traits::PolyxNum,
  Polynomial,
};

use crate::RatioFrac;

#[inline]
pub(crate) fn check<T: PolyxNum>(denominator: &Polynomial<T>)
{
  if denominator.is_zero() {
    panic!("Cannot use zero as a denominator for RatioFrac")
  }
}

// TODO: test and eventually correct this macro
// #[macro_export]
// macro_rules! ratiofrac {
//   ($($x:expr),* ; $($y:expr),*) => ({
//     let denominator = Polynomial::from(vec![$($y),*]);
//     if denominator.is_empty() {
//       panic!("Cannot use zero as a denominator for RatioFrac")
//     };
//     RatioFrac::from((Polynomial::from(vec![$($x),*]), denominator))
//   })
// }

impl<T: PolyxNum> From<(Polynomial<T>, Polynomial<T>)> for RatioFrac<T>
{
  #[inline]
  fn from((numerator, denominator): (Polynomial<T>, Polynomial<T>)) -> Self
  {
    check(&denominator);
    RatioFrac {
      numerator,
      denominator,
    }
  }
}

impl<T: PolyxNum> From<Polynomial<T>> for RatioFrac<T>
{
  #[inline]
  fn from(numerator: Polynomial<T>) -> Self
  {
    RatioFrac {
      numerator,
      denominator: Polynomial::one(),
    }
  }
}

impl<T: PolyxNum> From<T> for RatioFrac<T>
{
  #[inline]
  fn from(numerator: T) -> Self
  {
    RatioFrac {
      numerator:   polynomial![numerator],
      denominator: Polynomial::one(),
    }
  }
}

impl<T: PolyxNum> Zero for RatioFrac<T>
{
  #[inline]
  fn zero() -> Self
  {
    RatioFrac {
      numerator:   Polynomial::zero(),
      denominator: Polynomial::one(),
    }
  }

  #[inline]
  fn is_zero(&self) -> bool
  {
    self.numerator.is_zero()
  }
}

impl<T: PolyxNum> One for RatioFrac<T>
{
  #[inline]
  fn one() -> Self
  {
    RatioFrac {
      numerator:   Polynomial::one(),
      denominator: Polynomial::one(),
    }
  }
}

impl<T: PolyxNum> Default for RatioFrac<T>
{
  #[inline]
  fn default() -> Self
  {
    RatioFrac {
      numerator:   Polynomial::zero(),
      denominator: Polynomial::one(),
    }
  }
}

impl<T: PolyxNum> RatioFrac<T>
{
  #[inline]
  pub fn new(numerator: Polynomial<T>, denominator: Polynomial<T>) -> Self
  {
    check(&denominator);
    RatioFrac {
      numerator,
      denominator,
    }
  }

  #[inline]
  pub fn is_empty(&self) -> bool
  {
    self.numerator.is_empty()
  }
}

impl<T: PolyxNum> From<Vec<T>> for RatioFrac<T>
{
  #[inline]
  fn from(values: Vec<T>) -> Self
  {
    RatioFrac {
      numerator:   Polynomial::from(values),
      denominator: Polynomial::one(),
    }
  }
}
