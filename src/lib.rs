//! A library for manipulating rational fractions. It is nothing more than a
//! thin wrapper around the [`polyx`] crate. It is meant to be used in a circuit
//! simulator the author is working on.

use polyx::*;
mod basic;
mod display;
mod instantiate;
mod ops;
mod ops_macros;
pub mod traits;

/// A rational fraction, i.e. a fraction of two polynomials.
#[derive(Clone, Debug)]
pub struct RatioFrac<T>
{
  pub numerator:   Polynomial<T>,
  pub denominator: Polynomial<T>,
}
