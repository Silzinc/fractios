use crate::{traits::RatioFracType, RatioFrac};
use num_traits::{One, Zero};
use polyx::Polynomial;
use std::{convert::From, default::Default};

#[inline]
pub(crate) fn check<T: RatioFracType>(denominator: &Polynomial<T>)
{
	if denominator.is_zero() {
		panic!("Cannot use zero as a denominator for RatioFrac")
	}
}

#[macro_export]
macro_rules! ratiofrac {
  ($($x:expr),* ; $($y:expr),*) => ({
    let denominator = Polynomial::from(vec![$($y),*]);
    if denominator.is_empty() {
      panic!("Cannot use zero as a denominator for RatioFrac")
    };
    RatioFrac::from((Polynomial::from(vec![$($x),*]), denominator))
  })
}

impl<T: RatioFracType> From<(Polynomial<T>, Polynomial<T>)> for RatioFrac<T>
{
	#[inline]
	fn from((numerator, denominator): (Polynomial<T>, Polynomial<T>)) -> Self
	{
		check(&denominator);
		RatioFrac { numerator,
		            denominator }
	}
}

impl<T: RatioFracType> From<Polynomial<T>> for RatioFrac<T>
{
	#[inline]
	fn from(numerator: Polynomial<T>) -> Self
	{
		RatioFrac { numerator,
		            denominator: Polynomial::one() }
	}
}

impl<T: RatioFracType> Zero for RatioFrac<T>
{
	#[inline]
	fn zero() -> Self
	{
		RatioFrac { numerator:   Polynomial::zero(),
		            denominator: Polynomial::one(), }
	}

	#[inline]
	fn is_zero(&self) -> bool { self.numerator.is_zero() }
}

impl<T: RatioFracType> One for RatioFrac<T>
{
	#[inline]
	fn one() -> Self
	{
		RatioFrac { numerator:   Polynomial::one(),
		            denominator: Polynomial::one(), }
	}
}

impl<T: RatioFracType> Default for RatioFrac<T>
{
	#[inline]
	fn default() -> Self
	{
		RatioFrac { numerator:   Polynomial::zero(),
		            denominator: Polynomial::one(), }
	}
}

impl<T: RatioFracType> RatioFrac<T>
{
	#[inline]
	pub fn new() -> Self
	{
		RatioFrac { numerator:   Polynomial::zero(),
		            denominator: Polynomial::one(), }
	}

	#[inline]
	pub fn is_empty(&self) -> bool { self.numerator.is_empty() }
}

impl<T: RatioFracType> From<Vec<T>> for RatioFrac<T>
{
	#[inline]
	fn from(values: Vec<T>) -> Self
	{
		RatioFrac { numerator:   Polynomial::from(values),
		            denominator: Polynomial::one(), }
	}
}