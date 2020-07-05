use crate::parser::node::Node;
use std::collections::HashMap;
use std::rc::Rc;

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
    BoundFunction(Rc<Object>, Rc<Object>),
    Class(HashMap<String, Rc<Object>>),
    Instance(HashMap<String, Rc<Object>>),
}
