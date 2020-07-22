extern crate libloading;

use std::collections::HashMap;

use object::{Object, Native};
use parser::node::Node;
use std::rc::Rc;

pub struct Scope {
    variables: HashMap<String, Rc<Object>>,
}

pub fn interpret(ast: Node) {
    Env::new().interpret(ast, &mut Scope::new());
}

impl Scope {
    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        if let Some(object) = self.variables.get(name) {
            Some(Rc::clone(&object))
        } else {
            None
        }
    }

    pub fn insert(&mut self, name: &str, obj: Rc<Object>) {
        self.variables.insert(name.to_string(), obj);
    }

    pub fn new() -> Scope {
        Scope{
            variables: HashMap::new()
        }
    }
}

pub struct Env {
    natives: Scope,
}

pub fn build_native_class(native: Native) -> Rc<Object> {
    let mut prototype = Object::default();
    prototype.native = Some(native);

    let mut native_class = Object::default();
    native_class.fields.insert(String::from("$prototype"), Rc::new(prototype));

    Rc::new(native_class)
}

pub fn build_native_function(symbol: &str) -> Rc<Object> {
    let lib = String::from("../std/target/release/libstd.so");
    let symbol = symbol.to_string();
    let mut function = Object::default();
    function.native = Some(Native::NativeCode(lib, symbol));

    Rc::new(function)
}

impl Env {
    pub fn new() -> Env {
        let mut env = Env{
            natives: Scope::new(),
        };
        env.build_natives();

        env
    }

    pub fn build_natives(&mut self) {
        self.natives.insert("Float", build_native_class(Native::Float(0.)));
        self.natives.insert("Integer", build_native_class(Native::Integer(0)));
        self.natives.insert("Function", build_native_class(Native::Function(vec![], vec![])));
        self.natives.insert("print", build_native_function("print"));
    }

    pub fn get_obj(&self, name: &str, scope: &Scope) -> Rc<Object> {
        if let Some(obj) = scope.get(name) {
            obj
        } else if let Some(obj) = self.natives.get(name) {
            obj
        } else {
            panic!("Object {:?} does not exist in scope", name)
        }
    }

    pub fn build_integer(&self, integer: i64) -> Rc<Object> {
        let mut prototype = (*self.natives.get("Integer").unwrap().get("$prototype")).clone();
        prototype.native = Some(Native::Integer(integer));
        Rc::new(prototype)
    }

    pub fn build_float(&self, float: f64) -> Rc<Object> {
        let mut prototype = (*self.natives.get("Float").unwrap().get("$prototype")).clone();
        prototype.native = Some(Native::Float(float));
        Rc::new(prototype)
    }

    pub fn build_function(&self, args: Vec<Node>, body: Vec<Node>) -> Rc<Object> {
        let mut prototype = (*self.natives.get("Function").unwrap().get("$prototype")).clone();
        prototype.native = Some(Native::Function(args, body));
        Rc::new(prototype)
    }

    pub fn interpret(&mut self, ast: Node, scope: &mut Scope) -> Rc<Object> {
        match ast {
            Node::Program(nodes) => {
                let mut rtn = Rc::new(Object::default());
                for node in nodes {
                    rtn = self.interpret(node, scope);
                }
                rtn
            }
            Node::Identifier(name) => {
                self.get_obj(&name, scope)
            }
            Node::Assignment(variable, object) => {
                if let Node::Identifier(variable_name) = *variable {
                    let object = self.interpret(*object, scope);
                    scope.insert(&variable_name, object);
                    Rc::new(Object::default())
                } else {
                    panic!("Assignment to non identifer {:?} is not implemented", variable)
                }
            }
            Node::Number(number_string) => {
                if number_string.contains('.') {
                    self.build_float(number_string.parse::<f64>().unwrap())
                } else {
                    self.build_integer(number_string.parse::<i64>().unwrap())
                }
            }
            Node::Addition(lhs, rhs) => {
                let lhs = self.interpret(*lhs, scope);
                let rhs = self.interpret(*rhs, scope);
                if lhs.is_native() && rhs.is_native() {
                    self.native_add(lhs, rhs)
                } else {
                    panic!("Cannot add {:?} to {:?}");
                }
            }
            Node::FunctionDefinition(args, body) => {
                self.build_function(args, body)
            }
            Node::Call(node, args) => {
                let callee = self.interpret(*node, scope);
                let mut arg_objects = vec![];
                for arg in args.iter() {
                    arg_objects.push(self.interpret(arg.to_owned(), scope))
                }
                if callee.is_native() {
                    self.native_call(callee, arg_objects)
                } else {
                    unimplemented!("Cannot call")
                }
                // call(&callee_object, &arg_objects, &callee_object)
            }
            _ => {
                panic!("Interpretation of node {:?} is not implemented", ast)
            }
        }
    }

    fn native_call(&self, callee: Rc<Object>, arg_objects: Vec<Rc<Object>>) -> Rc<Object> {
        if let Some(Native::NativeCode(lib, symbol)) = callee.native.as_ref() {
            call_dynamic(&lib, &symbol, arg_objects)
        } else if let Some(Native::Function(_args, _body)) = callee.native.as_ref() {
            unimplemented!("Cannot call function")
        } else {
            panic!("Cannot native call {:?}", callee);
        }
    }

    fn native_add(&self, lhs: Rc<Object>, rhs: Rc<Object>) -> Rc<Object> {
        let lhs = lhs.native.as_ref().unwrap().to_owned();
        let rhs = rhs.native.as_ref().unwrap().to_owned();

        match lhs {
            Native::Integer(lhs_value) => match rhs {
                Native::Integer(rhs_value) => self.build_integer(lhs_value + rhs_value),
                Native::Float(rhs_value) => self.build_float(lhs_value as f64 + rhs_value),
                _ => panic!("Cannot add {:?} to {:?}", lhs, rhs)
            },
            Native::Float(lhs_value) => match rhs {
                Native::Integer(rhs_value) => self.build_float(lhs_value + rhs_value as f64),
                Native::Float(rhs_value) => self.build_float(lhs_value + rhs_value),
                _ => panic!("Cannot add {:?} to {:?}", lhs, rhs)
            },
            _ => panic!("Cannot add {:?} to {:?}", lhs, rhs)
        }
    }
}

fn call_dynamic(lib: &str, symbol: &str, args: Vec<Rc<Object>>) -> Rc<Object> {
    let lib = libloading::Library::new(lib).unwrap();
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(Vec<Rc<Object>>) -> Rc<Object>> = lib.get(symbol.as_bytes()).unwrap();
        func(args)
    }
}

// fn buildNatives() -> HashMap<String, Rc<Object>> {
//     let mut natives = HashMap::new();
//     natives.insert(
//         String::from("print"),
//         Rc::new(
//             Object::Native(
//                 String::from("../std/target/release/libstd.so"),
//                 String::from("println"),
//             ),
//         ),
//     );
//     natives.insert(
//         String::from("native"),
//         Rc::new(
//             Object::Native(
//                 String::from("../std/target/release/libstd.so"),
//                 String::from("native"),
//             ),
//         ),
//     );

//     natives
// }


// fn build_scope() -> HashMap<String, Rc<Object>> {
//     let mut scope = HashMap::new();
//     let class = Object::class();
//     scope.insert(String::from("Class"), Rc::clone(&class));

//     // let integer = Object::default();
//     // integer.fields.insert(String::from("$class"), Rc::clone(&class));
//     // scope.insert(String::from("IntegerClass"), Rc::new(integer));

//     let mut none_class = Object::default();
//     none_class.fields.insert(String::from("$class"), Rc::clone(&class));
//     let none_class = Rc::new(none_class);
//     scope.insert(String::from("NoneClass"), Rc::clone(&none_class));

//     let mut none = Object::default();
//     none.fields.insert(String::from("$class"), Rc::clone(&none_class));
//     scope.insert(String::from("none"), Rc::new(none));

//     scope
// }

// pub fn interpret(ast: Node) -> Rc<Object> {
    // let mut scope = build_scope();
    // interpret_node(ast, &mut scope)
// }

// pub fn interpret_node(ast: Node, scope: &mut HashMap<String, Rc<Object>>) -> Rc<Object> {
//     match ast {
//         Node::Member(node, string) => get(&interpret_node(*node, scope), string),
        // Node::Program(nodes) => {
        //     for node in nodes {
        //         interpret_node(node, scope);
        //     }
        //     Rc::clone(scope.get("none").unwrap())
        // }
        // _ => Rc::clone(scope.get("none").unwrap()),
//         Node::Literal(string, _delimiter) => Rc::new(Object::String(string)),
//         Node::Match(matched, match_arms) => {
//             let matched = interpret_node(*matched, scope);
//             for arm in match_arms.iter() {
//                 if let Node::MatchArm(matcher, program) = arm {
//                     if eq(&matched, &interpret_node(*matcher.to_owned(), scope)) {
//                         return interpret_node(*program.to_owned(), scope);
//                     }
//                 }
//             }
//             Rc::new(Object::None)
//         }
//         Node::Parenthesized(node) => interpret_node(*node, scope),
//         Node::EnumDefinition(name, variations) => {
//             if let Node::Identifier(name) = *name {
//                 scope.insert(name.clone(), Rc::new(Object::Enum(name.clone(), variations)));
//             }
//             Rc::new(Object::None)
//         }
//         Node::ClassDefinition(name, body) => {
//             if let Node::Identifier(name) = *name {
//                 let mut fields = HashMap::new();
//                 fields.insert(String::from("self"), Rc::new(Object::Instance(HashMap::new())));
//                 interpret_node(*body, &mut fields);
//                 let mut prototype_fields = HashMap::new();
//                 let mut class_fields = HashMap::new();
//                 for (var_name, object) in fields.iter_mut() {
//                     if var_name == "self" {
//                         prototype_fields.insert(String::from("$class"), object.clone());
//                     } else {
//                         prototype_fields.insert(var_name.to_owned(), object.clone());
//                     }
//                 }
//                 class_fields.insert(String::from("$prototype"), Rc::new(Object::Instance(prototype_fields.clone())));
//                 if let Object::Instance(self_fields) = &**fields.get("self").unwrap() {
//                     for (var_name, object) in self_fields.iter() {
//                         class_fields.insert(var_name.to_owned(), object.clone());
//                     }
//                 }

//                 scope.insert(name.clone(), Rc::new(Object::Class(class_fields)));
//             }
//             Rc::new(Object::None)
//         }
//         Node::Addition(lhs, rhs) => add(interpret_node(*lhs, scope), interpret_node(*rhs, scope)),
//         Node::Multiplication(lhs, rhs) => multiply(interpret_node(*lhs, scope), interpret_node(*rhs, scope)),
//         Node::Identifier(literal_string) => scope.get(&literal_string).unwrap_or(natives.get(&literal_string).unwrap_or(&Rc::new(Object::None))).to_owned(),
//         Node::FunctionDefinition(args, body) => Rc::new(Object::Function(args, body)),
//         Node::Call(callee, args) => {
//             let mut callee_object = interpret_node(*callee, scope);
//             let mut arg_objects = vec![];
//             for arg in args.iter() {
//                 arg_objects.push(interpret_node(arg.to_owned(), scope))
//             }
//             call(&callee_object, &arg_objects, &callee_object)
//         }
//         Node::Assignment(lhs, rhs) => {
//             match *lhs {
//                 Node::Identifier(variable_name) => {
//                     let value = interpret_node(*rhs, scope);
//                     scope.insert(variable_name, value);
//                     Rc::new(Object::None)
//                 }
//                 Node::Member(instance, field_name) => {
//                     let mut instance = interpret_node(*instance, scope);
//                     let value = interpret_node(*rhs, scope);
//                     let mut instance = unsafe {
//                         Rc::get_mut_unchecked(&mut instance)
//                     };
//                     match instance {
//                         Object::Class(class_fields) => {
//                             class_fields.insert(field_name, value);
//                         }
//                         Object::Instance(instance_fields) => {
//                             instance_fields.insert(field_name, value);
//                         }
//                         _ => {
//                             unimplemented!("member assignment on {:?}", instance);
//                         }
//                     }
//                     Rc::new(Object::None)
//                 }
//                 _ => unimplemented!("assigment on {:?}", lhs),
//             }
//         }
//         _ => Rc::new(Object::None),
//     }
// }

// fn add(lhs: Rc<Object>, rhs: Rc<Object>) -> Rc<Object> {
//     match &*lhs {
//         Object::Integer(lhs_value) => match *rhs {
//             Object::Integer(rhs_value) => Rc::new(Object::Integer(lhs_value + rhs_value)),
//             Object::Float(rhs_value) => Rc::new(Object::Float(*lhs_value as f64 + rhs_value)),
//             _ => Rc::new(Object::None),
//         },
//         Object::Float(lhs_value) => match *rhs {
//             Object::Integer(rhs_value) => Rc::new(Object::Float(lhs_value + rhs_value as f64)),
//             Object::Float(rhs_value) => Rc::new(Object::Float(lhs_value + rhs_value)),
//             _ => Rc::new(Object::None),
//         },
//         _ => Rc::new(Object::None),
//     }
// }

// fn eq(lhs: &Rc<Object>, rhs: &Rc<Object>) -> bool {
//     match &**lhs {
//         Object::EnumVariant(string_lhs) => match &**rhs {
//             Object::EnumVariant(string_rhs) => string_lhs == string_rhs,
//             _ => false,
//         },
//         _ => false,
//     }
// }

// fn multiply(lhs: Rc<Object>, rhs: Rc<Object>) -> Rc<Object> {
//     match *lhs {
//         Object::Integer(lhs_value) => match *rhs {
//             Object::Integer(rhs_value) => Rc::new(Object::Integer(lhs_value * rhs_value)),
//             Object::Float(rhs_value) => Rc::new(Object::Float(lhs_value as f64 * rhs_value)),
//             _ => Rc::new(Object::None),
//         },
//         Object::Float(lhs_value) => match *rhs {
//             Object::Integer(rhs_value) => Rc::new(Object::Float(lhs_value * rhs_value as f64)),
//             Object::Float(rhs_value) => Rc::new(Object::Float(lhs_value * rhs_value)),
//             _ => Rc::new(Object::None),
//         },
//         _ => Rc::new(Object::None),
//     }
// }


// fn get(obj: &Rc<Object>, string: String) -> Rc<Object> {
//     match &**obj {
//         Object::Enum(name, variants) => {
//             for variant in variants.iter() {
//                 if let Node::Identifier(variant_name) = variant {
//                     if *variant_name == string {
//                         return Rc::new(Object::EnumVariant(format!("{}.{}", name, variant_name)));
//                     }
//                 } else {
//                     unimplemented!();
//                 }
//             }
//             Rc::new(Object::Error(format!("Variant {} does not exist for enum {}", string, name)))
//         }
//         Object::Instance(fields) => {
//             let field = fields.get(&string).unwrap().to_owned();
//             if let Object::Function(_, _) = *field {
//                 Rc::new(Object::BoundFunction(obj.to_owned(), field))
//             } else {
//                 field
//             }
//         }
//         Object::Class(fields) => {
//             let field = fields.get(&string).unwrap().to_owned();
//             if let Object::Function(_, _) = *field {
//                 Rc::new(Object::BoundFunction(obj.to_owned(), field))
//             } else {
//                 field
//             }
//         }
//         _ => unimplemented!("cannot get {:?} on {:?}", string, obj),
//     }
// }

// fn call(callee: &Rc<Object>, args: &Vec<Rc<Object>>, instance: &Rc<Object>) -> Rc<Object> {
//     match &**callee {
//         Object::Native(lib, symbol) => {
//             call_dynamic(lib, symbol, args.to_vec())
//         }
//         Object::Class(fields) => {
//             fields.get("$prototype").unwrap().to_owned()
//         }
//         Object::Function(argument_names, body) => {
//             let mut rtn = Rc::new(Object::None);
//             let mut scope = HashMap::new();

//             for (index, argument_name) in argument_names.iter().enumerate() {
//                 if let Node::Identifier(string) = argument_name {
//                     if let Some(arg) = args.get(index) {
//                         scope.insert(string.to_string(), arg.to_owned());
//                     } else {
//                         scope.insert(string.to_string(), Rc::new(Object::None));
//                     }
//                 } else {
//                     unreachable!();
//                 }
//             }

//             scope.insert("self".to_string(), instance.to_owned());
//             for node in body.iter() {
//                 rtn = interpret_node(node.to_owned(), &mut scope);
//             }
//             rtn
//         }
//         Object::BoundFunction(instance, function) => {
//             call(&function, args, &instance)
//         }
//         _ => unimplemented!(),
//     }
// }
