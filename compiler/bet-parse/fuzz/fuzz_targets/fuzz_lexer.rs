// SPDX-License-Identifier: PMPL-1.0-or-later
//! Fuzz target for the BetLang lexer.
//!
//! Invariant: the lexer must NEVER panic on ANY input. It should always
//! return either Ok(tokens) or Err(LexError) without crashing.
//!
//! Run with:
//!   cargo +nightly fuzz run fuzz_lexer

#![no_main]

use libfuzzer_sys::fuzz_target;

use bet_parse::lexer::{lex, Lexer, Token};
use logos::Logos;

fuzz_target!(|data: &[u8]| {
    // Convert arbitrary bytes to a UTF-8 string (lossy — replaces invalid
    // sequences with U+FFFD). This is intentional: the lexer must handle
    // any valid UTF-8 string without panicking.
    let input = String::from_utf8_lossy(data);

    // --- Test the `lex` convenience function ---
    // Must return Ok or Err, never panic.
    let _ = lex(&input);

    // --- Test the Logos iterator directly ---
    // Iterate through every token the logos-generated lexer produces.
    let mut logos_lexer = Token::lexer(&input);
    while let Some(result) = logos_lexer.next() {
        // Each result is Ok(Token) or Err(()). Either is fine.
        let _ = result;
        // Accessing span must not panic.
        let _ = logos_lexer.span();
        let _ = logos_lexer.slice();
    }

    // --- Test the LALRPOP iterator adapter ---
    let lalrpop_lexer = Lexer::new(&input);
    for item in lalrpop_lexer {
        // Each item is Ok((usize, Token, usize)) or Err(LexError).
        let _ = item;
    }
});
