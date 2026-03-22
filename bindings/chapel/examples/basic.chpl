// SPDX-License-Identifier: MIT OR Apache-2.0
// Basic example of using Betlang from Chapel

// Compile with:
//   chpl basic.chpl -L../target/release -lbet_chapel -o basic
// Run with:
//   LD_LIBRARY_PATH=../target/release ./basic

module BetlangExample {
  use CTypes;

  // Import Betlang FFI functions
  extern proc bet_ternary(): c_int;
  extern proc bet_weighted_ternary(w0: c_double, w1: c_double, w2: c_double): c_int;
  extern proc bet_uniform(low: c_double, high: c_double): c_double;
  extern proc bet_normal(mean: c_double, std: c_double): c_double;
  extern proc bet_exponential(rate: c_double): c_double;
  extern proc bet_bernoulli(p: c_double): c_int;
  extern proc bet_sample_normal_array(out: c_ptr(c_double), n: c_size_t, mean: c_double, std: c_double);
  extern proc bet_mean(arr: c_ptr(c_double), n: c_size_t): c_double;
  extern proc bet_std(arr: c_ptr(c_double), n: c_size_t): c_double;
  extern proc bet_version(): c_ptrConst(c_char);

  proc main() {
    writeln("=== Betlang Chapel Bindings Demo ===");
    writeln();

    // Print version
    var version = bet_version();
    writeln("Betlang version: ", string.createCopyingBuffer(version));
    writeln();

    // Ternary bet demonstration
    writeln("--- Ternary Bet ---");
    var options = ["Rock", "Paper", "Scissors"];
    for i in 1..5 {
      var choice = bet_ternary();
      writeln("Game ", i, ": ", options[choice]);
    }
    writeln();

    // Weighted ternary bet (biased coin flip with third option)
    writeln("--- Weighted Ternary Bet ---");
    var outcomes = ["Win", "Lose", "Draw"];
    writeln("Weights: Win=0.4, Lose=0.4, Draw=0.2");
    for i in 1..5 {
      var result = bet_weighted_ternary(0.4, 0.4, 0.2);
      writeln("Round ", i, ": ", outcomes[result]);
    }
    writeln();

    // Continuous distributions
    writeln("--- Continuous Distributions ---");
    writeln("Uniform[0,10]: ", bet_uniform(0.0, 10.0):string);
    writeln("Normal(100, 15): ", bet_normal(100.0, 15.0):string);
    writeln("Exponential(2): ", bet_exponential(2.0):string);
    writeln();

    // Monte Carlo simulation
    writeln("--- Monte Carlo Pi Estimation ---");
    var inside = 0;
    const numSamples = 100000;

    for i in 1..numSamples {
      var x = bet_uniform(0.0, 1.0);
      var y = bet_uniform(0.0, 1.0);
      if x*x + y*y <= 1.0 {
        inside += 1;
      }
    }

    var piEstimate = 4.0 * inside:real / numSamples:real;
    writeln("Estimated π: ", piEstimate:string);
    writeln("Actual π:    3.14159...");
    writeln();

    // Array sampling
    writeln("--- Normal Samples Statistics ---");
    var samples: [0..999] c_double;
    bet_sample_normal_array(c_ptrTo(samples[0]), 1000, 0.0, 1.0);

    var mean = bet_mean(c_ptrTo(samples[0]), 1000);
    var std = bet_std(c_ptrTo(samples[0]), 1000);

    writeln("1000 samples from Normal(0, 1):");
    writeln("  Mean: ", mean:string);
    writeln("  Std:  ", std:string);
    writeln();

    // Parallel ternary sampling with Chapel's forall
    writeln("--- Parallel Ternary Bet Histogram ---");
    var counts: [0..2] atomic int;

    forall i in 1..10000 {
      var choice = bet_ternary();
      counts[choice].add(1);
    }

    writeln("10,000 uniform ternary samples:");
    for i in 0..2 {
      writeln("  Option ", i, ": ", counts[i].read(), " (",
              (100.0 * counts[i].read():real / 10000.0):string, "%)");
    }
    writeln();

    writeln("=== Demo Complete ===");
  }
}
