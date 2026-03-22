// SPDX-License-Identifier: MIT OR Apache-2.0
//! Randomness primitives for Betlang
//!
//! This crate provides the core random number generation and
//! distribution sampling used by the betlang runtime.

#![forbid(unsafe_code)]
pub use rand;
pub use rand_distr;
pub use rand_pcg;
pub use rand_chacha;

use rand::prelude::*;
use rand_distr::{Distribution, Uniform};

/// Seeded RNG for reproducible randomness
pub type SeededRng = rand_pcg::Pcg64;

/// Create a new seeded RNG
pub fn seeded_rng(seed: u64) -> SeededRng {
    rand_pcg::Pcg64::seed_from_u64(seed)
}

/// Ternary choice - the core betlang primitive
/// Returns 0, 1, or 2 with equal probability
pub fn ternary() -> usize {
    thread_rng().gen_range(0..3)
}

/// Ternary choice with custom RNG
pub fn ternary_with<R: Rng>(rng: &mut R) -> usize {
    rng.gen_range(0..3)
}

/// Weighted ternary choice
/// Weights are normalized internally
pub fn weighted_ternary(w0: f64, w1: f64, w2: f64) -> usize {
    let total = w0 + w1 + w2;
    let r: f64 = thread_rng().gen();

    if r < w0 / total {
        0
    } else if r < (w0 + w1) / total {
        1
    } else {
        2
    }
}

/// Sample from uniform distribution
pub fn uniform(low: f64, high: f64) -> f64 {
    Uniform::new(low, high).sample(&mut thread_rng())
}

/// Sample from uniform integer distribution
pub fn uniform_int(low: i64, high: i64) -> i64 {
    thread_rng().gen_range(low..=high)
}

/// Shuffle a slice in place
pub fn shuffle<T>(slice: &mut [T]) {
    slice.shuffle(&mut thread_rng());
}

/// Sample k elements from a slice without replacement
pub fn sample<T: Clone>(slice: &[T], k: usize) -> Vec<T> {
    slice.choose_multiple(&mut thread_rng(), k).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary() {
        let mut counts = [0; 3];
        for _ in 0..3000 {
            counts[ternary()] += 1;
        }
        // Each should be roughly 1000
        for count in counts {
            assert!(count > 800 && count < 1200);
        }
    }

    #[test]
    fn test_seeded_reproducibility() {
        let mut rng1 = seeded_rng(42);
        let mut rng2 = seeded_rng(42);

        for _ in 0..100 {
            assert_eq!(ternary_with(&mut rng1), ternary_with(&mut rng2));
        }
    }
}
