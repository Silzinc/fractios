use polyx::*;
mod basic;
mod display;
mod instantiate;
mod ops;
mod ops_macros;
pub mod traits;

#[derive(Clone, Debug)]
pub struct RatioFrac<T>
{
	pub numerator:   Polynomial<T>,
	pub denominator: Polynomial<T>,
}
