use super::epsilon::{EPSILON, LOW_EPSILON};
use approx::AbsDiffEq;

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(self, other: Rhs) -> bool;
    fn approx_eq_low_precision(self, other: Rhs) -> bool;
    fn approx_eq_epsilon(self, other: Rhs, epsilon: f64) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: Self) -> bool {
        self.approx_eq_epsilon(other, EPSILON)
    }

    fn approx_eq_low_precision(self, other: Self) -> bool {
        self.approx_eq_epsilon(other, LOW_EPSILON)
    }

    fn approx_eq_epsilon(self, other: Self, epsilon: f64) -> bool {
        self.abs_diff_eq(&other, epsilon)
    }
}
