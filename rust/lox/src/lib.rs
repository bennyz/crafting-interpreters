use anyhow::{anyhow, Result};
use scanner::Scanner;
use std::{
    fs::File,
    io::{self, Read, Write},
    process::exit,
};

mod scanner;
mod token;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            let bytes = io::stdin().read_line(&mut line).unwrap();
            if bytes == 0 {
                break;
            }

            self.run(line)?;
            self.had_error = false;
        }

        Ok(())
    }

    pub fn run_file(&self, path: String) -> Result<()> {
        let mut f = File::open(&path)?;
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf)?;
        self.run(String::from_utf8_lossy(&buf).to_string())?;

        if self.had_error {
            exit(65);
        }

        Ok(())
    }

    pub fn error(&mut self, line: u32, message: String) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u32, where_in_line: &str, message: String) {
        println!("[line {}] Error {}: {}", line, where_in_line, message);
        self.had_error = true;
    }

    fn run(&self, source: String) -> Result<()> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        for token in tokens {
            println!("{:#?}", token);
        }

        Ok(())
    }
}
