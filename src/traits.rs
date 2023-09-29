use num::{complex::ComplexFloat, Complex};
use polyx::traits::FloatLike;
use std::ops::Neg;

use num_traits::{Float, NumAssign, NumCast};

pub trait SignedRatioFracType: RatioFracType + Neg<Output = Self> {}
pub trait RatioFracFloat: SignedRatioFracType + Float {}
pub trait RatioFracComplexFloat: RatioFracType + ComplexFloat {}

pub trait RatioFracType: NumCast + NumAssign + FloatLike {}

impl<T> RatioFracType for T where T: NumCast + NumAssign + FloatLike {}

impl<T> SignedRatioFracType for T where T: RatioFracType + Neg<Output = Self> {}
impl RatioFracFloat for f32 {}
impl RatioFracFloat for f64 {}
impl RatioFracComplexFloat for Complex<f32> {}
impl RatioFracComplexFloat for Complex<f64> {}
