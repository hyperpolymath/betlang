// SPDX-License-Identifier: MIT OR Apache-2.0
//! Interactive REPL for Betlang

use miette::Result;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RlResult};

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
                    match handle_command(line) {
                        CommandResult::Continue => continue,
                        CommandResult::Quit => break,
                    }
                } else {
                    // Parse and evaluate expression
                    match evaluate_line(line) {
                        Ok(result) => {
                            println!("=> {}", result);
                            line_count += 1;
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
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

fn handle_command(line: &str) -> CommandResult {
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
            println!("  (Statistics tracking not yet implemented)");
        }
        ":type" | ":t" => {
            if arg.is_empty() {
                println!("Usage: :type <expression>");
            } else {
                match bet_parse::parse_expr(arg) {
                    Ok(expr) => {
                        // TODO: Type inference
                        if expr.is_probabilistic() {
                            println!("Dist _  (probabilistic expression)");
                        } else {
                            println!("_  (type inference not yet implemented)");
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

fn evaluate_line(source: &str) -> Result<String> {
    // Parse the expression
    let expr = bet_parse::parse_expr(source).map_err(|e| miette::miette!("{}", e))?;

    // TODO: Actual evaluation
    // For now, just print what we parsed

    if expr.is_probabilistic() {
        Ok(format!("<probabilistic: {:?}>", expr))
    } else {
        Ok(format!("<value: {:?}>", expr))
    }
}
