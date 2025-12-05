//! REPL (Read-Eval-Print Loop) for kakei_lisp
//!
//! This binary provides an interactive shell for evaluating Lisp expressions.
//! Users can type expressions, see the results immediately, and experiment
//! with the language.

use clap::Parser;
use kakei_lisp::{Value, create_global_env, eval, parse};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let _args: CLIArgs = CLIArgs::parse();

    println!("kakei_lisp REPL v{}", env!("CARGO_PKG_VERSION"));
    println!("Type expressions to evaluate. Press Ctrl+C or Ctrl+D to exit.");
    println!();

    // Create readline editor for line editing and history
    let mut rl = DefaultEditor::new()?;

    // Create the global environment with built-in functions
    let mut env = create_global_env();

    loop {
        let readline = rl.readline("klisp> ");
        match readline {
            Ok(line) => {
                // Skip empty lines
                if line.trim().is_empty() {
                    continue;
                }

                // Add to history
                let _ = rl.add_history_entry(line.as_str());

                // Parse and evaluate the input
                match parse(&line) {
                    Ok((_, sexprs)) => {
                        // Evaluate each expression and print the last result
                        let mut result = Value::Nil;
                        let is_empty = sexprs.is_empty();
                        for sexpr in sexprs {
                            match eval(&sexpr, &mut env) {
                                Ok(val) => result = val,
                                Err(e) => {
                                    eprintln!("Evaluation error: {:?}", e);
                                    break;
                                }
                            }
                        }
                        // Print the result of the last expression
                        if !matches!(result, Value::Nil) || !is_empty {
                            println!("{}", result);
                        }
                    }
                    Err(e) => {
                        eprintln!("Parse error: {:?}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

#[derive(Parser)]
#[command(version, about, author)]
struct CLIArgs;
