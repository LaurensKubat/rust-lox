use crate::interpreter::Interpreter;
use crate::scanner::Scanner;

pub struct Lox {
    pub interpreter: Interpreter,
    pub had_error: bool,
    pub had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            interpreter: Interpreter::new(),
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source);
        let _tokens = scanner.scan_tokens();
        // let parser = Parser::new(tokens);
        // let statements = parser.parse();
        //
        // self.interpreter.interpret(statements);
    }

    // TODO expand error() to also report the offending character for Rust like error reporting
    pub(crate) fn error(&mut self, line: usize, message: &str) {
        self.had_error = true;
        println!("[line {}] Error: {}", line, message);
    }
}
