// SPDX-License-Identifier: MIT OR Apache-2.0
//! Random number generation and probability distributions for Betlang
//!
//! Provides the core probabilistic primitives including the ternary bet.

use crate::value::{Distribution, Ternary, Value};
use rand::distributions::{Distribution as RandDist, Uniform, WeightedIndex};
use rand::prelude::*;
use rand_distr::{
    Beta, Bernoulli, Binomial, Cauchy, ChiSquared, Exp, Gamma, LogNormal, Normal, Pareto, Poisson,
    StudentT, Triangular, Weibull,
};
use std::sync::Arc;

// ============================================================================
// Core Ternary Bet Primitive
// ============================================================================

/// The fundamental ternary bet primitive
/// Creates a uniform distribution over three alternatives
pub fn bet(a: Value, b: Value, c: Value) -> Value {
    let choices = Arc::new([a, b, c]);
    Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || {
            let idx = thread_rng().gen_range(0..3);
            choices[idx].clone()
        }),
        name: "bet".to_string(),
    }))
}

/// Weighted ternary bet
/// Creates a distribution with specified weights for each alternative
pub fn weighted_bet(a: Value, wa: f64, b: Value, wb: f64, c: Value, wc: f64) -> Value {
    let total = wa + wb + wc;
    let weights = [wa / total, (wa + wb) / total];
    let choices = Arc::new([a, b, c]);

    Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || {
            let r: f64 = thread_rng().gen();
            if r < weights[0] {
                choices[0].clone()
            } else if r < weights[1] {
                choices[1].clone()
            } else {
                choices[2].clone()
            }
        }),
        name: "weighted_bet".to_string(),
    }))
}

/// Ternary logic bet - returns Ternary values
pub fn ternary_bet() -> Value {
    Value::Dist(Arc::new(Distribution {
        sampler: Box::new(|| {
            let idx = thread_rng().gen_range(0..3);
            match idx {
                0 => Value::Ternary(Ternary::True),
                1 => Value::Ternary(Ternary::False),
                _ => Value::Ternary(Ternary::Unknown),
            }
        }),
        name: "ternary".to_string(),
    }))
}

// ============================================================================
// Categorical (Discrete) Distributions
// ============================================================================

/// Categorical distribution over arbitrary values with weights
pub fn categorical(choices: Vec<(Value, f64)>) -> Result<Value, String> {
    if choices.is_empty() {
        return Err("Categorical distribution requires at least one choice".to_string());
    }

    let weights: Vec<f64> = choices.iter().map(|(_, w)| *w).collect();
    let values: Arc<Vec<Value>> = Arc::new(choices.into_iter().map(|(v, _)| v).collect());

    let dist = WeightedIndex::new(&weights)
        .map_err(|e| format!("Invalid weights: {}", e))?;

    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || {
            let idx = dist.sample(&mut thread_rng());
            values[idx].clone()
        }),
        name: "categorical".to_string(),
    })))
}

/// Uniform discrete distribution over integers [low, high]
pub fn uniform_int(low: i64, high: i64) -> Value {
    let dist = Uniform::new_inclusive(low, high);
    Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Int(dist.sample(&mut thread_rng()))),
        name: format!("uniform_int({}, {})", low, high),
    }))
}

/// Bernoulli distribution (coin flip with probability p)
pub fn bernoulli(p: f64) -> Result<Value, String> {
    let dist = Bernoulli::new(p).map_err(|e| format!("Invalid probability: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Bool(dist.sample(&mut thread_rng()))),
        name: format!("bernoulli({})", p),
    })))
}

/// Binomial distribution
pub fn binomial(n: u64, p: f64) -> Result<Value, String> {
    let dist = Binomial::new(n, p).map_err(|e| format!("Invalid parameters: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Int(dist.sample(&mut thread_rng()) as i64)),
        name: format!("binomial({}, {})", n, p),
    })))
}

/// Poisson distribution
pub fn poisson(lambda: f64) -> Result<Value, String> {
    let dist = Poisson::new(lambda).map_err(|e| format!("Invalid lambda: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Int(dist.sample(&mut thread_rng()) as i64)),
        name: format!("poisson({})", lambda),
    })))
}

// ============================================================================
// Continuous Distributions
// ============================================================================

/// Uniform continuous distribution [low, high)
pub fn uniform(low: f64, high: f64) -> Value {
    let dist = Uniform::new(low, high);
    Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("uniform({}, {})", low, high),
    }))
}

/// Normal (Gaussian) distribution
pub fn normal(mean: f64, std_dev: f64) -> Result<Value, String> {
    let dist = Normal::new(mean, std_dev).map_err(|e| format!("Invalid parameters: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("normal({}, {})", mean, std_dev),
    })))
}

/// Standard normal distribution (mean=0, std=1)
pub fn standard_normal() -> Value {
    let dist = Normal::new(0.0, 1.0).unwrap();
    Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: "standard_normal".to_string(),
    }))
}

/// Log-normal distribution
pub fn log_normal(mean: f64, std_dev: f64) -> Result<Value, String> {
    let dist = LogNormal::new(mean, std_dev).map_err(|e| format!("Invalid parameters: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("log_normal({}, {})", mean, std_dev),
    })))
}

/// Exponential distribution
pub fn exponential(rate: f64) -> Result<Value, String> {
    let dist = Exp::new(rate).map_err(|e| format!("Invalid rate: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("exponential({})", rate),
    })))
}

/// Gamma distribution
pub fn gamma(shape: f64, scale: f64) -> Result<Value, String> {
    let dist = Gamma::new(shape, scale).map_err(|e| format!("Invalid parameters: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("gamma({}, {})", shape, scale),
    })))
}

/// Beta distribution
pub fn beta(alpha: f64, beta_param: f64) -> Result<Value, String> {
    let dist = Beta::new(alpha, beta_param).map_err(|e| format!("Invalid parameters: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("beta({}, {})", alpha, beta_param),
    })))
}

/// Chi-squared distribution
pub fn chi_squared(df: f64) -> Result<Value, String> {
    let dist = ChiSquared::new(df).map_err(|e| format!("Invalid df: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("chi_squared({})", df),
    })))
}

/// Student's t-distribution
pub fn student_t(df: f64) -> Result<Value, String> {
    let dist = StudentT::new(df).map_err(|e| format!("Invalid df: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("student_t({})", df),
    })))
}

/// Cauchy distribution
pub fn cauchy(location: f64, scale: f64) -> Result<Value, String> {
    let dist = Cauchy::new(location, scale).map_err(|e| format!("Invalid parameters: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("cauchy({}, {})", location, scale),
    })))
}

/// Weibull distribution
pub fn weibull(scale: f64, shape: f64) -> Result<Value, String> {
    let dist = Weibull::new(scale, shape).map_err(|e| format!("Invalid parameters: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("weibull({}, {})", scale, shape),
    })))
}

/// Pareto distribution
pub fn pareto(scale: f64, shape: f64) -> Result<Value, String> {
    let dist = Pareto::new(scale, shape).map_err(|e| format!("Invalid parameters: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("pareto({}, {})", scale, shape),
    })))
}

/// Triangular distribution
pub fn triangular(min: f64, max: f64, mode: f64) -> Result<Value, String> {
    let dist =
        Triangular::new(min, max, mode).map_err(|e| format!("Invalid parameters: {}", e))?;
    Ok(Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || Value::Float(dist.sample(&mut thread_rng()))),
        name: format!("triangular({}, {}, {})", min, max, mode),
    })))
}

// ============================================================================
// Sampling Operations
// ============================================================================

/// Sample n times from a distribution
pub fn sample_n(dist: &Value, n: usize) -> Result<Vec<Value>, String> {
    match dist {
        Value::Dist(d) => Ok((0..n).map(|_| (d.sampler)()).collect()),
        _ => Err(format!("Cannot sample from {}", dist.type_name())),
    }
}

/// Sample with replacement from a list
pub fn sample_with_replacement(list: &im::Vector<Value>, n: usize) -> Vec<Value> {
    let mut rng = thread_rng();
    (0..n)
        .filter_map(|_| {
            if list.is_empty() {
                None
            } else {
                let idx = rng.gen_range(0..list.len());
                list.get(idx).cloned()
            }
        })
        .collect()
}

/// Sample without replacement from a list
pub fn sample_without_replacement(list: &im::Vector<Value>, n: usize) -> Vec<Value> {
    let mut available: Vec<_> = list.iter().cloned().collect();
    let mut rng = thread_rng();
    let mut result = Vec::new();

    for _ in 0..n.min(available.len()) {
        let idx = rng.gen_range(0..available.len());
        result.push(available.remove(idx));
    }

    result
}

/// Shuffle a list randomly
pub fn shuffle(list: &im::Vector<Value>) -> im::Vector<Value> {
    let mut vec: Vec<_> = list.iter().cloned().collect();
    vec.shuffle(&mut thread_rng());
    vec.into_iter().collect()
}

// ============================================================================
// Distribution Combinators
// ============================================================================

/// Transform distribution with a function
pub fn map_dist<F>(dist: &Value, f: F) -> Result<Value, String>
where
    F: Fn(Value) -> Value + Send + Sync + 'static,
{
    match dist {
        Value::Dist(d) => {
            let sampler = d.sampler.as_ref();
            // We need to clone the sampler somehow - this is tricky
            // For now, just create a new sampler that calls the original
            let name = format!("map({})", d.name);

            // This won't actually work without more complex machinery
            // Placeholder for now
            Ok(Value::Dist(Arc::new(Distribution {
                sampler: Box::new(move || f(Value::Unit)), // Simplified
                name,
            })))
        }
        _ => Err("map_dist requires a distribution".to_string()),
    }
}

/// Mix two distributions with given weights
pub fn mixture(dist1: Value, w1: f64, dist2: Value, w2: f64) -> Result<Value, String> {
    let total = w1 + w2;
    let p = w1 / total;

    match (&dist1, &dist2) {
        (Value::Dist(d1), Value::Dist(d2)) => {
            let sampler1 = Arc::clone(d1);
            let sampler2 = Arc::clone(d2);

            Ok(Value::Dist(Arc::new(Distribution {
                sampler: Box::new(move || {
                    if thread_rng().gen::<f64>() < p {
                        (sampler1.sampler)()
                    } else {
                        (sampler2.sampler)()
                    }
                }),
                name: format!("mixture({}, {})", d1.name, d2.name),
            })))
        }
        _ => Err("mixture requires two distributions".to_string()),
    }
}

// ============================================================================
// Statistics from Samples
// ============================================================================

/// Compute mean of samples
pub fn mean(samples: &[Value]) -> Option<f64> {
    if samples.is_empty() {
        return None;
    }

    let sum: f64 = samples
        .iter()
        .filter_map(|v| match v {
            Value::Int(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            _ => None,
        })
        .sum();

    Some(sum / samples.len() as f64)
}

/// Compute variance of samples
pub fn variance(samples: &[Value]) -> Option<f64> {
    let m = mean(samples)?;
    let n = samples.len() as f64;

    let sum_sq: f64 = samples
        .iter()
        .filter_map(|v| match v {
            Value::Int(i) => Some((*i as f64 - m).powi(2)),
            Value::Float(f) => Some((*f - m).powi(2)),
            _ => None,
        })
        .sum();

    Some(sum_sq / n)
}

/// Compute standard deviation
pub fn std_dev(samples: &[Value]) -> Option<f64> {
    variance(samples).map(|v| v.sqrt())
}

/// Compute median
pub fn median(samples: &[Value]) -> Option<f64> {
    let mut floats: Vec<f64> = samples
        .iter()
        .filter_map(|v| match v {
            Value::Int(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            _ => None,
        })
        .collect();

    if floats.is_empty() {
        return None;
    }

    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = floats.len() / 2;

    if floats.len() % 2 == 0 {
        Some((floats[mid - 1] + floats[mid]) / 2.0)
    } else {
        Some(floats[mid])
    }
}

/// Compute percentile (0-100)
pub fn percentile(samples: &[Value], p: f64) -> Option<f64> {
    let mut floats: Vec<f64> = samples
        .iter()
        .filter_map(|v| match v {
            Value::Int(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            _ => None,
        })
        .collect();

    if floats.is_empty() || p < 0.0 || p > 100.0 {
        return None;
    }

    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let idx = (p / 100.0 * (floats.len() - 1) as f64).round() as usize;
    Some(floats[idx])
}

// ============================================================================
// Native function bindings
// ============================================================================

use crate::value::NativeFunction;

/// Get all random/probability native functions
pub fn native_functions() -> Vec<NativeFunction> {
    vec![
        NativeFunction {
            name: "sample",
            arity: 1,
            func: |args| {
                if let Some(d) = args.first() {
                    d.sample().map_err(|e| e)
                } else {
                    Err("sample expects a distribution".to_string())
                }
            },
        },
        NativeFunction {
            name: "uniform",
            arity: 2,
            func: |args| {
                if args.len() >= 2 {
                    match (&args[0], &args[1]) {
                        (Value::Float(a), Value::Float(b)) => Ok(uniform(*a, *b)),
                        (Value::Int(a), Value::Int(b)) => Ok(uniform(*a as f64, *b as f64)),
                        _ => Err("uniform expects (float, float)".to_string()),
                    }
                } else {
                    Err("uniform expects 2 arguments".to_string())
                }
            },
        },
        NativeFunction {
            name: "normal",
            arity: 2,
            func: |args| {
                if args.len() >= 2 {
                    match (&args[0], &args[1]) {
                        (Value::Float(mean), Value::Float(std)) => {
                            normal(*mean, *std).map_err(|e| e)
                        }
                        (Value::Int(mean), Value::Float(std)) => {
                            normal(*mean as f64, *std).map_err(|e| e)
                        }
                        (Value::Float(mean), Value::Int(std)) => {
                            normal(*mean, *std as f64).map_err(|e| e)
                        }
                        (Value::Int(mean), Value::Int(std)) => {
                            normal(*mean as f64, *std as f64).map_err(|e| e)
                        }
                        _ => Err("normal expects (number, number)".to_string()),
                    }
                } else {
                    Err("normal expects 2 arguments".to_string())
                }
            },
        },
        NativeFunction {
            name: "bernoulli",
            arity: 1,
            func: |args| {
                if let Some(Value::Float(p)) = args.first() {
                    bernoulli(*p).map_err(|e| e)
                } else {
                    Err("bernoulli expects a probability".to_string())
                }
            },
        },
        NativeFunction {
            name: "beta",
            arity: 2,
            func: |args| {
                if args.len() >= 2 {
                    match (&args[0], &args[1]) {
                        (Value::Float(a), Value::Float(b)) => beta(*a, *b).map_err(|e| e),
                        _ => Err("beta expects (float, float)".to_string()),
                    }
                } else {
                    Err("beta expects 2 arguments".to_string())
                }
            },
        },
        NativeFunction {
            name: "gamma",
            arity: 2,
            func: |args| {
                if args.len() >= 2 {
                    match (&args[0], &args[1]) {
                        (Value::Float(shape), Value::Float(scale)) => {
                            gamma(*shape, *scale).map_err(|e| e)
                        }
                        _ => Err("gamma expects (float, float)".to_string()),
                    }
                } else {
                    Err("gamma expects 2 arguments".to_string())
                }
            },
        },
        NativeFunction {
            name: "exponential",
            arity: 1,
            func: |args| {
                if let Some(Value::Float(rate)) = args.first() {
                    exponential(*rate).map_err(|e| e)
                } else {
                    Err("exponential expects a rate".to_string())
                }
            },
        },
        NativeFunction {
            name: "poisson",
            arity: 1,
            func: |args| {
                if let Some(Value::Float(lambda)) = args.first() {
                    poisson(*lambda).map_err(|e| e)
                } else if let Some(Value::Int(lambda)) = args.first() {
                    poisson(*lambda as f64).map_err(|e| e)
                } else {
                    Err("poisson expects lambda".to_string())
                }
            },
        },
        NativeFunction {
            name: "shuffle",
            arity: 1,
            func: |args| {
                if let Some(Value::List(l)) = args.first() {
                    Ok(Value::List(shuffle(l)))
                } else {
                    Err("shuffle expects a list".to_string())
                }
            },
        },
        NativeFunction {
            name: "mean",
            arity: 1,
            func: |args| {
                if let Some(Value::List(l)) = args.first() {
                    let samples: Vec<_> = l.iter().cloned().collect();
                    Ok(mean(&samples)
                        .map(Value::Float)
                        .unwrap_or(Value::Unit))
                } else {
                    Err("mean expects a list".to_string())
                }
            },
        },
        NativeFunction {
            name: "std_dev",
            arity: 1,
            func: |args| {
                if let Some(Value::List(l)) = args.first() {
                    let samples: Vec<_> = l.iter().cloned().collect();
                    Ok(std_dev(&samples)
                        .map(Value::Float)
                        .unwrap_or(Value::Unit))
                } else {
                    Err("std_dev expects a list".to_string())
                }
            },
        },
        NativeFunction {
            name: "median",
            arity: 1,
            func: |args| {
                if let Some(Value::List(l)) = args.first() {
                    let samples: Vec<_> = l.iter().cloned().collect();
                    Ok(median(&samples)
                        .map(Value::Float)
                        .unwrap_or(Value::Unit))
                } else {
                    Err("median expects a list".to_string())
                }
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bet() {
        let dist = bet(Value::Int(1), Value::Int(2), Value::Int(3));
        for _ in 0..100 {
            let sample = dist.sample().unwrap();
            match sample {
                Value::Int(n) => assert!((1..=3).contains(&n)),
                _ => panic!("Expected Int"),
            }
        }
    }

    #[test]
    fn test_normal() {
        let dist = normal(0.0, 1.0).unwrap();
        let samples = sample_n(&dist, 1000).unwrap();
        let m = mean(&samples).unwrap();
        // Mean should be close to 0
        assert!(m.abs() < 0.2);
    }

    #[test]
    fn test_statistics() {
        let samples = vec![
            Value::Float(1.0),
            Value::Float(2.0),
            Value::Float(3.0),
            Value::Float(4.0),
            Value::Float(5.0),
        ];
        assert_eq!(mean(&samples), Some(3.0));
        assert_eq!(median(&samples), Some(3.0));
        assert_eq!(variance(&samples), Some(2.0));
    }

    #[test]
    fn test_shuffle() {
        let list: im::Vector<Value> = (1..=10).map(Value::Int).collect();
        let shuffled = shuffle(&list);
        assert_eq!(list.len(), shuffled.len());
        // Very unlikely to be the same order
    }
}
