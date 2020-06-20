use crate::parser::node::Node;

#[derive(Debug, Clone)]
pub enum Object {
    None,
    Integer(i64),
    Float(f64),
    String(String),
    Native(String),
    Enum(String, Vec<Node>),
    Function(Vec<Node>, Vec<Node>),
}
