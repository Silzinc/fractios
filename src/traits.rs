use num::{complex::ComplexFloat, Complex};
use num_traits::Float;
use polyx::traits::PolyxNum;
use std::ops::Neg;

pub trait SignedRatioFracType: PolyxNum + Neg<Output = Self> {}
pub trait RatioFracFloat: SignedRatioFracType + Float {}
pub trait RatioFracComplexFloat: PolyxNum + ComplexFloat {}

impl<T> SignedRatioFracType for T where T: PolyxNum + Neg<Output = Self> {}
impl RatioFracFloat for f32 {}
impl RatioFracFloat for f64 {}
impl RatioFracComplexFloat for Complex<f32> {}
impl RatioFracComplexFloat for Complex<f64> {}
