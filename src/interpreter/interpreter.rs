extern crate libloading;

use std::collections::HashMap;

use object::{Object};
use parser::node::Node;
use std::rc::Rc;

fn buildNatives() -> HashMap<String, Rc<Object>> {
    let mut natives = HashMap::new();
    natives.insert(
        String::from("print"),
        Rc::new(
            Object::Native(
                String::from("../std/target/release/libstd.so"),
                b"println",
            ),
        ),
    );

    natives.insert(
        String::from("test"),
        Rc::new(
            Object::Native(
                String::from("../std/target/release/libstd.so"),
                b"test",
            ),
        ),
    );

    natives
}

fn call_dynamic(lib: &str, symbol: &[u8], args: Vec<Rc<Object>>) -> Rc<Object> {
    let lib = libloading::Library::new(lib).unwrap();
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(Vec<Rc<Object>>) -> Rc<Object>> = lib.get(symbol).unwrap();
        func(args)
    }
}

pub fn interpret(ast: Node, scope: &mut HashMap<String, Rc<Object>>) -> Rc<Object> {
    let mut natives = buildNatives();
    match ast {
        Node::Member(node, string) => get(&interpret(*node, scope), string),
        Node::Program(nodes) => {
            for node in nodes {
                interpret(node, scope);
            }
            Rc::new(Object::None)
        }
        Node::Literal(string, _delimiter) => Rc::new(Object::String(string)),
        Node::Match(matched, match_arms) => {
            let matched = interpret(*matched, scope);
            for arm in match_arms.iter() {
                if let Node::MatchArm(matcher, program) = arm {
                    if eq(&matched, &interpret(*matcher.to_owned(), scope)) {
                        return interpret(*program.to_owned(), scope);
                    }
                }
            }
            Rc::new(Object::None)
        }
        Node::Parenthesized(node) => interpret(*node, scope),
        Node::EnumDefinition(name, variations) => {
            if let Node::Identifier(name) = *name {
                scope.insert(name.clone(), Rc::new(Object::Enum(name.clone(), variations)));
            }
            Rc::new(Object::None)
        }
        Node::ClassDefinition(name, body) => {
            if let Node::Identifier(name) = *name {
                let mut fields = HashMap::new();
                fields.insert(String::from("self"), Rc::new(Object::Instance(HashMap::new())));
                interpret(*body, &mut fields);
                let mut prototype_fields = HashMap::new();
                let mut class_fields = HashMap::new();
                for (var_name, object) in fields.iter_mut() {
                    if var_name == "self" {
                        prototype_fields.insert(String::from("$class"), object.clone());
                    } else {
                        prototype_fields.insert(var_name.to_owned(), object.clone());
                    }
                }
                class_fields.insert(String::from("$prototype"), Rc::new(Object::Instance(prototype_fields.clone())));
                if let Object::Instance(self_fields) = &**fields.get("self").unwrap() {
                    for (var_name, object) in self_fields.iter() {
                        class_fields.insert(var_name.to_owned(), object.clone());
                    }
                }

                scope.insert(name.clone(), Rc::new(Object::Class(class_fields)));
            }
            Rc::new(Object::None)
        }
        Node::Addition(lhs, rhs) => add(interpret(*lhs, scope), interpret(*rhs, scope)),
        Node::Multiplication(lhs, rhs) => multiply(interpret(*lhs, scope), interpret(*rhs, scope)),
        Node::Number(number_string) => {
            if number_string.contains('.') {
                Rc::new(Object::Float(number_string.parse::<f64>().unwrap()))
            } else {
                Rc::new(Object::Integer(number_string.parse::<i64>().unwrap()))
            }
        }
        Node::Identifier(literal_string) => scope.get(&literal_string).unwrap_or(natives.get(&literal_string).unwrap_or(&Rc::new(Object::None))).to_owned(),
        Node::FunctionDefinition(args, body) => Rc::new(Object::Function(args, body)),
        Node::Call(callee, args) => {
            let mut callee_object = interpret(*callee, scope);
            let mut arg_objects = vec![];
            for arg in args.iter() {
                arg_objects.push(interpret(arg.to_owned(), scope))
            }
            call(&callee_object, &arg_objects, &callee_object)
        }
        Node::Assignment(lhs, rhs) => {
            match *lhs {
                Node::Identifier(variable_name) => {
                    let value = interpret(*rhs, scope);
                    scope.insert(variable_name, value);
                    Rc::new(Object::None)
                }
                Node::Member(instance, field_name) => {
                    let mut instance = interpret(*instance, scope);
                    let value = interpret(*rhs, scope);
                    let mut instance = unsafe {
                        Rc::get_mut_unchecked(&mut instance)
                    };
                    match instance {
                        Object::Class(class_fields) => {
                            class_fields.insert(field_name, value);
                        }
                        Object::Instance(instance_fields) => {
                            instance_fields.insert(field_name, value);
                        }
                        _ => {
                            unimplemented!("member assignment on {:?}", instance);
                        }
                    }
                    Rc::new(Object::None)
                }
                _ => unimplemented!("assigment on {:?}", lhs),
            }
        }
        _ => Rc::new(Object::None),
    }
}

fn add(lhs: Rc<Object>, rhs: Rc<Object>) -> Rc<Object> {
    match &*lhs {
        Object::Integer(lhs_value) => match *rhs {
            Object::Integer(rhs_value) => Rc::new(Object::Integer(lhs_value + rhs_value)),
            Object::Float(rhs_value) => Rc::new(Object::Float(*lhs_value as f64 + rhs_value)),
            _ => Rc::new(Object::None),
        },
        Object::Float(lhs_value) => match *rhs {
            Object::Integer(rhs_value) => Rc::new(Object::Float(lhs_value + rhs_value as f64)),
            Object::Float(rhs_value) => Rc::new(Object::Float(lhs_value + rhs_value)),
            _ => Rc::new(Object::None),
        },
        _ => Rc::new(Object::None),
    }
}

fn eq(lhs: &Rc<Object>, rhs: &Rc<Object>) -> bool {
    match &**lhs {
        Object::EnumVariant(string_lhs) => match &**rhs {
            Object::EnumVariant(string_rhs) => string_lhs == string_rhs,
            _ => false,
        },
        _ => false,
    }
}

fn multiply(lhs: Rc<Object>, rhs: Rc<Object>) -> Rc<Object> {
    match *lhs {
        Object::Integer(lhs_value) => match *rhs {
            Object::Integer(rhs_value) => Rc::new(Object::Integer(lhs_value * rhs_value)),
            Object::Float(rhs_value) => Rc::new(Object::Float(lhs_value as f64 * rhs_value)),
            _ => Rc::new(Object::None),
        },
        Object::Float(lhs_value) => match *rhs {
            Object::Integer(rhs_value) => Rc::new(Object::Float(lhs_value * rhs_value as f64)),
            Object::Float(rhs_value) => Rc::new(Object::Float(lhs_value * rhs_value)),
            _ => Rc::new(Object::None),
        },
        _ => Rc::new(Object::None),
    }
}


fn get(obj: &Rc<Object>, string: String) -> Rc<Object> {
    match &**obj {
        Object::Enum(name, variants) => {
            for variant in variants.iter() {
                if let Node::Identifier(variant_name) = variant {
                    if *variant_name == string {
                        return Rc::new(Object::EnumVariant(format!("{}.{}", name, variant_name)));
                    }
                } else {
                    unimplemented!();
                }
            }
            Rc::new(Object::Error(format!("Variant {} does not exist for enum {}", string, name)))
        }
        Object::Instance(fields) => {
            let field = fields.get(&string).unwrap().to_owned();
            if let Object::Function(_, _) = *field {
                Rc::new(Object::BoundFunction(obj.to_owned(), field))
            } else {
                field
            }
        }
        Object::Class(fields) => {
            let field = fields.get(&string).unwrap().to_owned();
            if let Object::Function(_, _) = *field {
                Rc::new(Object::BoundFunction(obj.to_owned(), field))
            } else {
                field
            }
        }
        _ => unimplemented!("cannot get {:?} on {:?}", string, obj),
    }
}

fn call(callee: &Rc<Object>, args: &Vec<Rc<Object>>, instance: &Rc<Object>) -> Rc<Object> {
    match &**callee {
        Object::Native(lib, symbol) => {
            call_dynamic(lib, symbol, args.to_vec())
        }
        Object::Class(fields) => {
            fields.get("$prototype").unwrap().to_owned()
        }
        Object::Function(argument_names, body) => {
            let mut rtn = Rc::new(Object::None);
            let mut scope = HashMap::new();

            for (index, argument_name) in argument_names.iter().enumerate() {
                if let Node::Identifier(string) = argument_name {
                    if let Some(arg) = args.get(index) {
                        scope.insert(string.to_string(), arg.to_owned());
                    } else {
                        scope.insert(string.to_string(), Rc::new(Object::None));
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
            call(&function, args, &instance)
        }
        _ => unimplemented!(),
    }
}
