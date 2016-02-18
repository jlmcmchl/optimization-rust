use types::{DifferentiableObjective, Optimizer, Solution};
use line_search::{LineSearch, ArmijoLineSearch};
use utils::flat;


/// A simple Gradient Descent optimizer.
pub struct GradientDescent<T> {
    line_search: T,
    gradient_tolerance: f64,
    max_iterations: Option<u64>
}

impl GradientDescent<ArmijoLineSearch> {
    /// Creates a new `GradientDescent` optimizer using the following defaults:
    ///
    /// - **`line_search`** = `ArmijoLineSearch(0.5, 1.0, 0.5)`
    /// - **`gradient_tolerance`** = `1e-4`
    /// - **`max_iterations`** = `None`
    pub fn new() -> GradientDescent<ArmijoLineSearch> {
        GradientDescent {
            line_search: ArmijoLineSearch::new(0.5, 1.0, 0.5),
            gradient_tolerance: 1.0e-4,
            max_iterations: None
        }
    }
}

impl<T: LineSearch> GradientDescent<T> {
    /// Specifies the line search method to use.
    pub fn line_search<S: LineSearch>(self, line_search: S) -> GradientDescent<S> {
        GradientDescent {
            line_search: line_search,
            gradient_tolerance: self.gradient_tolerance,
            max_iterations: self.max_iterations
        }
    }

    /// Adjusts the gradient tolerance which is used as abort criterion to decide
    /// whether we reached a plateau.
    pub fn gradient_tolerance(mut self, gradient_tolerance: f64) -> Self {
        assert!(gradient_tolerance > 0.0);

        self.gradient_tolerance = gradient_tolerance;
        self
    }

    /// Adjusts the number of maximally run iterations. A value of `None` instructs the
    /// optimizer to ignore the nubmer of iterations.
    pub fn max_iterations(mut self, max_iterations: Option<u64>) -> Self {
        assert!(max_iterations.map_or(true, |max_iterations| max_iterations > 0));

        self.max_iterations = max_iterations;
        self
    }
}

impl<T: DifferentiableObjective, S: LineSearch> Optimizer<T> for GradientDescent<S> {
    fn optimize(&self, objective: &T, initial_xs: Vec<f64>) -> Solution {
        info!("Using GradientDescent for optimization: gradient_tolerance = {:?}, max_iterations = {:?}",
            self.gradient_tolerance, self.max_iterations);

        let mut xs = initial_xs;
        let mut y = objective.f(&xs);

        debug!("Starting with y₀ = {:e} for x₀ = {:?}", y, xs);

        let mut iteration = 0;

        loop {
            let gradient = objective.df(&xs);

            if flat(&gradient, self.gradient_tolerance) {
                info!("Gradient to small, stopping optimization");

                return Solution {
                    x: xs,
                    y: y
                }
            }

            let direction: Vec<_> = gradient.into_iter().map(|g| -g).collect();

            let (iter_xs, iter_y) = self.line_search.search(objective, &xs, &direction);

            xs = iter_xs;
            y = iter_y;

            iteration += 1;

            debug!("Iteration {:4}: y = {:e}", iteration, y);

            let reached_max_iterations = self.max_iterations.map_or(false,
                    |max_iterations| iteration == max_iterations);

            if reached_max_iterations {
                info!("Reached maximal number of iterations, stopping optimization");

                return Solution {
                    x: xs,
                    y: y
                }
            }
        }
    }
}