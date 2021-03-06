// Copyright 2018 Developers of the Rand project.
// Copyright 2013 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The dirichlet distribution.

use rand::Rng;
use crate::Distribution;
use crate::gamma::Gamma;

/// The dirichelet distribution `Dirichlet(alpha)`.
///
/// The Dirichlet distribution is a family of continuous multivariate
/// probability distributions parameterized by a vector alpha of positive reals.
/// It is a multivariate generalization of the beta distribution.
///
/// # Example
///
/// ```
/// use rand::prelude::*;
/// use rand_distr::Dirichlet;
///
/// let dirichlet = Dirichlet::new(vec![1.0, 2.0, 3.0]).unwrap();
/// let samples = dirichlet.sample(&mut rand::thread_rng());
/// println!("{:?} is from a Dirichlet([1.0, 2.0, 3.0]) distribution", samples);
/// ```
#[derive(Clone, Debug)]
pub struct Dirichlet {
    /// Concentration parameters (alpha)
    alpha: Vec<f64>,
}

/// Error type returned from `Dirchlet::new`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `alpha.len() < 2`.
    AlphaTooShort,
    /// `alpha <= 0.0` or `nan`.
    AlphaTooSmall,
    /// `size < 2`.
    SizeTooSmall,
}

impl Dirichlet {
    /// Construct a new `Dirichlet` with the given alpha parameter `alpha`.
    ///
    /// Requires `alpha.len() >= 2`.
    #[inline]
    pub fn new<V: Into<Vec<f64>>>(alpha: V) -> Result<Dirichlet, Error> {
        let a = alpha.into();
        if a.len() < 2 {
            return Err(Error::AlphaTooShort);
        }
        for i in 0..a.len() {
            if !(a[i] > 0.0) {
                return Err(Error::AlphaTooSmall);
            }
        }

        Ok(Dirichlet { alpha: a })
    }

    /// Construct a new `Dirichlet` with the given shape parameter `alpha` and `size`.
    ///
    /// Requires `size >= 2`.
    #[inline]
    pub fn new_with_size(alpha: f64, size: usize) -> Result<Dirichlet, Error> {
        if !(alpha > 0.0) {
            return Err(Error::AlphaTooSmall);
        }
        if size < 2 {
            return Err(Error::SizeTooSmall);
        }
        Ok(Dirichlet {
            alpha: vec![alpha; size],
        })
    }
}

impl Distribution<Vec<f64>> for Dirichlet {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec<f64> {
        let n = self.alpha.len();
        let mut samples = vec![0.0f64; n];
        let mut sum = 0.0f64;

        for i in 0..n {
            let g = Gamma::new(self.alpha[i], 1.0).unwrap();
            samples[i] = g.sample(rng);
            sum += samples[i];
        }
        let invacc = 1.0 / sum;
        for i in 0..n {
            samples[i] *= invacc;
        }
        samples
    }
}

#[cfg(test)]
mod test {
    use super::Dirichlet;
    use crate::Distribution;

    #[test]
    fn test_dirichlet() {
        let d = Dirichlet::new(vec![1.0, 2.0, 3.0]).unwrap();
        let mut rng = crate::test::rng(221);
        let samples = d.sample(&mut rng);
        let _: Vec<f64> = samples
            .into_iter()
            .map(|x| {
                assert!(x > 0.0);
                x
            })
            .collect();
    }

    #[test]
    fn test_dirichlet_with_param() {
        let alpha = 0.5f64;
        let size = 2;
        let d = Dirichlet::new_with_size(alpha, size).unwrap();
        let mut rng = crate::test::rng(221);
        let samples = d.sample(&mut rng);
        let _: Vec<f64> = samples
            .into_iter()
            .map(|x| {
                assert!(x > 0.0);
                x
            })
            .collect();
    }

    #[test]
    #[should_panic]
    fn test_dirichlet_invalid_length() {
        Dirichlet::new_with_size(0.5f64, 1).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_dirichlet_invalid_alpha() {
        Dirichlet::new_with_size(0.0f64, 2).unwrap();
    }
}
