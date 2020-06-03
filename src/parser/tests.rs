#[allow(unused_imports)]
use crate::parser;
use crate::lexer;

#[cfg(test)]
mod tests {
    #[test]
    fn identifier() {
        assert_eq!(
            super::parser::parser::parse(vec![
                super::lexer::token::Token::Identifier(String::from("x")),
            ]),
            Some(
                super::parser::node::Node::Program(
                    vec![
                        super::parser::node::Node::Identifier(
                            super::lexer::token::Token::Identifier(
                                String::from("x"),
                            ),
                        ),
                    ],
                ),
            )
        );
    }

    #[test]
    fn number() {
        assert_eq!(
            super::parser::parser::parse(vec![
                super::lexer::token::Token::Number(String::from("4")),
            ]),
            Some(
                super::parser::node::Node::Program(
                    vec![
                        super::parser::node::Node::Number(
                            super::lexer::token::Token::Number(
                                String::from("4"),
                            ),
                        ),
                    ],
                ),
            )
        );
    }

    #[test]
    fn basic_assignation() {
        assert_eq!(
            super::parser::parser::parse(vec![
                super::lexer::token::Token::Identifier(String::from("x")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from("4")),
            ]),
            Some(
                super::parser::node::Node::Program(
                    vec![
                        super::parser::node::Node::Assignment(
                            Box::new(
                                super::parser::node::Node::Identifier(
                                    super::lexer::token::Token::Identifier(
                                        String::from("x"),
                                    ),
                                ),
                            ),
                            Box::new(
                                super::parser::node::Node::Number(
                                    super::lexer::token::Token::Number(
                                        String::from("4"),
                                    ),
                                ),
                            ),
                        ),
                    ],
                ),
            )
        );
    }
}
