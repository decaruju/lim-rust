use std::collections::HashMap;

use super::object::Object;
use crate::parser::node::Node;

pub fn interpret(ast: Node, scope: &mut HashMap<String, Object>) -> Object {
    let mut natives: HashMap<String, Object> = HashMap::new();
    natives.insert(String::from("print"), Object::Native(String::from("print")));
    match ast {
        Node::Program(nodes) => {
            let mut rtn = Object::None;
            for node in nodes {
                rtn = interpret(node, scope);
            }
            rtn
        }
        Node::Parenthesized(node) => {
            interpret(*node, scope)
        }
        Node::Addition(lhs, rhs) => add(interpret(*lhs, scope), interpret(*rhs, scope)),
        Node::Multiplication(lhs, rhs) => multiply(interpret(*lhs, scope), interpret(*rhs, scope)),
        Node::Number(number_string) => {
            if number_string.contains('.') {
                Object::Float(number_string.parse::<f64>().unwrap())
            } else {
                Object::Integer(number_string.parse::<i64>().unwrap())
            }
        }
        Node::Identifier(literal_string) => scope.get(&literal_string).unwrap_or(natives.get(&literal_string).unwrap_or(&Object::None)).to_owned(),
        Node::FunctionDefinition(_args, body) => Object::Function(body),
        Node::Call(callee, args) => {
            let mut callee_object = interpret(*callee, scope);
            let mut arg_objects = args.iter().map(|arg| interpret(arg.to_owned(), scope)).collect();
            call(&callee_object, arg_objects)
        }
        Node::Assignment(lhs, rhs) => {
            if let Node::Identifier(variable_name) = *lhs {
                let value = interpret(*rhs, scope);
                scope.insert(variable_name, value);
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
        Object::Integer(lhs_value) => match rhs {
            Object::Integer(rhs_value) => Object::Integer(lhs_value + rhs_value),
            Object::Float(rhs_value) => Object::Float(lhs_value as f64 + rhs_value),
            _ => Object::None,
        },
        Object::Float(lhs_value) => match rhs {
            Object::Integer(rhs_value) => Object::Float(lhs_value + rhs_value as f64),
            Object::Float(rhs_value) => Object::Float(lhs_value + rhs_value),
            _ => Object::None,
        },
        _ => Object::None,
    }
}

fn multiply(lhs: Object, rhs: Object) -> Object {
    match lhs {
        Object::Integer(lhs_value) => match rhs {
            Object::Integer(rhs_value) => Object::Integer(lhs_value * rhs_value),
            Object::Float(rhs_value) => Object::Float(lhs_value as f64 * rhs_value),
            _ => Object::None,
        },
        Object::Float(lhs_value) => match rhs {
            Object::Integer(rhs_value) => Object::Float(lhs_value * rhs_value as f64),
            Object::Float(rhs_value) => Object::Float(lhs_value * rhs_value),
            _ => Object::None,
        },
        _ => Object::None,
    }
}

fn to_string(obj: &Object) -> String {
    match &*obj {
        Object::Integer(number) => format!("{}", number),
        Object::Float(number) => format!("{}", number),
        Object::Function(_body) => format!("{:?}", obj),
        _ => "".to_string(),
    }
}

fn call(callee: &Object, args: Vec<Object>) -> Object {
    match callee {
        Object::Native(identifier) => {
            if identifier == "print" {
                for arg in args.iter() {
                    println!("{}", to_string(&arg));
                }
            }
            Object::None
        }
        Object::Function(body) => {
            let mut rtn = Object::None;
            let mut scope = HashMap::new();
            scope.insert("self".to_string(), callee.clone());
            for node in body.iter() {
                rtn = interpret(node.to_owned(), &mut scope);
            }
            rtn
        }
        _ => Object::None,
    }
}
