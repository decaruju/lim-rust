#[allow(unused_imports)]
use lexer;
#[allow(unused_imports)]
use crate::parser;

#[cfg(test)]
mod tests {
    use super::lexer::token::Token;
    use super::parser::node::Node;
    use super::parser::parser::parse;

    #[test]
    fn identifier() {
        assert_eq!(parse(vec![Token::Identifier(String::from("x")),]), Some(Node::Program(vec![Node::Identifier(String::from("x")),],),));
    }

    #[test]
    fn number() {
        assert_eq!(parse(vec![Token::Number(String::from("4")),]), Some(Node::Program(vec![Node::Number(String::from("4"),),],),));
    }

    #[test]
    fn basic_assignation() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("x")), Token::Equal, Token::Number(String::from("4")),]),
            Some(Node::Program(vec![Node::Assignment(Box::new(Node::Identifier(String::from("x"),),), Box::new(Node::Number(String::from("4"),),),),],),)
        );
    }

    #[test]
    fn member_on_literal() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("foo")), Token::Period, Token::Identifier(String::from("bar")),]),
            Some(Node::Program(vec![Node::Member(Box::new(Node::Identifier(String::from("foo"),),), String::from("bar"))])),
        );
    }

    #[test]
    fn assignment_of_member_on_literal() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::Equal,
                Token::Identifier(String::from("baz")),
                Token::Period,
                Token::Identifier(String::from("bar")),
            ]),
            Some(Node::Program(vec![Node::Assignment(
                Box::new(Node::Identifier(String::from("foo"))),
                Box::new(Node::Member(Box::new(Node::Identifier(String::from("baz"),),), String::from("bar")))
            )])),
        );
    }

    #[test]
    fn float_assignation() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("x")), Token::Equal, Token::Number(String::from("4.0")),]),
            Some(Node::Program(vec![Node::Assignment(Box::new(Node::Identifier(String::from("x"),),), Box::new(Node::Number(String::from("4.0"),),),),],),)
        );
    }

    #[test]
    fn sum() {
        assert_eq!(
            parse(vec![Token::Number(String::from("2")), Token::Plus, Token::Number(String::from("2.0")),]),
            Some(Node::Program(vec![Node::Addition(Box::new(Node::Number(String::from("2"),),), Box::new(Node::Number(String::from("2.0"),),),),],),)
        );
    }

    #[test]
    fn minus() {
        assert_eq!(
            parse(vec![Token::Number(String::from("2")), Token::Minus, Token::Number(String::from("2.0")),]),
            Some(Node::Program(vec![Node::Substraction(Box::new(Node::Number(String::from("2"),),), Box::new(Node::Number(String::from("2.0"),),),),],),)
        );
    }

    #[test]
    fn modulus() {
        assert_eq!(
            parse(vec![Token::Number(String::from("2")), Token::Modulus, Token::Number(String::from("2.0")),]),
            Some(Node::Program(vec![Node::Modulus(Box::new(Node::Number(String::from("2"),),), Box::new(Node::Number(String::from("2.0"),),),),],),)
        );
    }

    #[test]
    fn variable_sum() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("x")), Token::Plus, Token::Number(String::from("2.0")),]),
            Some(Node::Program(vec![Node::Addition(Box::new(Node::Identifier(String::from("x"),),), Box::new(Node::Number(String::from("2.0"),),),),],),)
        );
    }

    #[test]
    fn variable_minus() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("x")), Token::Minus, Token::Number(String::from("2.0")),]),
            Some(Node::Program(vec![Node::Substraction(Box::new(Node::Identifier(String::from("x"),),), Box::new(Node::Number(String::from("2.0"),),),),],),)
        );
    }

    #[test]
    fn variable_times() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("x")), Token::Times, Token::Number(String::from("2.0")),]),
            Some(Node::Program(vec![Node::Multiplication(Box::new(Node::Identifier(String::from("x"),),), Box::new(Node::Number(String::from("2.0"),),),),],),)
        );
    }

    #[test]
    fn variable_division() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("x")), Token::Division, Token::Number(String::from("2.0")),]),
            Some(Node::Program(vec![Node::Division(Box::new(Node::Identifier(String::from("x"),),), Box::new(Node::Number(String::from("2.0"),),),),],),)
        );
    }

    #[test]
    fn variable_modulus() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("x")), Token::Modulus, Token::Number(String::from("2.0")),]),
            Some(Node::Program(vec![Node::Modulus(Box::new(Node::Identifier(String::from("x"),),), Box::new(Node::Number(String::from("2.0"),),),),],),)
        );
    }

    #[test]
    fn two_sums() {
        assert_eq!(
            parse(vec![Token::Number(String::from("2")), Token::Plus, Token::Number(String::from("2.0")), Token::Plus, Token::Number(String::from("3.0")),]),
            Some(Node::Program(vec![Node::Addition(
                Box::new(Node::Addition(Box::new(Node::Number(String::from("2"),),), Box::new(Node::Number(String::from("2.0"),),),),),
                Box::new(Node::Number(String::from("3.0"),),),
            ),],),)
        );
    }

    #[test]
    fn product() {
        assert_eq!(
            parse(vec![Token::Number(String::from("2")), Token::Times, Token::Number(String::from("2.0")),]),
            Some(Node::Program(vec![Node::Multiplication(Box::new(Node::Number(String::from("2"),),), Box::new(Node::Number(String::from("2.0"),),),),],),)
        );
    }

    #[test]
    fn division() {
        assert_eq!(
            parse(vec![Token::Number(String::from("2")), Token::Division, Token::Number(String::from("2.0")),]),
            Some(Node::Program(vec![Node::Division(Box::new(Node::Number(String::from("2"),),), Box::new(Node::Number(String::from("2.0"),),),),],),)
        );
    }

    #[test]
    fn two_products() {
        assert_eq!(
            parse(vec![Token::Number(String::from("2")), Token::Times, Token::Number(String::from("2.0")), Token::Times, Token::Number(String::from("3.0")),]),
            Some(Node::Program(vec![Node::Multiplication(
                Box::new(Node::Multiplication(Box::new(Node::Number(String::from("2"),),), Box::new(Node::Number(String::from("2.0"),),),),),
                Box::new(Node::Number(String::from("3.0"),),),
            ),],),)
        );
    }

    #[test]
    fn product_and_sum() {
        assert_eq!(
            parse(vec![Token::Number(String::from("2")), Token::Plus, Token::Number(String::from("2")), Token::Times, Token::Number(String::from("2")),]),
            Some(Node::Program(vec![Node::Addition(
                Box::new(Node::Number(String::from("2"),),),
                Box::new(Node::Multiplication(Box::new(Node::Number(String::from("2"),),), Box::new(Node::Number(String::from("2"),),),),),
            ),],),)
        );
    }

    #[test]
    fn sum_and_product() {
        assert_eq!(
            parse(vec![Token::Number(String::from("2")), Token::Times, Token::Number(String::from("2")), Token::Plus, Token::Number(String::from("2")),]),
            Some(Node::Program(vec![Node::Addition(
                Box::new(Node::Multiplication(Box::new(Node::Number(String::from("2"),),), Box::new(Node::Number(String::from("2"),),),),),
                Box::new(Node::Number(String::from("2"),),),
            ),],),)
        );
    }

    #[test]
    fn function_call_without_args() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("foo")), Token::OpenParenthesis, Token::CloseParenthesis,]),
            Some(Node::Program(vec![Node::Call(Box::new(Node::Identifier(String::from("foo"))), vec![],),],),)
        );
    }

    #[test]
    fn function_call_addition_arg() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::OpenParenthesis,
                Token::Number(String::from("2")),
                Token::Plus,
                Token::Number(String::from("2")),
                Token::CloseParenthesis,
            ]),
            Some(Node::Program(vec![Node::Call(
                Box::new(Node::Identifier(String::from("foo"))),
                vec![Node::Addition(Box::new(Node::Number(String::from("2"))), Box::new(Node::Number(String::from("2"))),),],
            ),],),)
        );
    }

    #[test]
    fn function_call_number_arg() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("foo")), Token::OpenParenthesis, Token::Number(String::from("2")), Token::CloseParenthesis,]),
            Some(Node::Program(vec![Node::Call(Box::new(Node::Identifier(String::from("foo"))), vec![Node::Number(String::from("2"))],),],),)
        );
    }

    // #[test]
    // fn function_call_one_and_a_half_args() {
    //     assert_eq!(
    //         parse(vec![
    //             Token::Identifier(String::from("foo")),
    //             Token::OpenParenthesis,
    //             Token::Number(String::from("2")),
    //             Token::Comma,
    //             Token::CloseParenthesis,
    //         ]),
    //         Some(Node::Program(vec![Node::Call(
    //             Box::new(Node::Identifier(String::from("foo"))),
    //             vec![
    //                 Node::Number(String::from("2")),
    //                 Node::Empty,
    //             ],
    //         ),],),)
    //     );
    // }

    #[test]
    fn function_call_two_args() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::OpenParenthesis,
                Token::Number(String::from("2")),
                Token::Comma,
                Token::Number(String::from("2")),
                Token::CloseParenthesis,
            ]),
            Some(Node::Program(vec![Node::Call(
                Box::new(Node::Identifier(String::from("foo"))),
                vec![Node::Number(String::from("2")), Node::Number(String::from("2")),],
            ),],),)
        );
    }

    #[test]
    fn function_call_first_arg_is_function_call() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::OpenParenthesis,
                Token::Identifier(String::from("bar")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::CloseParenthesis,
            ]),
            Some(Node::Program(vec![Node::Call(
                Box::new(Node::Identifier(String::from("foo"))),
                vec![Node::Call(Box::new(Node::Identifier(String::from("bar"),),), vec![],),],
            ),],),)
        );
    }

    #[test]
    fn function_call_first_arg_is_function_call_with_arg() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::OpenParenthesis,
                Token::Identifier(String::from("bar")),
                Token::OpenParenthesis,
                Token::Identifier(String::from("baz")),
                Token::CloseParenthesis,
                Token::CloseParenthesis,
            ]),
            Some(Node::Program(vec![Node::Call(
                Box::new(Node::Identifier(String::from("foo"))),
                vec![Node::Call(Box::new(Node::Identifier(String::from("bar"),),), vec![Node::Identifier(String::from("baz"),),],),],
            ),],),)
        );
    }

    #[test]
    fn function_call_first_arg_is_function_call_with_two_args() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::OpenParenthesis,
                Token::Identifier(String::from("bar")),
                Token::OpenParenthesis,
                Token::Identifier(String::from("baz")),
                Token::Comma,
                Token::Identifier(String::from("bim")),
                Token::CloseParenthesis,
                Token::CloseParenthesis,
            ]),
            Some(Node::Program(vec![Node::Call(
                Box::new(Node::Identifier(String::from("foo"))),
                vec![Node::Call(
                    Box::new(Node::Identifier(String::from("bar"),),),
                    vec![Node::Identifier(String::from("baz"),), Node::Identifier(String::from("bim"),),],
                ),],
            ),],),)
        );
    }

    #[test]
    fn function_call_two_args_first_is_function_call_with_two_args() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::OpenParenthesis,
                Token::Identifier(String::from("bar")),
                Token::OpenParenthesis,
                Token::Identifier(String::from("baz")),
                Token::Comma,
                Token::Identifier(String::from("bim")),
                Token::CloseParenthesis,
                Token::Comma,
                Token::Identifier(String::from("bam")),
                Token::CloseParenthesis,
            ]),
            Some(Node::Program(vec![Node::Call(
                Box::new(Node::Identifier(String::from("foo"))),
                vec![
                    Node::Call(Box::new(Node::Identifier(String::from("bar"),),), vec![Node::Identifier(String::from("baz"),), Node::Identifier(String::from("bim"),),],),
                    Node::Identifier(String::from("bam"),),
                ],
            ),],),)
        );
    }

    #[test]
    fn method_call() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::Period,
                Token::Identifier(String::from("bar")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
            ]),
            Some(Node::Program(vec![Node::Call(
                Box::new(Node::Member(
                    Box::new(
                        Node::Identifier(
                            String::from("foo"),
                        ),
                    ),
                    String::from("bar"),
                )),
                vec![]
            ),],),)
        );
    }

    #[test]
    fn parenthesized_empty() {
        assert_eq!(parse(vec![Token::OpenParenthesis, Token::CloseParenthesis,]), Some(Node::Program(vec![Node::Parenthesized(Box::new(Node::Empty),),],),));
    }

    #[test]
    fn parenthesized_addition() {
        assert_eq!(
            parse(vec![Token::OpenParenthesis, Token::Identifier(String::from("bar")), Token::Plus, Token::Number(String::from("2")), Token::CloseParenthesis,]),
            Some(Node::Program(vec![Node::Parenthesized(Box::new(Node::Addition(
                Box::new(Node::Identifier(String::from("bar"))),
                Box::new(Node::Number(String::from("2"))),
            )),),],),)
        );
    }

    #[test]
    fn parenthesized_addition_times() {
        assert_eq!(
            parse(vec![
                Token::OpenParenthesis,
                Token::Identifier(String::from("bar")),
                Token::Plus,
                Token::Number(String::from("2")),
                Token::CloseParenthesis,
                Token::Times,
                Token::Number(String::from("2")),
            ]),
            Some(Node::Program(vec![Node::Multiplication(
                Box::new(Node::Parenthesized(Box::new(Node::Addition(Box::new(Node::Identifier(String::from("bar"))), Box::new(Node::Number(String::from("2"))),)),),),
                Box::new(Node::Number(String::from("2"))),
            ),],),)
        );
    }

    #[test]
    fn function_definition_no_args_no_body() {
        assert_eq!(
            parse(vec![Token::OpenParenthesis, Token::CloseParenthesis, Token::OpenBrace, Token::CloseBrace,]),
            Some(Node::Program(vec![Node::FunctionDefinition(vec![], vec![],),],),)
        );
    }

    #[test]
    fn function_definition_no_args_no_body_with_linebreak() {
        assert_eq!(
            parse(vec![Token::OpenParenthesis, Token::CloseParenthesis, Token::OpenBrace, Token::NewLine, Token::CloseBrace,]),
            Some(Node::Program(vec![Node::FunctionDefinition(vec![], vec![],),],),)
        );
    }

    #[test]
    fn function_definition_no_args_single_statement() {
        assert_eq!(
            parse(vec![
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::Identifier(String::from("foo")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::CloseBrace,
            ]),
            Some(Node::Program(vec![Node::FunctionDefinition(vec![], vec![Node::Call(Box::new(Node::Identifier(String::from("foo"),),), vec![],)],),],),)
        );
    }

    #[test]
    fn function_definition_no_args_single_statement_with_linebreaks() {
        assert_eq!(
            parse(vec![
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::NewLine,
                Token::Identifier(String::from("foo")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::NewLine,
                Token::CloseBrace,
            ]),
            Some(Node::Program(vec![Node::FunctionDefinition(vec![], vec![Node::Call(Box::new(Node::Identifier(String::from("foo"),),), vec![],)],),],),)
        );
    }

    #[test]
    fn function_definition_no_args_single_statement_with_linebreaks_and_semi_colon() {
        assert_eq!(
            parse(vec![
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::NewLine,
                Token::Identifier(String::from("foo")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::SemiColon,
                Token::NewLine,
                Token::CloseBrace,
            ]),
            Some(Node::Program(vec![Node::FunctionDefinition(vec![], vec![Node::Call(Box::new(Node::Identifier(String::from("foo"),),), vec![],)],),],),)
        );
    }

    #[test]
    fn function_definition_no_args_two_statements() {
        assert_eq!(
            parse(vec![
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::NewLine,
                Token::Identifier(String::from("foo")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::NewLine,
                Token::Identifier(String::from("bar")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::NewLine,
                Token::CloseBrace,
            ]),
            Some(Node::Program(vec![Node::FunctionDefinition(
                vec![],
                vec![Node::Call(Box::new(Node::Identifier(String::from("foo"),),), vec![],), Node::Call(Box::new(Node::Identifier(String::from("bar"),),), vec![],)],
            ),],),)
        );
    }

    #[test]
    fn function_definition_one_arg() {
        assert_eq!(
            parse(vec![Token::OpenParenthesis, Token::Identifier(String::from("foo")), Token::CloseParenthesis, Token::OpenBrace, Token::CloseBrace,]),
            Some(Node::Program(vec![Node::FunctionDefinition(vec![Node::Identifier(String::from("foo")),], vec![],),],),)
        );
    }

    #[test]
    fn enum_definition() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("enum")), Token::Identifier(String::from("foo")), Token::OpenBrace, Token::CloseBrace,]),
            Some(Node::Program(vec![Node::EnumDefinition(Box::new(Node::Identifier(String::from("foo"))), vec![],)])),
        );
    }

    #[test]
    fn class_definition() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("class")), Token::Identifier(String::from("foo")), Token::OpenBrace, Token::CloseBrace,]),
            Some(Node::Program(vec![Node::ClassDefinition(Box::new(Node::Identifier(String::from("foo"))), Box::new(Node::Program(vec![])),)])),
        );
    }

    #[test]
    fn class_definition_with_single_field() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("class")), Token::Identifier(String::from("foo")), Token::OpenBrace, Token::NewLine, Token::Identifier(String::from("field1")), Token::Equal, Token::Identifier(String::from("None")), Token::NewLine, Token::CloseBrace,]),
            Some(Node::Program(vec![Node::ClassDefinition(Box::new(Node::Identifier(String::from("foo"))), Box::new(Node::Program(vec![
                Node::Assignment(
                    Box::new(
                        Node::Identifier(
                            String::from("field1")
                        )
                    ),
                    Box::new(
                        Node::Identifier(
                            String::from("None")
                        )
                    )
                )
            ]),))])),
        );
    }

    #[test]
    fn class_definition_with_method() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("class")), Token::Identifier(String::from("foo")), Token::OpenBrace, Token::NewLine, Token::Identifier(String::from("field1")), Token::Equal, Token::OpenParenthesis, Token::CloseParenthesis, Token::OpenBrace, Token::NewLine, Token::Identifier(String::from("None")), Token::NewLine, Token::CloseBrace, Token::NewLine, Token::CloseBrace,]),
            Some(Node::Program(vec![Node::ClassDefinition(Box::new(Node::Identifier(String::from("foo"))), Box::new(Node::Program(vec![
                Node::Assignment(
                    Box::new(
                        Node::Identifier(
                            String::from("field1")
                        )
                    ),
                    Box::new(
                        Node::FunctionDefinition(
                            vec![],
                            vec![
                                Node::Identifier(
                                    String::from("None")
                                )
                            ],
                        ),
                    ),
                ),
            ]),))])),
        );
    }

    #[test]
    fn class_definition_with_static_method() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("class")), Token::Identifier(String::from("foo")), Token::OpenBrace, Token::NewLine, Token::Identifier(String::from("self")), Token::Period, Token::Identifier(String::from("field1")), Token::Equal, Token::OpenParenthesis, Token::CloseParenthesis, Token::OpenBrace, Token::NewLine, Token::Identifier(String::from("None")), Token::NewLine, Token::CloseBrace, Token::NewLine, Token::CloseBrace,]),
            Some(Node::Program(vec![Node::ClassDefinition(Box::new(Node::Identifier(String::from("foo"))), Box::new(Node::Program(vec![
                Node::Assignment(
                    Box::new(
                        Node::Member(
                            Box::new(
                                Node::Identifier(
                                    String::from("self")
                                )
                            ),
                            String::from("field1")
                        ),
                    ),
                    Box::new(
                        Node::FunctionDefinition(
                            vec![],
                            vec![
                                Node::Identifier(
                                    String::from("None")
                                )
                            ],
                        ),
                    ),
                ),
            ]),))])),
        );
    }

    #[test]
    fn enum_with_variations() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("enum")),
                Token::Identifier(String::from("foo")),
                Token::OpenBrace,
                Token::Identifier(String::from("bar")),
                Token::NewLine,
                Token::Identifier(String::from("baz")),
                Token::CloseBrace,
            ]),
            Some(Node::Program(vec![Node::EnumDefinition(
                Box::new(Node::Identifier(String::from("foo"))),
                vec![Node::Identifier(String::from("bar")), Node::Identifier(String::from("baz")),],
            )])),
        );
    }

    #[test]
    fn pattern_matching_with_no_arms() {
        assert_eq!(
            parse(vec![Token::Identifier(String::from("foo")), Token::Colon, Token::OpenBrace, Token::CloseBrace,]),
            Some(Node::Program(vec![Node::Match(Box::new(Node::Identifier(String::from("foo"))), vec![],)])),
        );
    }

    #[test]
    fn pattern_matching_with_one_empty_arm() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::Colon,
                Token::OpenBrace,
                Token::Identifier(String::from("bar")),
                Token::Colon,
                Token::OpenBrace,
                Token::CloseBrace,
                Token::NewLine,
                Token::CloseBrace,
            ]),
            Some(Node::Program(vec![Node::Match(
                Box::new(Node::Identifier(String::from("foo"))),
                vec![Node::MatchArm(Box::new(Node::Identifier(String::from("bar")),), Box::new(Node::Program(vec![])))],
            )])),
        );
    }

    #[test]
    fn pattern_matching_with_one_arm() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::Colon,
                Token::OpenBrace,
                Token::Identifier(String::from("bar")),
                Token::Colon,
                Token::OpenBrace,
                Token::NewLine,
                Token::Identifier(String::from("bar")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::CloseBrace,
                Token::NewLine,
                Token::CloseBrace,
            ]),
            Some(Node::Program(vec![Node::Match(
                Box::new(Node::Identifier(String::from("foo"))),
                vec![Node::MatchArm(
                    Box::new(Node::Identifier(String::from("bar")),),
                    Box::new(Node::Program(vec![Node::Call(Box::new(Node::Identifier(String::from("bar"))), vec![],)]))
                )],
            )])),
        );
    }

    #[test]
    fn pattern_matching_with_one_arm_with_enum_variant() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::Colon,
                Token::OpenBrace,
                Token::Identifier(String::from("bar")),
                Token::Period,
                Token::Identifier(String::from("bim")),
                Token::Colon,
                Token::OpenBrace,
                Token::NewLine,
                Token::Identifier(String::from("bar")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::CloseBrace,
                Token::NewLine,
                Token::CloseBrace,
            ]),
            Some(Node::Program(vec![Node::Match(
                Box::new(Node::Identifier(String::from("foo"))),
                vec![Node::MatchArm(
                    Box::new(Node::Member(Box::new(Node::Identifier(String::from("bar"),),), String::from("bim"),),),
                    Box::new(Node::Program(vec![Node::Call(Box::new(Node::Identifier(String::from("bar"))), vec![],)]))
                )],
            )])),
        );
    }

    #[test]
    fn pattern_matching_with_two_arms() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("foo")),
                Token::Colon,
                Token::OpenBrace,
                Token::Identifier(String::from("bar")),
                Token::Colon,
                Token::OpenBrace,
                Token::NewLine,
                Token::Identifier(String::from("bar")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::CloseBrace,
                Token::NewLine,
                Token::Identifier(String::from("baz")),
                Token::Colon,
                Token::OpenBrace,
                Token::NewLine,
                Token::Identifier(String::from("baz")),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::CloseBrace,
                Token::NewLine,
                Token::CloseBrace,
            ]),
            Some(Node::Program(vec![Node::Match(
                Box::new(Node::Identifier(String::from("foo"))),
                vec![
                    Node::MatchArm(
                        Box::new(Node::Identifier(String::from("bar")),),
                        Box::new(Node::Program(vec![Node::Call(Box::new(Node::Identifier(String::from("bar"))), vec![],)]))
                    ),
                    Node::MatchArm(
                        Box::new(Node::Identifier(String::from("baz")),),
                        Box::new(Node::Program(vec![Node::Call(Box::new(Node::Identifier(String::from("baz"))), vec![],)]))
                    )
                ],
            )])),
        );
    }
}
