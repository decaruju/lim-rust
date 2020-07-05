use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod interpreter;
mod lexer;
mod parser;

use interpreter::interpreter::interpret;
use lexer::lexer::lex;
use parser::parser::parse;

fn run(string: &str) -> interpreter::object::Object {
    interpret(parse(lex(string).unwrap()).unwrap(), &mut HashMap::new())
}

fn lex_parse(string: &str) -> parser::node::Node {
    parse(lex(string).unwrap()).unwrap()
}

fn main() {
    let mut file = File::open("test.lim").unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    // println!("{:?}", lex_parse(&code));
    run(&code);
}
