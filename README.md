## The `fractios` crate

#### *A Rust crate to handle rational fractions*

This library simply allows to create, multiply, add, subtract, divide rational fractions, through a `RatioFrac` struct. It uses the polynomials of the [`polyx`](https://github.com/Silzinc/polyx/tree/master) crate. It also grants functions to parse to $\LaTeX$ strings and a `reduce` method to change the fraction in order to make its numerator and denominator coprime.

`RatioFrac` structs are only intended to be instantiated with coefficients of type `f32, f64, Complex<f32>, Complex<f64>`.
