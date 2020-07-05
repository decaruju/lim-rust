#![feature(get_mut_unchecked)]

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod interpreter;
mod lexer;
mod parser;

use interpreter::interpreter::interpret;
use lexer::lexer::lex;
use parser::parser::parse;
use std::rc::Rc;

fn run(string: &str) -> interpreter::object::Object {
    (*Rc::clone(&interpret(parse(lex(string).unwrap()).unwrap(), &mut HashMap::new()))).clone()
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
