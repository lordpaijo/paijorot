mod token;
mod lexer;
mod parser;
mod interpreter;
mod environment;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: paijorot [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(content) => {
            if let Err(e) = run(content) {
                eprintln!("Runtime error: {}", e);
                process::exit(70);
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(66);
        }
    }
}

fn run_prompt() {
    let mut environment = environment::Environment::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        if stdin.read_line(&mut line).is_err() || line.trim() == "exit" {
            break;
        }

        match run_with_env(line, &mut environment) {
            Ok(_) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn run(source: String) -> Result<(), String> {
    let mut environment = environment::Environment::new();
    run_with_env(source, &mut environment)
}

fn run_with_env(source: String, environment: &mut environment::Environment) -> Result<(), String> {
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.scan_tokens()?;

    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse()?;

    let mut interpreter = interpreter::Interpreter::new(environment);
    interpreter.interpret(statements)
}
