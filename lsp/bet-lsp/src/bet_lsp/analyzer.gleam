// SPDX-License-Identifier: MIT OR Apache-2.0
//// Code analysis for Betlang LSP
////
//// Provides parsing, diagnostics, hover info, and completions.

import gleam/list
import gleam/string
import gleam/option.{type Option, None, Some}
import gleam/regex
import gleam/result
import bet_lsp/state.{type Diagnostic, type DiagnosticSeverity, type Range, Error, Warning, Information, Hint}

/// Completion item
pub type CompletionItem {
  CompletionItem(
    label: String,
    kind: Int,       // LSP CompletionItemKind
    detail: String,
    insert_text: String,
  )
}

/// Symbol info
pub type Symbol {
  Symbol(
    name: String,
    kind: Int,       // LSP SymbolKind
    range: Range,
  )
}

// LSP CompletionItemKind constants
const keyword_kind = 14
const function_kind = 3
const variable_kind = 6
const module_kind = 9

// LSP SymbolKind constants
const function_symbol = 12
const variable_symbol = 13
const module_symbol = 2

// ============================================================================
// Diagnostics
// ============================================================================

/// Analyze source code and return diagnostics
pub fn analyze(source: String) -> List(Diagnostic) {
  let lines = string.split(source, "\n")

  lines
  |> list.index_map(fn(line, idx) { check_line(line, idx) })
  |> list.flatten
}

/// Check a single line for issues
fn check_line(line: String, line_num: Int) -> List(Diagnostic) {
  let diagnostics = []

  // Check for unclosed bet expressions
  let diagnostics = case check_unclosed_bet(line, line_num) {
    Some(d) -> [d, ..diagnostics]
    None -> diagnostics
  }

  // Check for unknown keywords
  let diagnostics = case check_unknown_keyword(line, line_num) {
    Some(d) -> [d, ..diagnostics]
    None -> diagnostics
  }

  // Check for unbalanced braces
  let diagnostics = case check_braces(line, line_num) {
    Some(d) -> [d, ..diagnostics]
    None -> diagnostics
  }

  // Check for deprecated syntax
  let diagnostics = case check_deprecated(line, line_num) {
    Some(d) -> [d, ..diagnostics]
    None -> diagnostics
  }

  diagnostics
}

fn check_unclosed_bet(line: String, line_num: Int) -> Option(Diagnostic) {
  let open_count = string.split(line, "bet {") |> list.length |> fn(x) { x - 1 }
  let close_count = string.split(line, "}") |> list.length |> fn(x) { x - 1 }

  case open_count > close_count {
    True -> Some(Diagnostic(
      range: Range(line_num, 0, line_num, string.length(line)),
      severity: Error,
      message: "Unclosed bet expression - missing '}'",
      source: "bet-lsp",
    ))
    False -> None
  }
}

fn check_unknown_keyword(line: String, line_num: Int) -> Option(Diagnostic) {
  // Simple keyword check
  let keywords = ["let", "in", "fun", "if", "then", "else", "match", "with", "bet", "true", "false", "unknown"]

  case find_identifier(line) {
    Some(#(ident, start)) -> {
      case list.contains(keywords, ident) || string.starts_with(ident, "_") {
        True -> None
        False -> None  // Don't flag unknown identifiers as errors
      }
    }
    None -> None
  }
}

fn find_identifier(line: String) -> Option(#(String, Int)) {
  // Simplified - just find first word
  case regex.from_string("\\b[a-z_][a-zA-Z0-9_]*\\b") {
    Ok(re) -> {
      case regex.scan(re, line) {
        [match, ..] -> Some(#(match.content, 0))
        [] -> None
      }
    }
    Error(_) -> None
  }
}

fn check_braces(line: String, line_num: Int) -> Option(Diagnostic) {
  let open = string.split(line, "{") |> list.length |> fn(x) { x - 1 }
  let close = string.split(line, "}") |> list.length |> fn(x) { x - 1 }

  case open != close {
    True -> Some(Diagnostic(
      range: Range(line_num, 0, line_num, string.length(line)),
      severity: Warning,
      message: "Unbalanced braces on this line",
      source: "bet-lsp",
    ))
    False -> None
  }
}

fn check_deprecated(line: String, line_num: Int) -> Option(Diagnostic) {
  // Check for deprecated patterns
  case string.contains(line, "rand()") {
    True -> Some(Diagnostic(
      range: Range(line_num, 0, line_num, string.length(line)),
      severity: Information,
      message: "rand() is deprecated, use uniform(0.0, 1.0) instead",
      source: "bet-lsp",
    ))
    False -> None
  }
}

// ============================================================================
// Hover Information
// ============================================================================

/// Get hover information at position
pub fn get_hover(source: String, line: Int, character: Int) -> Option(String) {
  let lines = string.split(source, "\n")

  case list.at(lines, line) {
    Ok(line_text) -> {
      let word = get_word_at(line_text, character)
      get_hover_for_word(word)
    }
    Error(_) -> None
  }
}

fn get_word_at(line: String, character: Int) -> String {
  // Simplified word extraction
  let chars = string.to_graphemes(line)
  let before = list.take(chars, character)
  let after = list.drop(chars, character)

  let word_before = before
    |> list.reverse
    |> list.take_while(is_word_char)
    |> list.reverse
    |> string.join("")

  let word_after = after
    |> list.take_while(is_word_char)
    |> string.join("")

  word_before <> word_after
}

fn is_word_char(c: String) -> Bool {
  case regex.from_string("[a-zA-Z0-9_]") {
    Ok(re) -> regex.check(re, c)
    Error(_) -> False
  }
}

fn get_hover_for_word(word: String) -> Option(String) {
  case word {
    "bet" -> Some("```betlang\nbet { A, B, C }\n```\n\n**Ternary Bet**\n\nUniformly selects one of three alternatives with probability 1/3 each.\n\n*Core primitive of Betlang.*")

    "let" -> Some("```betlang\nlet x = expr\n```\n\n**Let Binding**\n\nBinds a value to a name in the current scope.")

    "fun" -> Some("```betlang\nfun x -> expr\n```\n\n**Lambda Function**\n\nCreates an anonymous function.")

    "if" -> Some("```betlang\nif cond then expr1 else expr2\n```\n\n**Conditional Expression**\n\nEvaluates to `expr1` if `cond` is true, otherwise `expr2`.")

    "match" -> Some("```betlang\nmatch expr with\n| pattern -> result\n```\n\n**Pattern Matching**\n\nMatches expression against patterns.")

    "true" -> Some("```betlang\ntrue : Bool\n```\n\n**Boolean True**\n\nThe boolean value true.")

    "false" -> Some("```betlang\nfalse : Bool\n```\n\n**Boolean False**\n\nThe boolean value false.")

    "unknown" -> Some("```betlang\nunknown : Ternary\n```\n\n**Ternary Unknown**\n\nThe third truth value in Kleene's three-valued logic.")

    "uniform" -> Some("```betlang\nuniform : Float -> Float -> Dist Float\n```\n\n**Uniform Distribution**\n\nCreates a uniform distribution over [low, high).")

    "normal" -> Some("```betlang\nnormal : Float -> Float -> Dist Float\n```\n\n**Normal Distribution**\n\nCreates a normal (Gaussian) distribution with given mean and standard deviation.")

    "sample" -> Some("```betlang\nsample : Dist a -> a\n```\n\n**Sample from Distribution**\n\nDraws a random sample from the distribution.")

    "mean" -> Some("```betlang\nmean : List Float -> Float\n```\n\n**Arithmetic Mean**\n\nComputes the average of a list of numbers.")

    "std" -> Some("```betlang\nstd : List Float -> Float\n```\n\n**Standard Deviation**\n\nComputes the standard deviation of a list of numbers.")

    _ -> None
  }
}

// ============================================================================
// Completions
// ============================================================================

/// Get completion items at position
pub fn get_completions(source: String, line: Int, character: Int) -> List(CompletionItem) {
  let lines = string.split(source, "\n")

  let prefix = case list.at(lines, line) {
    Ok(line_text) -> get_prefix_at(line_text, character)
    Error(_) -> ""
  }

  // Filter completions by prefix
  all_completions()
  |> list.filter(fn(c) { string.starts_with(c.label, prefix) })
}

fn get_prefix_at(line: String, character: Int) -> String {
  let chars = string.to_graphemes(line)
  let before = list.take(chars, character)

  before
  |> list.reverse
  |> list.take_while(is_word_char)
  |> list.reverse
  |> string.join("")
}

fn all_completions() -> List(CompletionItem) {
  [
    // Keywords
    CompletionItem("bet", keyword_kind, "Ternary bet expression", "bet { ${1:a}, ${2:b}, ${3:c} }"),
    CompletionItem("let", keyword_kind, "Let binding", "let ${1:name} = ${2:value}"),
    CompletionItem("fun", keyword_kind, "Lambda function", "fun ${1:x} -> ${2:body}"),
    CompletionItem("if", keyword_kind, "Conditional", "if ${1:cond} then ${2:then} else ${3:else}"),
    CompletionItem("match", keyword_kind, "Pattern match", "match ${1:expr} with\n| ${2:pattern} -> ${3:result}"),
    CompletionItem("true", keyword_kind, "Boolean true", "true"),
    CompletionItem("false", keyword_kind, "Boolean false", "false"),
    CompletionItem("unknown", keyword_kind, "Ternary unknown", "unknown"),

    // Distributions
    CompletionItem("uniform", function_kind, "Uniform distribution", "uniform(${1:low}, ${2:high})"),
    CompletionItem("normal", function_kind, "Normal distribution", "normal(${1:mean}, ${2:std})"),
    CompletionItem("bernoulli", function_kind, "Bernoulli distribution", "bernoulli(${1:p})"),
    CompletionItem("binomial", function_kind, "Binomial distribution", "binomial(${1:n}, ${2:p})"),
    CompletionItem("poisson", function_kind, "Poisson distribution", "poisson(${1:lambda})"),
    CompletionItem("exponential", function_kind, "Exponential distribution", "exponential(${1:rate})"),
    CompletionItem("gamma", function_kind, "Gamma distribution", "gamma(${1:shape}, ${2:scale})"),
    CompletionItem("beta", function_kind, "Beta distribution", "beta(${1:alpha}, ${2:beta})"),

    // Functions
    CompletionItem("sample", function_kind, "Sample from distribution", "sample(${1:dist})"),
    CompletionItem("mean", function_kind, "Compute mean", "mean(${1:list})"),
    CompletionItem("std", function_kind, "Compute standard deviation", "std(${1:list})"),
    CompletionItem("variance", function_kind, "Compute variance", "variance(${1:list})"),
    CompletionItem("median", function_kind, "Compute median", "median(${1:list})"),
    CompletionItem("println", function_kind, "Print with newline", "println(${1:value})"),
    CompletionItem("print", function_kind, "Print without newline", "print(${1:value})"),
    CompletionItem("replicate", function_kind, "Repeat n times", "replicate(${1:n}, ${2:fn})"),
    CompletionItem("map", function_kind, "Map over list", "map(${1:fn}, ${2:list})"),
    CompletionItem("filter", function_kind, "Filter list", "filter(${1:pred}, ${2:list})"),
    CompletionItem("fold", function_kind, "Fold list", "fold(${1:fn}, ${2:init}, ${3:list})"),
  ]
}

// ============================================================================
// Symbols
// ============================================================================

/// Get document symbols
pub fn get_symbols(source: String) -> List(Symbol) {
  let lines = string.split(source, "\n")

  lines
  |> list.index_map(fn(line, idx) { find_symbols(line, idx) })
  |> list.flatten
}

fn find_symbols(line: String, line_num: Int) -> List(Symbol) {
  let symbols = []

  // Find let bindings
  let symbols = case find_let_binding(line, line_num) {
    Some(s) -> [s, ..symbols]
    None -> symbols
  }

  // Find function definitions
  let symbols = case find_function_def(line, line_num) {
    Some(s) -> [s, ..symbols]
    None -> symbols
  }

  symbols
}

fn find_let_binding(line: String, line_num: Int) -> Option(Symbol) {
  case regex.from_string("let\\s+([a-zA-Z_][a-zA-Z0-9_]*)\\s*=") {
    Ok(re) -> {
      case regex.scan(re, line) {
        [match, ..] -> {
          case match.submatches {
            [Some(name), ..] -> Some(Symbol(
              name: name,
              kind: variable_symbol,
              range: Range(line_num, 0, line_num, string.length(line)),
            ))
            _ -> None
          }
        }
        [] -> None
      }
    }
    Error(_) -> None
  }
}

fn find_function_def(line: String, line_num: Int) -> Option(Symbol) {
  case regex.from_string("let\\s+([a-zA-Z_][a-zA-Z0-9_]*)\\s*=\\s*fun") {
    Ok(re) -> {
      case regex.scan(re, line) {
        [match, ..] -> {
          case match.submatches {
            [Some(name), ..] -> Some(Symbol(
              name: name,
              kind: function_symbol,
              range: Range(line_num, 0, line_num, string.length(line)),
            ))
            _ -> None
          }
        }
        [] -> None
      }
    }
    Error(_) -> None
  }
}

// ============================================================================
// Evaluation
// ============================================================================

/// Evaluate code (for REPL)
pub fn eval(code: String) -> String {
  // This would call the actual betlang interpreter
  // For now, return a placeholder
  "=> <evaluation result for: " <> string.slice(code, 0, 50) <> ">"
}
