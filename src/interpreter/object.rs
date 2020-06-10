use crate::parser::node::Node;

#[derive(Debug, Clone)]
pub enum Object {
    None,
    Integer(i64),
    Float(f64),
    String(String),
    Native(String),
    Function(Vec<Node>, Vec<Node>),
}
