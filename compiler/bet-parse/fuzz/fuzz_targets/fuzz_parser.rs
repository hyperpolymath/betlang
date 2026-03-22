// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell
//
// Fuzz target for the BetLang parser.
//
// Invariant: the parser must NEVER panic on ANY input. It should return
// ParseError, never abort.
//
// The BetLang parser is LALRPOP-generated. This harness feeds both raw
// UTF-8 and structured inputs through bet_parse::parse() and
// bet_parse::parse_expr().
//
// Run with:
//   cargo fuzz run fuzz_parser

#![no_main]

use libfuzzer_sys::fuzz_target;

/// BetLang keywords, operators, and common fragments.
const FRAGMENTS: &[&str] = &[
    // Keywords
    "let", "fun", "bet", "do", "end", "if", "then", "else", "match",
    "sample", "observe", "normal", "uniform", "bernoulli", "beta",
    "gamma", "poisson", "parallel", "true", "false",
    // Operators
    "+", "-", "*", "/", "%", "=", "==", "!=", "<", "<=", ">", ">=",
    "->", "<-", "|", "&&", "||", "!",
    // Delimiters
    "(", ")", "{", "}", "[", "]", ",", ";", ":",
    // Literals
    "42", "0", "3.14", "1e10", "0.5",
    // Identifiers
    "x", "y", "foo", "bar_baz", "_",
    // Whitespace
    " ", "\t", "\n",
    // Comments
    "// comment\n",
];

fn structured_input(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len() * 4);
    for &b in data {
        let idx = (b as usize) % FRAGMENTS.len();
        out.push_str(FRAGMENTS[idx]);
        if b & 0x80 != 0 {
            out.push(' ');
        }
    }
    out
}

fuzz_target!(|data: &[u8]| {
    // Strategy 1: raw UTF-8
    let raw = String::from_utf8_lossy(data);
    let _ = bet_parse::parse(&raw);
    let _ = bet_parse::parse_expr(&raw);

    // Strategy 2: structured token-plausible input
    if data.len() > 2 {
        let structured = structured_input(&data[2..]);
        let _ = bet_parse::parse(&structured);
        let _ = bet_parse::parse_expr(&structured);
    }
});
