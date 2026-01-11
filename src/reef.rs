use std::io::{self, Write};

use crate::error::ReefError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub struct Reef {
    had_error: bool,
    had_runtime_error: bool,
}
impl Reef {
    pub fn new() -> Self {
        Reef {
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn run(&mut self, text: &str) {
        let mut scanner = Scanner::new(text.to_string());
        let tokens = scanner.scan_tokens();
        scanner.print_info();
        let mut parser = Parser::new(tokens);

        let expressions = parser.parse();
        println!("expressions: {:#?}", expressions);
        let interpreter = Interpreter::new();
        for expression in expressions {
            let expr_val = match expression {
                Ok(expr) => interpreter.interpret(&expr),
                Err(e) => Err(e),
            };
            match expr_val {
                Ok(_) => {}
                Err(e) => self.report_error(&e),
            }
        }
        if self.had_error {
            std::process::exit(65)
        }
        if self.had_runtime_error {
            std::process::exit(70)
        }
    }
    pub fn run_repl(&mut self) -> io::Result<()> {
        println!("Starting REPL...");
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut input_text = String::new();
            io::stdin().read_line(&mut input_text)?;
            if input_text.trim() == "exit" {
                break;
            }
            self.run(&input_text);
        }
        Ok(())
    }
    fn report_error(&mut self, error: &ReefError) {
        eprintln!("{:?}", error);
        match error {
            ReefError::ParseError { .. } => self.had_error = true,
            ReefError::RuntimeError { .. } => self.had_runtime_error = true,
        }
    }
}
