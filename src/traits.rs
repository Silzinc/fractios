use num::{complex::ComplexFloat, Complex};
use polyx::traits::HasNorm;
use std::{
	fmt::Debug,
	ops::{Add, Div, Mul, Neg, Sub},
};

use num_traits::{Float, NumAssign, NumCast, One, Zero};

pub trait SignedRatioFracType: RatioFracType + Neg<Output = Self> {}
pub trait RatioFracFloat: SignedRatioFracType + Float {}
pub trait RatioFracComplexFloat: RatioFracType + ComplexFloat {}

pub trait RatioFracType:
	Clone
	+ Add<Output = Self>
	+ Sub<Output = Self>
	+ Mul<Output = Self>
	+ Div<Output = Self>
	+ NumCast
	+ Debug
	+ Zero
	+ One
	+ PartialEq
	+ NumAssign
	+ HasNorm
{
}

impl<T> RatioFracType for T
	where T: Clone
	        + Add<Output = Self>
	        + Sub<Output = Self>
	        + Mul<Output = Self>
	        + Div<Output = Self>
	        + NumCast
	        + Debug
	        + Zero
	        + One
	        + PartialEq
	        + NumAssign
	        + HasNorm
{
}

impl<T> SignedRatioFracType for T where T: RatioFracType + Neg<Output = Self> {}
impl RatioFracFloat for f32 {}
impl RatioFracFloat for f64 {}
impl RatioFracComplexFloat for Complex<f32> {}
impl RatioFracComplexFloat for Complex<f64> {}
