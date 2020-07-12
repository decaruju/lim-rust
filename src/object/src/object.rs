use parser::node::Node;
use std::collections::HashMap;
use std::rc::Rc;
extern crate libloading;

#[derive(Debug, Clone)]
pub enum Object {
    None,
    Integer(i64),
    Float(f64),
    Error(String),
    String(String),
    Native(String, String),
    Enum(String, Vec<Node>),
    EnumVariant(String),
    Function(Vec<Node>, Vec<Node>),
    BoundFunction(Rc<Object>, Rc<Object>),
    Class(HashMap<String, Rc<Object>>),
    Instance(HashMap<String, Rc<Object>>),
}

impl Object {
    pub fn to_string(&self) -> String {
        match self {
            Object::Integer(number) => format!("{}", number),
            Object::String(string) => format!("{}", string),
            Object::Float(number) => format!("{}", number),
            Object::Enum(name, variations) => format!("enum {:?}, variations {:?}", name, variations),
            Object::None => String::from("None"),
            Object::Function(_args, _body) => format!("{:?}", self),
            _ => format!("{:?}", self),
        }
    }
}
