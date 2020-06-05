use std::collections::HashMap;

use super::object::Object;
use crate::parser::node::Node;
use crate::lexer::token::Token;

pub fn interpret(ast: Node, scope: &mut HashMap<String, Object>) -> Object {
    match ast {
        Node::Program(nodes) => {
            let mut rtn = Object::None;
            println!("interpret, {:?}", nodes);
            for node in nodes {
                rtn = interpret(node, scope);
            }
            rtn
        },
        Node::Addition(lhs, rhs) => add(interpret(*lhs, scope), interpret(*rhs, scope)),
        Node::Number(number_string) => {
            if number_string.contains('.') {
                Object::Float(number_string.parse::<f64>().unwrap())
            } else {
                Object::Integer(number_string.parse::<i64>().unwrap())
            }
        }
        Node::Identifier(literal_string) => {
            scope.get(&literal_string).unwrap_or(&Object::None).to_owned()
        }
        Node::Assignment(lhs, rhs) => {
            if let Node::Identifier(variable_name) = *lhs {
                let value = interpret(*rhs, scope);
                scope.insert(variable_name, value);
                println!("{:?}", scope);
                Object::None
            } else {
                Object::None
            }
        }
        _ => Object::None,
    }
}

fn add(lhs: Object, rhs: Object) -> Object {
    match lhs {
        Object::Integer(lhs_value) => {
            match rhs {
                Object::Integer(rhs_value) => Object::Integer(lhs_value + rhs_value),
                Object::Float(rhs_value) => Object::Float(lhs_value as f64 + rhs_value),
                _ => Object::None,
            }
        }
        Object::Float(lhs_value) => {
            match rhs {
                Object::Integer(rhs_value) => Object::Float(lhs_value + rhs_value as f64),
                Object::Float(rhs_value) => Object::Float(lhs_value + rhs_value),
                _ => Object::None,
            }
        }
        _ => Object::None
    }
}
