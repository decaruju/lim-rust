use crate::parser::node::Node;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Object {
    None,
    Integer(i64),
    Float(f64),
    Error(String),
    String(String),
    Native(String),
    Enum(String, Vec<Node>),
    EnumVariant(String),
    Function(Vec<Node>, Vec<Node>),
    BoundFunction(Box<Object>, Box<Object>),
    Class(HashMap<String, Box<Object>>),
    Instance(HashMap<String, Box<Object>>),
}
