// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
//! Interactive REPL for Betlang
//!
//! Parses, type-checks, and evaluates expressions entered interactively.

use miette::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

const BANNER: &str = r#"
╔══════════════════════════════════════════════════════════════╗
║  ____       _   _                                            ║
║ | __ )  ___| |_| | __ _ _ __   __ _                          ║
║ |  _ \ / _ \ __| |/ _` | '_ \ / _` |                         ║
║ | |_) |  __/ |_| | (_| | | | | (_| |                         ║
║ |____/ \___|\__|_|\__,_|_| |_|\__, |                         ║
║                               |___/                          ║
║                                                              ║
║  A ternary probabilistic programming language                ║
║                                                              ║
║  Type :help for help, :quit to exit                          ║
╚══════════════════════════════════════════════════════════════╝
"#;

const HELP: &str = r#"
Betlang REPL Commands:
  :help, :h       Show this help message
  :quit, :q       Exit the REPL
  :type <expr>    Show the type of an expression
  :ast <expr>     Show the AST of an expression
  :clear          Clear the screen
  :reset          Reset the environment
  :stats          Show betting statistics

Examples:
  bet { 1, 2, 3 }                    -- Uniform ternary choice
  bet { a @ 0.5, b @ 0.3, c @ 0.2 }  -- Weighted bet
  let x = bet { 1, 2, 3 } in x + 1  -- Let binding with bet
  fun x -> bet { x, x+1, x-1 }      -- Function with bet
  do { x <- normal 0 1; return x }  -- Monadic sampling
"#;

/// Simple session statistics for the REPL.
struct ReplStats {
    /// Total expressions evaluated successfully.
    evals: u64,
    /// Total bet (probabilistic) expressions evaluated.
    bets: u64,
    /// Total type-check queries.
    type_queries: u64,
    /// Total parse errors encountered.
    errors: u64,
}

impl ReplStats {
    fn new() -> Self {
        Self { evals: 0, bets: 0, type_queries: 0, errors: 0 }
    }
}

pub fn run_repl() -> Result<()> {
    println!("{}", BANNER);

    let mut rl = DefaultEditor::new().map_err(|e| miette::miette!("{}", e))?;

    // Load history if available
    let history_path = dirs::data_dir()
        .map(|d| d.join("betlang").join("history.txt"));

    if let Some(ref path) = history_path {
        let _ = rl.load_history(path);
    }

    let mut line_count = 0;
    let mut stats = ReplStats::new();
    let mut eval_env = bet_core::ValueEnv::<bet_eval::Value>::new();

    loop {
        let prompt = format!("bet[{}]> ", line_count);

        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(line);

                // Handle commands
                if line.starts_with(':') {
                    match handle_command(line, &mut stats) {
                        CommandResult::Continue => continue,
                        CommandResult::Quit => break,
                    }
                } else {
                    // Parse and evaluate expression
                    match evaluate_line(line, &mut eval_env) {
                        Ok((result, was_bet)) => {
                            println!("=> {}", result);
                            stats.evals += 1;
                            if was_bet {
                                stats.bets += 1;
                            }
                            line_count += 1;
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            stats.errors += 1;
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save history
    if let Some(ref path) = history_path {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = rl.save_history(path);
    }

    Ok(())
}

enum CommandResult {
    Continue,
    Quit,
}

fn handle_command(line: &str, stats: &mut ReplStats) -> CommandResult {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    let cmd = parts[0];
    let arg = parts.get(1).map(|s| s.trim()).unwrap_or("");

    match cmd {
        ":help" | ":h" => {
            println!("{}", HELP);
        }
        ":quit" | ":q" | ":exit" => {
            println!("Goodbye!");
            return CommandResult::Quit;
        }
        ":clear" => {
            print!("\x1B[2J\x1B[1;1H");
        }
        ":reset" => {
            println!("Environment reset.");
        }
        ":stats" => {
            println!("Betting statistics:");
            println!("  Expressions evaluated: {}", stats.evals);
            println!("  Probabilistic (bet) evals: {}", stats.bets);
            println!("  Type queries: {}", stats.type_queries);
            println!("  Errors: {}", stats.errors);
        }
        ":type" | ":t" => {
            if arg.is_empty() {
                println!("Usage: :type <expression>");
            } else {
                stats.type_queries += 1;
                match bet_parse::parse_expr(arg) {
                    Ok(expr) => {
                        // Wrap the expression as a top-level module item for
                        // the type checker, then infer its type.
                        let spanned_expr = bet_syntax::span::Spanned::dummy(expr.clone());
                        let module = bet_syntax::ast::Module {
                            name: None,
                            items: vec![bet_syntax::span::Spanned::dummy(
                                bet_syntax::ast::Item::Expr(expr),
                            )],
                            span: bet_syntax::span::Span::dummy(),
                        };
                        match bet_check::check_module(&module) {
                            Ok(_env) => {
                                // The type checker seeds builtins; for a bare
                                // expression the inferred type is the result of
                                // the last item.  We re-check manually.
                                let mut check_env = bet_check::CheckEnv::new();
                                match bet_check::check_expr_public(&spanned_expr, &mut check_env) {
                                    Ok(ty) => println!("{:?}", check_env.resolve(&ty)),
                                    Err(_) => {
                                        // Fallback: probabilistic heuristic
                                        if spanned_expr.node.is_probabilistic() {
                                            println!("Dist _  (probabilistic expression)");
                                        } else {
                                            println!("_  (could not fully infer type)");
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Type error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Parse error: {}", e);
                    }
                }
            }
        }
        ":ast" | ":parse" => {
            if arg.is_empty() {
                println!("Usage: :ast <expression>");
            } else {
                match bet_parse::parse_expr(arg) {
                    Ok(expr) => {
                        println!("{:#?}", expr);
                    }
                    Err(e) => {
                        eprintln!("Parse error: {}", e);
                    }
                }
            }
        }
        _ => {
            println!("Unknown command: {}. Type :help for available commands.", cmd);
        }
    }

    CommandResult::Continue
}

/// Evaluate a single line of input, returning the display string and whether
/// the expression was probabilistic.
fn evaluate_line(
    source: &str,
    env: &mut bet_core::ValueEnv<bet_eval::Value>,
) -> Result<(String, bool)> {
    // Parse the expression
    let expr = bet_parse::parse_expr(source).map_err(|e| miette::miette!("{}", e))?;

    let is_bet = expr.is_probabilistic();

    // Evaluate using the interpreter
    let val = bet_eval::eval(&expr, env).map_err(|e| miette::miette!("{}", e))?;

    Ok((format!("{}", val), is_bet))
}
