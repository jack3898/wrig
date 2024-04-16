use std::process;

use crate::components::Scanner;

pub struct Program {
    pub had_error: bool,
}

impl Program {
    fn report(&mut self, message: String) {
        println!("{message}");

        self.had_error = true;
    }

    pub fn run(&mut self, source: &str) {
        // First we scan the source for its distinct tokens
        let mut scanner = Scanner::new(source);

        let (tokens, errors) = scanner.scan_tokens();

        for error in errors {
            self.report(error.to_string());
        }

        println!("{:?}", tokens);

        self.exit();
    }

    fn exit(&self) {
        if self.had_error {
            process::exit(65)
        }

        process::exit(0);
    }
}

impl Default for Program {
    fn default() -> Self {
        Self { had_error: false }
    }
}
