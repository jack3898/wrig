use std::process;

use crate::components::Scanner;

pub struct Application {
    had_error: bool,
}

impl Application {
    fn report(&mut self, line: i32, where_msg: &str, message: &str) {
        println!("[line {line}] Error{where_msg}: {message}");

        self.had_error = true;
    }

    pub fn run(&self, source: &str) {
        // First we scan the source for its distinct tokens
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();

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

impl Default for Application {
    fn default() -> Self {
        Self { had_error: false }
    }
}
