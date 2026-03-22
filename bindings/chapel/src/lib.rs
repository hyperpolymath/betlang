// SPDX-License-Identifier: MIT OR Apache-2.0
//! Chapel language bindings for Betlang
//!
//! Provides C-compatible FFI for Chapel programs to use betlang primitives.
//!
//! Chapel can call these functions using its extern blocks:
//!
//! ```chapel
//! extern proc bet_uniform(low: real, high: real): real;
//! extern proc bet_normal(mean: real, std: real): real;
//! extern proc bet_ternary(): c_int;
//! ```

use libc::{c_char, c_double, c_int, c_long, c_uint, size_t};
use rand::prelude::*;
use rand_distr::{Beta, Bernoulli, Binomial, Exp, Gamma, Normal, Poisson};
use std::ffi::{CStr, CString};
use std::ptr;

// ============================================================================
// Core Ternary Bet Primitive
// ============================================================================

/// Ternary bet: returns 0, 1, or 2 with equal probability
/// In Chapel, use this to index into a tuple/array of 3 alternatives
#[no_mangle]
pub extern "C" fn bet_ternary() -> c_int {
    thread_rng().gen_range(0..3)
}

/// Weighted ternary bet: returns 0, 1, or 2 with given weights
/// Weights are normalized internally
#[no_mangle]
pub extern "C" fn bet_weighted_ternary(w0: c_double, w1: c_double, w2: c_double) -> c_int {
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

/// Ternary logic value (Kleene): returns -1 (false), 0 (unknown), or 1 (true)
#[no_mangle]
pub extern "C" fn bet_ternary_logic() -> c_int {
    thread_rng().gen_range(-1..=1)
}

// ============================================================================
// Discrete Distributions
// ============================================================================

/// Uniform integer in [low, high]
#[no_mangle]
pub extern "C" fn bet_uniform_int(low: c_long, high: c_long) -> c_long {
    thread_rng().gen_range(low..=high)
}

/// Bernoulli: returns 1 with probability p, else 0
#[no_mangle]
pub extern "C" fn bet_bernoulli(p: c_double) -> c_int {
    if let Ok(dist) = Bernoulli::new(p) {
        if dist.sample(&mut thread_rng()) { 1 } else { 0 }
    } else {
        0
    }
}

/// Binomial: number of successes in n trials with probability p
#[no_mangle]
pub extern "C" fn bet_binomial(n: c_uint, p: c_double) -> c_long {
    if let Ok(dist) = Binomial::new(n as u64, p) {
        dist.sample(&mut thread_rng()) as c_long
    } else {
        0
    }
}

/// Poisson: count of events with rate lambda
#[no_mangle]
pub extern "C" fn bet_poisson(lambda: c_double) -> c_long {
    if let Ok(dist) = Poisson::new(lambda) {
        dist.sample(&mut thread_rng()) as c_long
    } else {
        0
    }
}

/// Categorical: sample from discrete distribution with given weights
/// Returns index 0..n-1 based on weights
#[no_mangle]
pub extern "C" fn bet_categorical(weights: *const c_double, n: size_t) -> c_int {
    if weights.is_null() || n == 0 {
        return 0;
    }

    let weights_slice = unsafe { std::slice::from_raw_parts(weights, n) };
    let total: f64 = weights_slice.iter().sum();

    if total <= 0.0 {
        return 0;
    }

    let r: f64 = thread_rng().gen::<f64>() * total;
    let mut cumulative = 0.0;

    for (i, &w) in weights_slice.iter().enumerate() {
        cumulative += w;
        if r < cumulative {
            return i as c_int;
        }
    }

    (n - 1) as c_int
}

// ============================================================================
// Continuous Distributions
// ============================================================================

/// Uniform real in [low, high)
#[no_mangle]
pub extern "C" fn bet_uniform(low: c_double, high: c_double) -> c_double {
    thread_rng().gen_range(low..high)
}

/// Standard normal (mean=0, std=1)
#[no_mangle]
pub extern "C" fn bet_standard_normal() -> c_double {
    let dist = Normal::new(0.0, 1.0).unwrap();
    dist.sample(&mut thread_rng())
}

/// Normal with given mean and standard deviation
#[no_mangle]
pub extern "C" fn bet_normal(mean: c_double, std: c_double) -> c_double {
    if let Ok(dist) = Normal::new(mean, std) {
        dist.sample(&mut thread_rng())
    } else {
        mean
    }
}

/// Exponential with given rate
#[no_mangle]
pub extern "C" fn bet_exponential(rate: c_double) -> c_double {
    if let Ok(dist) = Exp::new(rate) {
        dist.sample(&mut thread_rng())
    } else {
        0.0
    }
}

/// Gamma with given shape and scale
#[no_mangle]
pub extern "C" fn bet_gamma(shape: c_double, scale: c_double) -> c_double {
    if let Ok(dist) = Gamma::new(shape, scale) {
        dist.sample(&mut thread_rng())
    } else {
        0.0
    }
}

/// Beta with given alpha and beta parameters
#[no_mangle]
pub extern "C" fn bet_beta(alpha: c_double, beta: c_double) -> c_double {
    if let Ok(dist) = Beta::new(alpha, beta) {
        dist.sample(&mut thread_rng())
    } else {
        0.5
    }
}

// ============================================================================
// Sampling Utilities
// ============================================================================

/// Sample n values from uniform [0, 1) into the provided array
#[no_mangle]
pub extern "C" fn bet_sample_uniform_array(out: *mut c_double, n: size_t) {
    if out.is_null() || n == 0 {
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts_mut(out, n) };
    let mut rng = thread_rng();

    for x in slice.iter_mut() {
        *x = rng.gen();
    }
}

/// Sample n values from normal(mean, std) into the provided array
#[no_mangle]
pub extern "C" fn bet_sample_normal_array(
    out: *mut c_double,
    n: size_t,
    mean: c_double,
    std: c_double,
) {
    if out.is_null() || n == 0 {
        return;
    }

    if let Ok(dist) = Normal::new(mean, std) {
        let slice = unsafe { std::slice::from_raw_parts_mut(out, n) };
        let mut rng = thread_rng();

        for x in slice.iter_mut() {
            *x = dist.sample(&mut rng);
        }
    }
}

/// Shuffle an integer array in place
#[no_mangle]
pub extern "C" fn bet_shuffle_int(arr: *mut c_long, n: size_t) {
    if arr.is_null() || n == 0 {
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts_mut(arr, n) };
    slice.shuffle(&mut thread_rng());
}

/// Shuffle a double array in place
#[no_mangle]
pub extern "C" fn bet_shuffle_real(arr: *mut c_double, n: size_t) {
    if arr.is_null() || n == 0 {
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts_mut(arr, n) };
    slice.shuffle(&mut thread_rng());
}

/// Sample k indices from 0..n without replacement
/// Returns number of samples actually written (min(k, n))
#[no_mangle]
pub extern "C" fn bet_sample_indices(out: *mut c_long, k: size_t, n: size_t) -> size_t {
    if out.is_null() || k == 0 || n == 0 {
        return 0;
    }

    let actual_k = k.min(n);
    let slice = unsafe { std::slice::from_raw_parts_mut(out, actual_k) };

    // Fisher-Yates sampling
    let mut indices: Vec<c_long> = (0..n as c_long).collect();
    indices.shuffle(&mut thread_rng());

    for (i, idx) in indices.iter().take(actual_k).enumerate() {
        slice[i] = *idx;
    }

    actual_k
}

// ============================================================================
// Statistics
// ============================================================================

/// Compute mean of an array
#[no_mangle]
pub extern "C" fn bet_mean(arr: *const c_double, n: size_t) -> c_double {
    if arr.is_null() || n == 0 {
        return 0.0;
    }

    let slice = unsafe { std::slice::from_raw_parts(arr, n) };
    slice.iter().sum::<f64>() / n as f64
}

/// Compute variance of an array
#[no_mangle]
pub extern "C" fn bet_variance(arr: *const c_double, n: size_t) -> c_double {
    if arr.is_null() || n == 0 {
        return 0.0;
    }

    let slice = unsafe { std::slice::from_raw_parts(arr, n) };
    let mean = slice.iter().sum::<f64>() / n as f64;
    slice.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64
}

/// Compute standard deviation of an array
#[no_mangle]
pub extern "C" fn bet_std(arr: *const c_double, n: size_t) -> c_double {
    bet_variance(arr, n).sqrt()
}

/// Compute covariance of two arrays
#[no_mangle]
pub extern "C" fn bet_covariance(
    x: *const c_double,
    y: *const c_double,
    n: size_t,
) -> c_double {
    if x.is_null() || y.is_null() || n == 0 {
        return 0.0;
    }

    let x_slice = unsafe { std::slice::from_raw_parts(x, n) };
    let y_slice = unsafe { std::slice::from_raw_parts(y, n) };

    let x_mean = x_slice.iter().sum::<f64>() / n as f64;
    let y_mean = y_slice.iter().sum::<f64>() / n as f64;

    x_slice
        .iter()
        .zip(y_slice.iter())
        .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean))
        .sum::<f64>()
        / n as f64
}

/// Compute correlation of two arrays
#[no_mangle]
pub extern "C" fn bet_correlation(
    x: *const c_double,
    y: *const c_double,
    n: size_t,
) -> c_double {
    let cov = bet_covariance(x, y, n);
    let std_x = bet_std(x, n);
    let std_y = bet_std(y, n);

    if std_x == 0.0 || std_y == 0.0 {
        0.0
    } else {
        cov / (std_x * std_y)
    }
}

// ============================================================================
// RNG State Management
// ============================================================================

/// Seed the random number generator (for reproducibility)
/// Note: This uses thread-local RNG, so it affects the calling thread only
#[no_mangle]
pub extern "C" fn bet_seed(seed: u64) {
    // Thread-local storage for seeded RNG
    // This is a simplified approach; production code might want ChaCha
    use std::cell::RefCell;

    thread_local! {
        static SEEDED_RNG: RefCell<Option<rand_pcg::Pcg64>> = const { RefCell::new(None) };
    }

    SEEDED_RNG.with(|rng| {
        *rng.borrow_mut() = Some(rand_pcg::Pcg64::seed_from_u64(seed));
    });
}

// ============================================================================
// Version Info
// ============================================================================

/// Get library version as string
#[no_mangle]
pub extern "C" fn bet_version() -> *const c_char {
    static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    VERSION.as_ptr() as *const c_char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary() {
        let mut counts = [0; 3];
        for _ in 0..3000 {
            let r = bet_ternary();
            assert!((0..3).contains(&r));
            counts[r as usize] += 1;
        }
        // Each outcome should occur roughly 1000 times
        for count in counts {
            assert!(count > 800 && count < 1200);
        }
    }

    #[test]
    fn test_normal() {
        let mut samples = [0.0; 1000];
        bet_sample_normal_array(samples.as_mut_ptr(), 1000, 0.0, 1.0);

        let mean = bet_mean(samples.as_ptr(), 1000);
        let std = bet_std(samples.as_ptr(), 1000);

        // Mean should be close to 0, std close to 1
        assert!(mean.abs() < 0.1);
        assert!((std - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_categorical() {
        let weights = [1.0, 2.0, 3.0]; // 1/6, 2/6, 3/6 probabilities
        let mut counts = [0; 3];

        for _ in 0..6000 {
            let r = bet_categorical(weights.as_ptr(), 3);
            counts[r as usize] += 1;
        }

        // Rough expected: 1000, 2000, 3000
        assert!(counts[0] > 700 && counts[0] < 1300);
        assert!(counts[1] > 1500 && counts[1] < 2500);
        assert!(counts[2] > 2500 && counts[2] < 3500);
    }
}
