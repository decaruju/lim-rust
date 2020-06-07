use crate::lexer::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Program(Vec<Node>),
    Assignment(Box<Node>, Box<Node>),
    Addition(Box<Node>, Box<Node>),
    Multiplication(Box<Node>, Box<Node>),
    Number(String),
    Identifier(String),
    Call(Box<Node>, Vec<Node>),
    PartialCall(Box<Node>, Vec<Node>),
    Empty,
}

impl Node {
    pub fn start_of(token: Token) -> Option<Node> {
        match token {
            Token::Identifier(string) => Some(Node::Identifier(string)),
            Token::Number(string) => Some(Node::Number(string)),
            _ => None,
        }
    }

    pub fn continues(&self, token: &Token) -> Option<bool> {
        match self {
            Node::Identifier(_identifier) => Some(*token == Token::Equal || *token == Token::Plus || *token == Token::OpenParenthesis),
            Node::Number(_identifier) => Some(*token == Token::Plus || *token == Token::Times),
            Node::Assignment(_lhs, rhs) => rhs.continues(token),
            Node::Call(_lhs, rhs) => {
                Some(false)
            }
            Node::PartialCall(_lhs, rhs) => {
                match token {
                    Token::Comma => Some(true),
                    Token::CloseParenthesis => Some(true),
                    _ => {
                        if rhs.len() == 0 {
                            Some(true)
                        } else {
                            rhs.last().unwrap().continues(token)
                        }
                    }
                }
            }
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
                Token::Plus => {
                    *self = Node::Addition(
                        Box::new(Node::Identifier(identifier.clone())),
                        Box::new(Node::Empty),
                    );
                }
                Token::OpenParenthesis => {
                    *self = Node::PartialCall(
                        Box::new(Node::Identifier(identifier.clone())),
                        vec![],
                    );
                }
                _ => {}
            },
            Node::PartialCall(callee, args) => {
                match token {
                    Token::CloseParenthesis => {
                        if let Some(last_arg) = args.last_mut() {
                            if last_arg.continues(&token).unwrap() {
                                last_arg.append(token)
                            } else {
                                *self = Node::Call(callee.to_owned(), args.to_vec())
                            }
                        } else {
                            *self = Node::Call(callee.to_owned(), args.to_vec())
                        }
                    },
                    Token::Comma => {
                        let mut last_arg = args.last_mut().unwrap();
                        if last_arg.continues(&token).unwrap() {
                            last_arg.append(token)
                        } else {
                            args.push(Node::Empty);
                        }
                    }
                    _ => {
                        if args.len() == 0 {
                            args.push(Node::Empty);
                        }
                        args.last_mut().unwrap().append(token);
                    }
                }
            }
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
