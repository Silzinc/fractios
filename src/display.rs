use crate::RatioFrac;
use num::complex::Complex;
use num_traits::One;
use polyx::traits::Primitive;

impl<T: Primitive> RatioFrac<T>
{
	pub fn to_latex_string(&self) -> String
	{
		if self.numerator.is_empty() {
			return "0".to_string();
		}
		if self.denominator.is_one() {
			return self.numerator.to_latex_string();
		}
		return format!(
		               "\\frac{{{}}}{{{}}}",
		               self.numerator.to_latex_string(),
		               self.denominator.to_latex_string()
		);
	}
}

impl<T: Primitive> RatioFrac<Complex<T>>
{
	pub fn to_latex_string_complex(&self) -> String
	{
		if self.numerator.is_empty() {
			return "0".to_string();
		}
		if self.denominator.is_one() {
			return self.numerator.to_latex_string_complex();
		}
		return format!(
		               "\\frac{{{}}}{{{}}}",
		               self.numerator.to_latex_string_complex(),
		               self.denominator.to_latex_string_complex()
		);
	}
}
