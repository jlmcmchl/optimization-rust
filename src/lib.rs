//! Collection of various optimization algorithms and strategies.
//!
//! # Building Blocks
//!
//! Each central primitive is specified by a trait:
//!
//! - **`Function`** - Specifies a function that can be minimized
//! - **`Function1`** - Extends a `Function` by its first derivative
//! - **`Summation`** - Represents a summation of functions, exploited, e.g., by SGD
//! - **`Summation1`** - Analogous to `Function` and `Function1` but for `Summation`
//! - **`Minimizer`** - A minimization algorithm
//! - **`Evaluation`** - A function evaluation `f(x) = y` that is returned by a `Minimizer`
//! - **`Func`** - A new-type wrapper for the `Function` trait
//! - **`NumericalDifferentiation`** - Provides numerical differentiation for arbitrary `Function`s
//!
//! # Algorithms
//!
//! Currently, the following algorithms are implemented. This list is not final and being
//! expanded over time.
//!
//! - **`GradientDescent`** - Iterative gradient descent minimization, supporting various line
//!   search methods:
//!    - *`FixedStepWidth`* - No line search is performed, but a fixed step width is used
//!    - *`ExactLineSearch`* - Exhaustive line search over a set of step widths
//!    - *`ArmijoLineSearch`* - Backtracking line search using the Armijo rule as stopping
//!      criterion
//! - **`StochasticGradientDescent`** - Iterative stochastic gradient descenent minimazation,
//!   currently using a fixed step width

#[macro_use]
extern crate log;
extern crate rand;

#[macro_use]
pub mod problems;

mod gd;
mod line_search;
mod numeric;
mod sgd;
mod types;
mod utils;

pub use gd::GradientDescent;
pub use line_search::{ArmijoLineSearch, ExactLineSearch, FixedStepWidth, LineSearch};
pub use numeric::NumericalDifferentiation;
pub use sgd::StochasticGradientDescent;
pub use types::{Evaluation, Func, Function, Function1, Minimizer, Summation, Summation1};
