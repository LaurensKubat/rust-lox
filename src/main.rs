extern crate core;

mod interpreter;
mod lox;
mod scanner;
mod token;
mod tokentype;
mod ast;

use crate::lox::Lox;
use std::env;
use std::fs::File;
use std::io::{stdout, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world! {:?}", args);

    if args.len() > 1 {
        println!("usage: lox-rust [script]");
        println!("{:}", args[1]);
    } else if args.len() == 1 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let mut lox = Lox::new();
    lox.run(&contents);
    if lox.had_error {
        std::process::exit(65);
    }
}

fn run_prompt() {
    let mut lox = Lox::new();
    loop {
        print!(">> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        lox.run(&input);
        if lox.had_error {
            lox.had_error = false;
        }
    }
}
