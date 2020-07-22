#![feature(get_mut_unchecked)]
use std::fs::File;
use std::io::prelude::*;

mod interpreter;

use interpreter::interpret;
use lexer::lexer::lex;
use parser::parser::parse;

fn run(string: &str) {
    interpret(parse(lex(string).unwrap()).unwrap())
}

fn main() {
    let mut file = File::open("../../test.lim").unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    run(&code);
}
