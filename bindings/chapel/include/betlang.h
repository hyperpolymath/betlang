#ifndef BET_CHAPEL_H
#define BET_CHAPEL_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Ternary bet: returns 0, 1, or 2 with equal probability
 * In Chapel, use this to index into a tuple/array of 3 alternatives
 */
int bet_ternary(void);

/**
 * Weighted ternary bet: returns 0, 1, or 2 with given weights
 * Weights are normalized internally
 */
int bet_weighted_ternary(double w0, double w1, double w2);

/**
 * Ternary logic value (Kleene): returns -1 (false), 0 (unknown), or 1 (true)
 */
int bet_ternary_logic(void);

/**
 * Uniform integer in [low, high]
 */
long bet_uniform_int(long low, long high);

/**
 * Bernoulli: returns 1 with probability p, else 0
 */
int bet_bernoulli(double p);

/**
 * Binomial: number of successes in n trials with probability p
 */
long bet_binomial(unsigned int n, double p);

/**
 * Poisson: count of events with rate lambda
 */
long bet_poisson(double lambda);

/**
 * Categorical: sample from discrete distribution with given weights
 * Returns index 0..n-1 based on weights
 */
int bet_categorical(const double *weights, size_t n);

/**
 * Uniform real in [low, high)
 */
double bet_uniform(double low, double high);

/**
 * Standard normal (mean=0, std=1)
 */
double bet_standard_normal(void);

/**
 * Normal with given mean and standard deviation
 */
double bet_normal(double mean, double std);

/**
 * Exponential with given rate
 */
double bet_exponential(double rate);

/**
 * Gamma with given shape and scale
 */
double bet_gamma(double shape, double scale);

/**
 * Beta with given alpha and beta parameters
 */
double bet_beta(double alpha, double beta);

/**
 * Sample n values from uniform [0, 1) into the provided array
 */
void bet_sample_uniform_array(double *out, size_t n);

/**
 * Sample n values from normal(mean, std) into the provided array
 */
void bet_sample_normal_array(double *out, size_t n, double mean, double std);

/**
 * Shuffle an integer array in place
 */
void bet_shuffle_int(long *arr, size_t n);

/**
 * Shuffle a double array in place
 */
void bet_shuffle_real(double *arr, size_t n);

/**
 * Sample k indices from 0..n without replacement
 * Returns number of samples actually written (min(k, n))
 */
size_t bet_sample_indices(long *out, size_t k, size_t n);

/**
 * Compute mean of an array
 */
double bet_mean(const double *arr, size_t n);

/**
 * Compute variance of an array
 */
double bet_variance(const double *arr, size_t n);

/**
 * Compute standard deviation of an array
 */
double bet_std(const double *arr, size_t n);

/**
 * Compute covariance of two arrays
 */
double bet_covariance(const double *x, const double *y, size_t n);

/**
 * Compute correlation of two arrays
 */
double bet_correlation(const double *x, const double *y, size_t n);

/**
 * Seed the random number generator (for reproducibility)
 * Note: This uses thread-local RNG, so it affects the calling thread only
 */
void bet_seed(uint64_t seed);

/**
 * Get library version as string
 */
const char *bet_version(void);

#endif /* BET_CHAPEL_H */
