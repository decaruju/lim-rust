use crate::lexer::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Program(Vec<Node>),
    Assignment(Box<Node>, Box<Node>),
    Addition(Box<Node>, Box<Node>),
    Multiplication(Box<Node>, Box<Node>),
    Number(Token),
    Identifier(Token),
    Empty,
}

impl Node {
    pub fn start_of(token: Token) -> Option<Node> {
        match token {
            Token::Identifier(_) => Some(Node::Identifier(token)),
            Token::Number(_) => Some(Node::Number(token)),
            _ => None,
        }
    }

    pub fn continues(&self, token: &Token) -> Option<bool> {
        match self {
            Node::Identifier(_identifier) => Some(*token == Token::Equal),
            Node::Number(_identifier) => Some(*token == Token::Plus || *token == Token::Times),
            Node::Assignment(_lhs, rhs) => rhs.continues(token),
            Node::Addition(_lhs, rhs) => rhs.continues(token),
            Node::Multiplication(_lhs, rhs) => rhs.continues(token),
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
                }
                _ => {}
            },
            Node::Number(_number) => match token {
                Token::Plus => {
                    *self = Node::Addition(Box::new(self.clone()), Box::new(Node::Empty));
                }
                Token::Times => {
                    *self = Node::Multiplication(Box::new(self.clone()), Box::new(Node::Empty));
                }
                _ => {}
            },
            Node::Assignment(_lhs, rhs) => {
                rhs.append(token);
            }
            Node::Addition(_lhs, rhs) => match token {
                Token::Plus => {
                    *self = Node::Addition(Box::new(self.clone()), Box::new(Node::Empty))
                }
                _ => {
                    rhs.append(token);
                }
            },
            Node::Multiplication(_lhs, rhs) => match token {
                Token::Times => {
                    *self = Node::Multiplication(Box::new(self.clone()), Box::new(Node::Empty))
                }
                Token::Plus => {
                    *self = Node::Addition(Box::new(self.clone()), Box::new(Node::Empty))
                }
                _ => {
                    rhs.append(token);
                }
            },
            Node::Empty => {
                *self = Node::start_of(token).unwrap();
            }
            _ => {}
        }
    }
}
