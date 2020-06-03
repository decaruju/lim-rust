use crate::lexer::token::Token;
use crate::parser::node::Node;

pub fn parse(mut tokens: Vec<Token>) -> Option<Node> {
    let mut tree = Node::Program(vec![]);
    while !tokens.is_empty() {
        let token = tokens.remove(0);
        if let Some(mut node) = Node::start_of(token) {
            while !tokens.is_empty() && node.continues(&tokens[0])? {
                node.append(tokens.remove(0));
            }
            if let Node::Program(nodes) = &mut tree {
                nodes.push(node);
            }
        }
    }
    Some(tree)
}
