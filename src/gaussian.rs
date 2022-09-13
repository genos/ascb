//! See HLearn's original Gaussian distribution work:
//! https://github.com/mikeizbicki/HLearn/blob/bb258e88a0f42be4cead167b4da2694a1a2c4605/src/HLearn/Models/Distributions/Gaussian.hs

use std::f64::consts::PI;
use std::iter::FromIterator;
use std::ops::{Add, AddAssign};

use crate::traits::{Monoid, Semigroup};

/// Parameterized 1D Gaussian distribution
#[derive(Clone, Copy, Debug)]
pub struct Gaussian {
    /// First moment of distribution (mean)
    m1: f64,
    /// Second moment of distribution
    m2: f64,
    /// Count of datapoints (stored as a float for convenience)
    n: f64,
}

/// numpy.isclose
fn _close(x: f64, y: f64) -> bool {
    (x - y).abs() <= 1e-8 + 1e-5 * y.abs()
}

impl PartialEq for Gaussian {
    fn eq(&self, other: &Gaussian) -> bool {
        (self.n == other.n) && _close(self.m1, other.m1) && _close(self.m2, other.m2)
    }
}
impl Eq for Gaussian {}

impl Default for Gaussian {
    fn default() -> Self {
        Gaussian {
            m1: 0.0,
            m2: 0.0,
            n: 0.0,
        }
    }
}

#[allow(dead_code)]
impl Gaussian {
    /// Construct from a single data point.
    pub fn new(x: f64) -> Gaussian {
        Gaussian {
            m1: x,
            m2: 0.0,
            n: 1.0,
        }
    }
    /// The mean of this distribution.
    pub fn mean(&self) -> f64 {
        self.m1
    }
    /// The (sample) variance of this distribution.
    pub fn variance(&self) -> f64 {
        assert!(self.n > 1.0, "Variance requires more than 1 sample.");
        self.m2 / (self.n - 1.0)
    }
    /// Probability Density Function.
    pub fn pdf(&self, x: f64) -> f64 {
        let m = self.mean();
        let v = self.variance();
        1.0 / (2.0 * PI * v).sqrt() * (-0.5 * ((x - m).powi(2) / v)).exp()
    }
    /// Cumulative Distribution Function.
    pub fn cdf(&self, x: f64) -> f64 {
        let m = self.mean();
        let v = self.variance();
        0.5 * (1.0 + libm::erf((x - m) / (2.0 * v).sqrt()))
    }
}

/// We can add a new data point to a Gaussian distribution.
impl Add<f64> for Gaussian {
    type Output = Self;
    fn add(self, x: f64) -> Self::Output {
        let n = self.n + 1.0;
        let m1 = self.m1 + (x - self.m1) / n;
        let m2 = self.m2 + (x - self.m1) * (x - m1);
        Gaussian { m1, m2, n }
    }
}

/// We can add a new data point to a Gaussian distribution.
impl AddAssign<f64> for Gaussian {
    fn add_assign(&mut self, x: f64) {
        self.n += 1.0;
        let m1_old = self.m1;
        self.m1 += (x - m1_old) / self.n;
        self.m2 += (x - m1_old) * (x - self.m1);
    }
}

/// Accumulate the points one at a time into a new Gaussian distribution.
impl FromIterator<f64> for Gaussian {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let mut g = Default::default();
        for i in iter {
            g += i
        }
        g
    }
}

/// Accumulate the points one at a time into a new Gaussian distribution.
impl<'a> FromIterator<&'a f64> for Gaussian {
    fn from_iter<I: IntoIterator<Item = &'a f64>>(iter: I) -> Self {
        let mut g = Default::default();
        for i in iter {
            g += *i
        }
        g
    }
}

/// Join together two gaussian distributions.
impl Semigroup for Gaussian {
    fn op(
        &Gaussian {
            m1: m1_a,
            m2: m2_a,
            n: n_a,
        }: &Self,
        &Gaussian {
            m1: m1_b,
            m2: m2_b,
            n: n_b,
        }: &Self,
    ) -> Self {
        let n = n_a + n_b;
        if n == 0.0 {
            Self::default()
        } else {
            let m1 = m1_a * (n_a / n) + m1_b * (n_b / n);
            let m2 = m2_a + m2_b + (m1_a - m1_b).powi(2) * (n_a * n_b) / n;
            Gaussian { m1, m2, n }
        }
    }
}

/// The "empty distribution."
impl Monoid for Gaussian {
    fn zero() -> Self {
        Self::default()
    }
}
