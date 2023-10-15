use crate::RatioFrac;
use polyx::traits::{FloatLike, PolyxNum};

impl<T: PolyxNum> RatioFrac<T>
{
	/// Evaluates the fraction at x.
	#[inline]
	pub fn eval(&self, x: T) -> T { self.numerator.eval(x.clone()) / self.denominator.eval(x.clone()) }
}

impl<T: FloatLike> RatioFrac<T>
{
	/// Reduces the fraction to p/q where p and q are coprime.
	#[inline]
	pub fn reduce(&mut self)
	{
		if self.numerator.is_empty() {
			return self.denominator = polyx::Polynomial::from(vec![T::one()]);
		}
		let (reduced_numerator, reduced_denominator) = polyx::Polynomial::cofactor_float(&mut self.numerator, &mut self.denominator);
		// println!("{:?}\n{:?}\n", reduced_numerator, reduced_denominator);
		self.numerator = reduced_numerator;
		self.denominator = reduced_denominator;
		let lc_inv = self.denominator[self.denominator.degree()].clone().inv();
		for i in 0..=self.denominator.degree() {
			self.denominator[i] = self.denominator[i].clone() * lc_inv.clone();
		}
		for i in 0..=self.numerator.degree() {
			self.numerator[i] = self.numerator[i].clone() * lc_inv.clone();
		}
	}
}
