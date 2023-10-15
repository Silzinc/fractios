use num::{complex::ComplexFloat, Complex};
use num_traits::Float;
use polyx::traits::PolyxNum;
use std::ops::Neg;

/// A trait for signed types that can be used as the coefficients of a
/// `RatioFrac`.
pub trait SignedRatioFracType: PolyxNum + Neg<Output = Self> {}
/// A trait for floating point types that can be used as the coefficients of a
/// `RatioFrac`.
pub trait RatioFracFloat: SignedRatioFracType + Float {}
/// A trait for complex floating point types that can be used as the
/// coefficients of a `RatioFrac`.
pub trait RatioFracComplexFloat: PolyxNum + ComplexFloat {}

impl<T> SignedRatioFracType for T where T: PolyxNum + Neg<Output = Self> {}
impl RatioFracFloat for f32 {}
impl RatioFracFloat for f64 {}
impl RatioFracComplexFloat for Complex<f32> {}
impl RatioFracComplexFloat for Complex<f64> {}
