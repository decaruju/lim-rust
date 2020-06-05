use std::collections::HashMap;

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
    println!("{:?}", run("x = 2; x+2"));
}
