#[allow(unused_imports)]
use crate::lexer;
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
}
