use std::collections::HashMap;

use super::object::Object;
use crate::parser::node::Node;

pub fn interpret(ast: Node, scope: &mut HashMap<String, Object>) -> Object {
    let mut natives: HashMap<String, Object> = HashMap::new();
    natives.insert(String::from("print"), Object::Native(String::from("print")));
    match ast {
        Node::Member(node, string) => get(&interpret(*node, scope), string),
        Node::Program(nodes) => {
            let mut rtn = Object::None;
            for node in nodes {
                rtn = interpret(node, scope);
            }
            rtn
        }
        Node::Literal(string, _delimiter) => Object::String(string),
        Node::Match(matched, match_arms) => {
            let matched = interpret(*matched, scope);
            for arm in match_arms.iter() {
                if let Node::MatchArm(matcher, program) = arm {
                    if eq(&matched, &interpret(*matcher.to_owned(), scope)) {
                        return interpret(*program.to_owned(), scope);
                    }
                }
            }
            Object::None
        }
        Node::Parenthesized(node) => interpret(*node, scope),
        Node::EnumDefinition(name, variations) => {
            if let Node::Identifier(name) = *name {
                scope.insert(name.clone(), Object::Enum(name.clone(), variations));
            }
            Object::None
        }
        Node::ClassDefinition(name, body) => {
            if let Node::Identifier(name) = *name {
                let mut fields = HashMap::new();
                interpret(*body, &mut fields);
                let mut prototype_fields = HashMap::new();
                let mut class_fields = HashMap::new();
                for (var_name, object) in fields.iter_mut() {
                    if var_name == "self" {
                        prototype_fields.insert(String::from("$class"), Box::new(object.clone()));
                    } else {
                        prototype_fields.insert(var_name.to_owned(), Box::new(object.clone()));
                    }
                }
                class_fields.insert(String::from("$prototype"), Box::new(Object::Instance(prototype_fields)));

                scope.insert(name.clone(), Object::Class(class_fields));
            }
            Object::None
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
        Node::FunctionDefinition(args, body) => Object::Function(args, body),
        Node::Call(callee, args) => {
            let mut callee_object = interpret(*callee, scope);
            let mut arg_objects = args.iter().map(|arg| interpret(arg.to_owned(), scope)).collect();
            call(&callee_object, arg_objects, &callee_object)
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

fn eq(lhs: &Object, rhs: &Object) -> bool {
    match lhs {
        Object::EnumVariant(string_lhs) => match rhs {
            Object::EnumVariant(string_rhs) => string_lhs == string_rhs,
            _ => false,
        },
        _ => false,
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
        Object::String(string) => format!("{}", string),
        Object::Float(number) => format!("{}", number),
        Object::Enum(name, variations) => format!("enum {:?}, variations {:?}", name, variations),
        Object::None => String::from("None"),
        Object::Function(_args, _body) => format!("{:?}", obj),
        _ => format!("{:?}", obj),
    }
}

fn get(obj: &Object, string: String) -> Object {
    match obj {
        Object::Enum(name, variants) => {
            for variant in variants.iter() {
                if let Node::Identifier(variant_name) = variant {
                    if *variant_name == string {
                        return Object::EnumVariant(format!("{}.{}", name, variant_name));
                    }
                } else {
                    unimplemented!();
                }
            }
            Object::Error(format!("Variant {} does not exist for enum {}", string, name))
        }
        Object::Instance(fields) => {
            let field = *fields.get(&string).unwrap().to_owned();
            if let Object::Function(_, _) = field {
                Object::BoundFunction(Box::new(obj.to_owned()), Box::new(field))
            } else {
                field
            }
        }
        _ => unimplemented!(),
    }
}

fn call(callee: &Object, args: Vec<Object>, instance: &Object) -> Object {
    match callee {
        Object::Native(identifier) => {
            if identifier == "print" {
                for arg in args.iter() {
                    println!("{}", to_string(&arg));
                }
            }
            Object::None
        }
        Object::Class(fields) => {
            *fields.get("$prototype").unwrap().clone()
        }
        Object::Function(argument_names, body) => {
            let mut rtn = Object::None;
            let mut scope = HashMap::new();

            for (index, argument_name) in argument_names.iter().enumerate() {
                if let Node::Identifier(string) = argument_name {
                    if let Some(arg) = args.get(index) {
                        scope.insert(string.to_string(), arg.to_owned());
                    } else {
                        scope.insert(string.to_string(), Object::None);
                    }
                } else {
                    unreachable!();
                }
            }

            scope.insert("self".to_string(), instance.to_owned());
            for node in body.iter() {
                rtn = interpret(node.to_owned(), &mut scope);
            }
            rtn
        }
        Object::BoundFunction(instance, function) => {
            call(function, args, &**instance)
        }
        _ => unimplemented!(),
    }
}
