use std::{env, process::exit};

use lox;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut lox = lox::Lox::new();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 1 {
        lox.run_file(args[0].clone()).unwrap();
    } else {
        lox.run_prompt().unwrap();
    }
}
