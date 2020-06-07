use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod lexer;
mod parser;
mod interpreter;

fn run(string: &str) -> interpreter::object::Object {
    interpreter::interpreter::interpret(
        parser::parser::parse(
            lexer::lexer::lex(string).unwrap()
        ).unwrap(),
        &mut HashMap::new(),
    )
}

fn main() {
    let mut file = File::open("test.lim").unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    run(&code);
}
