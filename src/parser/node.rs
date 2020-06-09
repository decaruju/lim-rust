use crate::lexer::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Program(Vec<Node>),
    Assignment(Box<Node>, Box<Node>),
    Addition(Box<Node>, Box<Node>),
    Substraction(Box<Node>, Box<Node>),
    Multiplication(Box<Node>, Box<Node>),
    Division(Box<Node>, Box<Node>),
    Modulus(Box<Node>, Box<Node>),
    Number(String),
    Identifier(String),
    Call(Box<Node>, Vec<Node>),
    PartialCall(Box<Node>, Vec<Node>),
    Parenthesized(Box<Node>),
    PartialParenthesized(Box<Node>),
    FunctionArgs(Vec<Node>),
    PartialFunctionDefinition(Vec<Node>, Vec<Node>),
    FunctionDefinition(Vec<Node>, Vec<Node>),
    Empty,
}

impl Node {
    pub fn start_of(token: Token) -> Option<Node> {
        match token {
            Token::Identifier(string) => Some(Node::Identifier(string)),
            Token::Number(string) => Some(Node::Number(string)),
            Token::OpenParenthesis => Some(Node::PartialParenthesized(Box::new(Node::Empty))),
            Token::OpenBrace => Some(Node::PartialFunctionDefinition(vec![], vec![Node::Empty])),
            Token::NewLine | Token::SemiColon => Some(Node::Empty),
            _ => None,
        }
    }

    pub fn continues(&self, token: &Token) -> Option<bool> {
        match self {
            Node::Identifier(_identifier) => match token {
                Token::Equal | Token::Plus | Token::Minus | Token::Times | Token::Division | Token::Period | Token::Modulus | Token::OpenParenthesis => Some(true),
                _ => Some(false),
            },
            Node::Number(_identifier) => match token {
                Token::Plus | Token::Minus | Token::Times | Token::Division | Token::Period | Token::Modulus | Token::OpenParenthesis => Some(true),
                _ => Some(false),
            },
            Node::Parenthesized(_identifier) => match token {
                Token::Plus | Token::Minus | Token::Times | Token::Division | Token::Period | Token::Modulus | Token::OpenParenthesis => Some(true),
                _ => Some(false),
            },
            Node::Addition(_lhs, rhs) => rhs.continues(token),
            Node::Substraction(_lhs, rhs) => rhs.continues(token),
            Node::Multiplication(_lhs, rhs) => rhs.continues(token),
            Node::Division(_lhs, rhs) => rhs.continues(token),
            Node::Modulus(_lhs, rhs) => rhs.continues(token),
            Node::Assignment(_lhs, rhs) => rhs.continues(token),
            Node::PartialFunctionDefinition(_args, nodes) => {
                let last_node = nodes.last().unwrap();
                if last_node.continues(&token).unwrap() {
                    Some(true)
                } else {
                    match token {
                        Token::CloseBrace | Token::NewLine | Token::SemiColon => Some(true),
                        _ => Some(false),
                    }
                }
            }
            Node::PartialParenthesized(node) => {
                if node.continues(&token).unwrap() {
                    Some(true)
                } else {
                    match token {
                        Token::CloseParenthesis => Some(true),
                        _ => Some(false),
                    }
                }
            }
            Node::Call(_lhs, rhs) => Some(false),
            Node::PartialCall(_lhs, rhs) => match token {
                Token::Comma => Some(true),
                Token::CloseParenthesis => Some(true),
                _ => {
                    if rhs.len() == 0 {
                        Some(true)
                    } else {
                        rhs.last().unwrap().continues(token)
                    }
                }
            },
            Node::Empty => {
                match token {
                    Token::CloseBrace => Some(false),
                    _ => Some(true),
                }
            },
            _ => Some(false),
        }
    }

    pub fn append(&mut self, token: Token) {
        match self {
            Node::Identifier(_) | Node::Number(_) => match token {
                Token::Equal => {
                    *self = Node::Assignment(Box::new(self.clone()), Box::new(Node::Empty));
                }
                Token::Plus => {
                    *self = Node::Addition(Box::new(self.clone()), Box::new(Node::Empty));
                }
                Token::Minus => {
                    *self = Node::Substraction(Box::new(self.clone()), Box::new(Node::Empty));
                }
                Token::Times => {
                    *self = Node::Multiplication(Box::new(self.clone()), Box::new(Node::Empty));
                }
                Token::Division => {
                    *self = Node::Division(Box::new(self.clone()), Box::new(Node::Empty));
                }
                Token::Modulus => {
                    *self = Node::Modulus(Box::new(self.clone()), Box::new(Node::Empty));
                }
                Token::OpenParenthesis => {
                    *self = Node::PartialCall(Box::new(self.clone()), vec![]);
                }
                Token::Period => {
                    unimplemented!();
                }
                _ => {}
            },
            Node::Parenthesized(node) => match token {
                Token::Times => *self = Node::Multiplication(Box::new(self.clone()), Box::new(Node::Empty)),
                _ => unimplemented!(),
            },
            Node::Addition(_lhs, rhs) => match token {
                Token::Plus => *self = Node::Addition(Box::new(self.clone()), Box::new(Node::Empty)),
                _ => {
                    rhs.append(token);
                }
            },
            Node::Substraction(_lhs, rhs) => match token {
                Token::Plus => *self = Node::Addition(Box::new(self.clone()), Box::new(Node::Empty)),
                _ => {
                    rhs.append(token);
                }
            },
            Node::Modulus(_lhs, rhs) => match token {
                Token::Plus => *self = Node::Addition(Box::new(self.clone()), Box::new(Node::Empty)),
                _ => {
                    rhs.append(token);
                }
            },
            Node::Multiplication(_lhs, rhs) => match token {
                Token::Times => *self = Node::Multiplication(Box::new(self.clone()), Box::new(Node::Empty)),
                Token::Plus => *self = Node::Addition(Box::new(self.clone()), Box::new(Node::Empty)),
                _ => {
                    rhs.append(token);
                }
            },
            Node::Division(_lhs, rhs) => match token {
                Token::Times => *self = Node::Multiplication(Box::new(self.clone()), Box::new(Node::Empty)),
                Token::Plus => *self = Node::Addition(Box::new(self.clone()), Box::new(Node::Empty)),
                _ => {
                    rhs.append(token);
                }
            },
            Node::PartialParenthesized(node) => {
                if node.continues(&token).unwrap() {
                    node.append(token);
                } else {
                    if token == Token::CloseParenthesis {
                        *self = Node::Parenthesized(node.clone());
                    }
                }
            },
            Node::PartialFunctionDefinition(args, nodes) => {
                let mut last_node = nodes.last_mut().unwrap();
                if last_node.continues(&token).unwrap() {
                    last_node.append(token);
                } else {
                    match token {
                        Token::CloseBrace => {
                            if *last_node == Node::Empty {
                                nodes.remove(nodes.len()-1);
                            }
                            *self = Node::FunctionDefinition(args.to_vec(), nodes.to_vec());
                        }
                        Token::NewLine => {
                            nodes.push(Node::Empty);
                        }
                        _ => {}
                    }
                }
            },
            Node::PartialCall(callee, args) => match token {
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
                }
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
            Node::Empty => {
                *self = Node::start_of(token).unwrap();
            }
            _ => {}
        }
    }
}
