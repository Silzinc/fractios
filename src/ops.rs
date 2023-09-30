use num_traits::Inv;

use crate::{
	instantiate::check,
	ops_macros::impl_op_all,
	traits::{RatioFracType, SignedRatioFracType},
	RatioFrac,
};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl<T: RatioFracType> Add for &RatioFrac<T>
{
	type Output = RatioFrac<T>;

	#[inline]
	fn add(self, rhs: Self) -> Self::Output
	{
		let mut numerator = self.numerator.clone();
		let mut denominator = self.denominator.clone();
		numerator *= &rhs.denominator;
		denominator *= &rhs.denominator;
		numerator += &rhs.numerator * &self.denominator;
		Self::Output { numerator,
		               denominator }
	}
}

impl_op_all!(Add, AddAssign, add, add_assign);

impl<T: SignedRatioFracType> Neg for &RatioFrac<T>
{
	type Output = RatioFrac<T>;

	#[inline]
	fn neg(self) -> Self::Output
	{
		Self::Output { numerator:   -&self.numerator,
		               denominator: self.denominator.clone(), }
	}
}

impl<T: SignedRatioFracType> Neg for RatioFrac<T>
{
	type Output = RatioFrac<T>;

	#[inline]
	fn neg(self) -> Self::Output
	{
		Self::Output { numerator:   -self.numerator,
		               denominator: self.denominator, }
	}
}

impl<T: RatioFracType> Sub for &RatioFrac<T>
{
	type Output = RatioFrac<T>;

	#[inline]
	fn sub(self, rhs: Self) -> Self::Output
	{
		let mut numerator = self.numerator.clone();
		let mut denominator = self.denominator.clone();
		numerator *= &rhs.denominator;
		denominator *= &rhs.denominator;
		numerator -= &rhs.numerator * &self.denominator;
		Self::Output { numerator,
		               denominator }
	}
}

impl_op_all!(Sub, SubAssign, sub, sub_assign);

impl<T: RatioFracType> Mul for &RatioFrac<T>
{
	type Output = RatioFrac<T>;

	#[inline]
	fn mul(self, rhs: Self) -> Self::Output
	{
		Self::Output { numerator:   &self.numerator * &rhs.numerator,
		               denominator: &self.denominator * &rhs.denominator, }
	}
}

impl_op_all!(Mul, MulAssign, mul, mul_assign);

impl<T: RatioFracType> Div for &RatioFrac<T>
{
	type Output = RatioFrac<T>;

	#[inline]
	fn div(self, rhs: Self) -> Self::Output
	{
		check(&rhs.numerator);
		Self::Output { numerator:   &self.numerator * &rhs.denominator,
		               denominator: &self.denominator * &rhs.numerator, }
	}
}

impl_op_all!(Div, DivAssign, div, div_assign);

impl<T: RatioFracType> Inv for &RatioFrac<T>
{
	type Output = RatioFrac<T>;

	#[inline]
	fn inv(self) -> Self::Output
	{
		Self::Output { numerator:   self.denominator.clone(),
		               denominator: self.numerator.clone(), }
	}
}

impl<T: RatioFracType> Inv for RatioFrac<T>
{
	type Output = RatioFrac<T>;

	#[inline]
	fn inv(self) -> Self::Output
	{
		Self::Output { numerator:   self.denominator,
		               denominator: self.numerator, }
	}
}

impl<T: RatioFracType> RatioFrac<T>
{
	#[inline]
	pub fn inv_inplace(&mut self) { std::mem::swap(&mut self.denominator, &mut self.numerator); }
}
