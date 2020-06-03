use crate::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Program(Vec<Node>),
    Assignment(Box<Node>, Box<Node>),
    Number(Token),
    Identifier(Token),
    Empty,
}

impl Node {
    pub fn start_of(token: Token) -> Option<Node> {
        match token {
            Token::Identifier(_) => Some(Node::Identifier(token)),
            Token::Number(_) => Some(Node::Number(token)),
            _ => None
        }
    }

    pub fn continues(&self, token: &Token) -> Option<bool> {
        match self {
            Node::Identifier(_identifier) => Some(*token == Token::Equal),
            Node::Assignment(_lhs, rhs) => {
                rhs.continues(token)
            }
            Node::Empty => Some(true),
            _ => Some(false),
        }
    }

    pub fn append(&mut self, token: Token) {
        match self {
            Node::Identifier(identifier) => match token {
                Token::Equal => {
                    *self = Node::Assignment(
                        Box::new(Node::Identifier(identifier.clone())),
                        Box::new(Node::Empty),
                    );
                },
                _ => {},
            },
            Node::Assignment(_lhs, rhs) => {
                rhs.append(token);
            }
            Node::Empty => {
                *self = Node::start_of(token).unwrap();
            }
            _ => {},
        }
    }
}
