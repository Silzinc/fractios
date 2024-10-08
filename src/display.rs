use num::complex::Complex;
use num_traits::One;
use polyx::traits::{Primitive, ToLaTeX};

use crate::RatioFrac;

impl<T: Primitive> ToLaTeX for RatioFrac<T> {
	fn to_latex(&self) -> String {
		if self.numerator.is_empty() {
			return "0".to_string();
		}
		if self.denominator.is_one() {
			return self.numerator.to_latex();
		}
		format!(
			"\\frac{{{}}}{{{}}}",
			self.numerator.to_latex(),
			self.denominator.to_latex()
		)
	}
}

impl<T: Primitive> RatioFrac<Complex<T>> {
	pub fn to_latex_complex(&self) -> String {
		if self.numerator.is_empty() {
			return "0".to_string();
		}
		if self.denominator.is_one() {
			return self.numerator.to_latex();
		}
		format!(
			"\\frac{{{}}}{{{}}}",
			self.numerator.to_latex(),
			self.denominator.to_latex()
		)
	}
}
