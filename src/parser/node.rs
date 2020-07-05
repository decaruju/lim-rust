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
    Literal(String, char),
    Call(Box<Node>, Vec<Node>),
    PartialCall(Box<Node>, Vec<Node>),
    Parenthesized(Box<Node>),
    Member(Box<Node>, String),
    PartialMember(Box<Node>),
    PartialParenthesized(Box<Node>),
    FunctionArgs(Vec<Node>),
    PartialFunctionDefinition(Vec<Node>, Vec<Node>),
    FunctionDefinition(Vec<Node>, Vec<Node>),
    UnnamedEnumDefinition,
    NamedEnumDefinition(Box<Node>),
    PartialEnumDefinition(Box<Node>, Vec<Node>),
    EnumDefinition(Box<Node>, Vec<Node>),
    UnnamedClassDefinition,
    NamedClassDefinition(Box<Node>),
    PartialClassDefinition(Box<Node>, Box<Node>),
    ClassDefinition(Box<Node>, Box<Node>),
    PartialMatch(Box<Node>, Vec<Node>),
    UnopenedMatch(Box<Node>),
    UnopenedMatchArm(Box<Node>),
    MatchArmWithoutColon(Box<Node>),
    PartialMatchArm(Box<Node>, Box<Node>),
    MatchArm(Box<Node>, Box<Node>),
    Match(Box<Node>, Vec<Node>),
    Empty,
}

impl Node {
    pub fn is_a(&self, node: Node) -> bool {
        match self {
            node => true,
            _ => false,
        }
    }

    pub fn start_of(token: Token) -> Option<Node> {
        match token {
            Token::Identifier(string) => {
                if string == String::from("enum") {
                    Some(Node::UnnamedEnumDefinition)
                } else if string == String::from("class") {
                    Some(Node::UnnamedClassDefinition)
                } else {
                    Some(Node::Identifier(string))
                }
            }
            Token::Literal(string, delimiter) => Some(Node::Literal(string, delimiter)),
            Token::Number(string) => Some(Node::Number(string)),
            Token::OpenParenthesis => Some(Node::PartialParenthesized(Box::new(Node::Empty))),
            Token::NewLine | Token::SemiColon => Some(Node::Empty),
            _ => None,
        }
    }

    pub fn continues(&self, token: &Token) -> Option<bool> {
        match self {
            Node::Member(node, string) => {
                match token {
                    Token::OpenParenthesis => Some(true),
                    _ => Some(false),
                }
            }
            Node::Program(nodes) => {
                if let Some(last_node) = nodes.last() {
                    if last_node.continues(token).unwrap() {
                        Some(true)
                    } else {
                        match token {
                            Token::Identifier(_) => Some(true),
                            _ => Some(false),
                        }
                    }
                } else {
                    match token {
                        Token::Identifier(_) => Some(true),
                        _ => Some(false),
                    }
                }
            }
            Node::PartialMember(node) => match token {
                Token::Identifier(_) => Some(true),
                _ => panic!("{:?}", token),
            },
            Node::UnnamedEnumDefinition | Node::UnnamedClassDefinition => match token {
                Token::Identifier(_) => Some(true),
                _ => panic!("{:?}", token),
            },
            Node::NamedEnumDefinition(_) | Node::NamedClassDefinition(_) => match token {
                Token::OpenBrace => Some(true),
                _ => panic!("{:?}", token),
            },
            Node::PartialEnumDefinition(_, _) => match token {
                Token::CloseBrace => Some(true),
                Token::Identifier(_) => Some(true),
                Token::NewLine => Some(true),
                _ => panic!("{:?}", token),
            },
            Node::PartialClassDefinition(_, code) => {
                if code.continues(token).unwrap() {
                    return code.continues(token)
                }
                match token {
                    Token::CloseBrace => Some(true),
                    Token::NewLine => Some(true),
                    _ => panic!("{:?}", token),
                }
            },
            Node::UnopenedMatch(node) => match token {
                Token::OpenBrace => Some(true),
                _ => panic!("{:?}", token),
            },
            Node::Identifier(_identifier) => match token {
                Token::Equal | Token::Plus | Token::Minus | Token::Times | Token::Division | Token::Period | Token::Modulus | Token::OpenParenthesis | Token::Colon => Some(true),
                _ => Some(false),
            },
            Node::Number(_identifier) => match token {
                Token::Plus | Token::Minus | Token::Times | Token::Division | Token::Period | Token::Modulus | Token::OpenParenthesis => Some(true),
                _ => Some(false),
            },
            Node::Parenthesized(_identifier) => match token {
                Token::Plus | Token::Minus | Token::Times | Token::Division | Token::Period | Token::Modulus | Token::OpenParenthesis | Token::OpenBrace => Some(true),
                _ => Some(false),
            },
            Node::PartialMatch(_matched, arms) => {
                if let Token::NewLine = token {
                    return Some(true);
                }
                let last_arm = arms.last();
                if let Some(Node::PartialMatchArm(matcher, nodes)) = last_arm {
                    if nodes.continues(token).unwrap() {
                        Some(true)
                    } else {
                        match token {
                            Token::CloseBrace => Some(true),
                            _ => Some(false),
                        }
                    }
                } else {
                    match token {
                        Token::Identifier(_) | Token::CloseBrace | Token::Colon | Token::OpenBrace | Token::Period => Some(true),
                        _ => panic!("{:?}", token),
                    }
                }
            }
            Node::PartialMatchArm(matcher, program) => {
                if let Some(true) = program.continues(token) {
                    Some(true)
                } else {
                    match token {
                        Token::CloseBrace => Some(true),
                        _ => panic!("Cant add token {:?} to node {:?}", token, self),
                    }
                }
            }
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
            Node::Empty => match token {
                Token::CloseBrace | Token::CloseParenthesis => Some(false),
                _ => Some(true),
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
                Token::Period => *self = Node::PartialMember(Box::new(self.clone())),
                Token::Colon => {
                    *self = Node::UnopenedMatch(Box::new(self.clone()));
                }
                _ => {}
            },
            Node::Parenthesized(node) => match token {
                Token::Times => *self = Node::Multiplication(Box::new(self.clone()), Box::new(Node::Empty)),
                Token::OpenBrace => {
                    let arguments = match **node {
                        Node::Empty => vec![],
                        _ => vec![*node.clone()],
                    };
                    *self = Node::PartialFunctionDefinition(arguments, vec![Node::Empty]);
                }
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
            }
            Node::PartialFunctionDefinition(args, nodes) => {
                let mut last_node = nodes.last_mut().unwrap();
                if last_node.continues(&token).unwrap() {
                    last_node.append(token);
                } else {
                    match token {
                        Token::CloseBrace => {
                            if *last_node == Node::Empty {
                                nodes.remove(nodes.len() - 1);
                            }
                            *self = Node::FunctionDefinition(args.to_vec(), nodes.to_vec());
                        }
                        Token::NewLine => {
                            nodes.push(Node::Empty);
                        }
                        _ => {}
                    }
                }
            }
            Node::UnnamedClassDefinition => match token {
                Token::Identifier(string) => {
                    *self = Node::NamedClassDefinition(Box::new(Node::Identifier(string)));
                }
                _ => {}
            },
            Node::UnnamedEnumDefinition => match token {
                Token::Identifier(string) => {
                    *self = Node::NamedEnumDefinition(Box::new(Node::Identifier(string)));
                }
                _ => {}
            },
            Node::NamedEnumDefinition(name) => match token {
                Token::OpenBrace => {
                    *self = Node::PartialEnumDefinition(name.clone(), vec![]);
                }
                _ => {}
            },
            Node::NamedClassDefinition(name) => match token {
                Token::OpenBrace => {
                    *self = Node::PartialClassDefinition(name.clone(), Box::new(Node::Program(vec![])));
                }
                _ => {}
            },
            Node::PartialClassDefinition(name, code) => {
                if code.continues(&token).unwrap() {
                    code.append(token)
                } else {
                    match token {
                        Token::CloseBrace => {
                            *self = Node::ClassDefinition(name.clone(), code.clone());
                        }
                        _ => {}
                    }
                }
            },
            Node::PartialEnumDefinition(name, variations) => match token {
                Token::CloseBrace => {
                    *self = Node::EnumDefinition(name.clone(), variations.to_vec());
                }
                Token::Identifier(string) => variations.push(Node::Identifier(string)),
                _ => {}
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

            Node::UnopenedMatch(node) => {
                *self = Node::PartialMatch(node.clone(), vec![]);
            }

            Node::Program(nodes) => {
                if let Some(last_node) = nodes.last_mut() {
                    if last_node.continues(&token).unwrap() {
                        last_node.append(token)
                    } else {
                        nodes.push(Node::start_of(token).unwrap())
                    }
                } else {
                    nodes.push(Node::start_of(token).unwrap())
                }
            }

            Node::PartialMatch(node, arms) => {
                if let Some(last_arm) = arms.last_mut() {
                    match last_arm {
                        Node::PartialMatchArm(matcher, nodes) => {
                            if nodes.continues(&token).unwrap() {
                                nodes.append(token);
                            } else {
                                match token {
                                    Token::CloseBrace => {
                                        *last_arm = Node::MatchArm(matcher.to_owned(), nodes.to_owned());
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Node::MatchArmWithoutColon(matcher) => match token {
                            Token::Colon => {
                                *last_arm = Node::UnopenedMatchArm(matcher.to_owned());
                            }
                            _ => {
                                if matcher.continues(&token).unwrap() {
                                    matcher.append(token);
                                }
                            }
                        },
                        Node::UnopenedMatchArm(matcher) => {
                            if let Token::OpenBrace = token {
                                *last_arm = Node::PartialMatchArm(matcher.to_owned(), Box::new(Node::Program(vec![])));
                            } else {
                                unreachable!();
                            }
                        }
                        Node::MatchArm(_matcher, _nodes) => match token {
                            Token::CloseBrace => {
                                *self = Node::Match(node.clone(), arms.to_vec());
                            }
                            Token::Identifier(string) => arms.push(Node::MatchArmWithoutColon(Box::new(Node::Identifier(string)))),
                            _ => {}
                        },
                        _ => {}
                    }
                } else {
                    match token {
                        Token::CloseBrace => {
                            *self = Node::Match(node.clone(), arms.to_vec());
                        }
                        Token::Identifier(string) => {
                            arms.push(Node::MatchArmWithoutColon(Box::new(Node::Identifier(string))));
                        }
                        _ => {}
                    }
                }
            }
            Node::PartialMember(node) => {
                if let Token::Identifier(string) = token {
                    *self = Node::Member(node.to_owned(), string);
                } else {
                    unreachable!();
                }
            }
            Node::Member(node, string) => {
                match token {
                    Token::OpenParenthesis => {
                        *self = Node::PartialCall(Box::new(self.to_owned()), vec![]);
                    }
                    _ => unimplemented!(),
                }
            }
            _ => {}
        }
    }
}
