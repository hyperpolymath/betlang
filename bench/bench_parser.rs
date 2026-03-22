// SPDX-License-Identifier: PMPL-1.0-or-later
// bench_parser.rs -- Parser benchmark harness for BetLang
//
// Generates a large synthetic BetLang program and measures
// parse throughput: LOC/sec, total parse time, AST node count.
//
// BetLang: ternary DSL for probabilistic modelling.
// Syntax: let/in, fun ->  , bet { A, B, C }, match ... end,
//         if/then/else/end, do ... end.
//
// Usage:  cargo run --release --example bench_parser

use std::time::Instant;

/// Generate a synthetic BetLang program.
fn generate_program(num_items: usize) -> String {
    let mut buf = String::with_capacity(num_items * 350);

    // Type definitions
    for i in (0..num_items).step_by(10) {
        buf.push_str(&format!("type Pair{} = (int, int)\n\n", i));
    }

    // Let definitions (top-level)
    for i in 0..num_items {
        buf.push_str(&format!(
            "let compute_{} x y = x + y * {}\n\n",
            i,
            i + 1
        ));

        // Bet expressions every 5th
        if i % 5 == 0 {
            buf.push_str(&format!(
                "let choice_{} = bet {{ {}, {}, {} }}\n\n",
                i, i, i + 1, i + 2
            ));
        }

        // Match expressions every 7th
        if i % 7 == 0 {
            buf.push_str(&format!("let classify_{} x = match x\n", i));
            buf.push_str(&format!("  true -> {}\n", i));
            buf.push_str(&format!("  false -> {}\n", i + 1));
            buf.push_str(&format!("  unknown -> {}\nend\n\n", i + 2));
        }

        // If-then-else every 8th
        if i % 8 == 0 {
            buf.push_str(&format!(
                "let branch_{} x = if x == {} then {} else {} end\n\n",
                i, i, i * 2, i * 3
            ));
        }

        // Lambda every 6th
        if i % 6 == 0 {
            buf.push_str(&format!(
                "let apply_{} = fun x -> x + {}\n\n",
                i,
                i + 1
            ));
        }

        // Let-in expressions every 9th
        if i % 9 == 0 {
            buf.push_str(&format!(
                "let bind_{} = let a = {} in let b = {} in a + b\n\n",
                i, i, i + 1
            ));
        }
    }

    buf
}

fn count_lines(s: &str) -> usize {
    s.lines().count()
}

fn main() {
    let num_items = 55;
    let iterations = 100;
    let source = generate_program(num_items);
    let loc = count_lines(&source);

    println!("=== BetLang Parser Benchmark ===");
    println!("Source: {} LOC, {} bytes", loc, source.len());
    println!("Iterations: {}\n", iterations);

    // Warm up
    {
        match bet_parse::parse(&source) {
            Ok(module) => println!("AST nodes (items): {}", module.items.len()),
            Err(e) => eprintln!("Warm-up parse error: {}", e),
        }
    }

    let start = Instant::now();
    for _ in 0..iterations {
        let result = bet_parse::parse(&source);
        std::hint::black_box(&result);
    }
    let elapsed = start.elapsed();

    let total_sec = elapsed.as_secs_f64();
    let per_iter = total_sec / iterations as f64;
    let loc_per_sec = (loc * iterations) as f64 / total_sec;

    println!("Total parse time : {:.4} s", total_sec);
    println!("Time per parse   : {:.6} s", per_iter);
    println!("LOC/sec          : {:.0}", loc_per_sec);
    println!("Bytes/sec        : {:.0}", (source.len() * iterations) as f64 / total_sec);
}
