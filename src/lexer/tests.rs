use crate::lexer;

#[cfg(test)]
mod tests {
    #[test]
    fn float_with_multiple_periods() {
        assert_eq!(
            super::lexer::lexer::lex("3.3."),
            None,
        );
    }

    #[test]
    fn basic_assignation() {
        assert_eq!(
            super::lexer::lexer::lex("x = 4"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("x")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from("4")),
           ])
        );
    }

    #[test]
    fn long_variable_name() {
        assert_eq!(
            super::lexer::lexer::lex("foobarbaz = 4"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("foobarbaz")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from("4")),
           ])
        );
    }

    #[test]
    fn long_integer_number() {
        assert_eq!(
            super::lexer::lexer::lex("x = 123456789"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("x")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from("123456789")),
           ])
        );
    }

    #[test]
    fn underscore_in_variable() {
        assert_eq!(
            super::lexer::lexer::lex("foo_bar = 4"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("foo_bar")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from("4")),
           ])
        );
    }

    #[test]
    fn variable_starting_with_underscore() {
        assert_eq!(
            super::lexer::lexer::lex("_x = 4"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("_x")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from("4")),
           ])
        );
    }

    #[test]
    fn number_in_variable() {
        assert_eq!(
            super::lexer::lexer::lex("foo4 = 4"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("foo4")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from("4")),
           ])
        );
    }

    #[test]
    fn camel_case_variable() {
        assert_eq!(
            super::lexer::lexer::lex("fooBar = 4"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("fooBar")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from("4")),
           ])
        );
    }

    #[test]
    fn float_assignation() {
        assert_eq!(
            super::lexer::lexer::lex("x = 4.2"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("x")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from("4.2")),
           ])
        );
    }

    #[test]
    fn float_starting_with_period() {
        assert_eq!(
            super::lexer::lexer::lex("x = .2"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("x")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from(".2")),
           ])
        );
    }

    #[test]
    fn variable_assigned_to_variable() {
        assert_eq!(
            super::lexer::lexer::lex("x = y"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("x")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Identifier(String::from("y")),
           ])
        );
    }

    #[test]
    fn addition() {
        assert_eq!(
            super::lexer::lexer::lex("2 + 4"),
            Some(vec![
                super::lexer::token::Token::Number(String::from("2")),
                super::lexer::token::Token::Plus,
                super::lexer::token::Token::Number(String::from("4")),
           ])
        );
    }

    #[test]
    fn float_addition() {
        assert_eq!(
            super::lexer::lexer::lex(".2 + 4.0"),
            Some(vec![
                super::lexer::token::Token::Number(String::from(".2")),
                super::lexer::token::Token::Plus,
                super::lexer::token::Token::Number(String::from("4.0")),
            ])
        );
    }

    #[test]
    fn multiple_addition() {
        assert_eq!(
            super::lexer::lexer::lex(".2 + 4.0 + 4 + 2"),
            Some(vec![
                super::lexer::token::Token::Number(String::from(".2")),
                super::lexer::token::Token::Plus,
                super::lexer::token::Token::Number(String::from("4.0")),
                super::lexer::token::Token::Plus,
                super::lexer::token::Token::Number(String::from("4")),
                super::lexer::token::Token::Plus,
                super::lexer::token::Token::Number(String::from("2")),
            ])
        );
    }

    #[test]
    fn addition_assignation() {
        assert_eq!(
            super::lexer::lexer::lex("x = .2 + 4.0"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("x")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from(".2")),
                super::lexer::token::Token::Plus,
                super::lexer::token::Token::Number(String::from("4.0")),
            ])
        );
    }

    #[test]
    fn all_minus() {
        assert_eq!(
            super::lexer::lexer::lex("x = .2 - 4.0 - 3 + 1"),
            Some(vec![
                super::lexer::token::Token::Identifier(String::from("x")),
                super::lexer::token::Token::Equal,
                super::lexer::token::Token::Number(String::from(".2")),
                super::lexer::token::Token::Minus,
                super::lexer::token::Token::Number(String::from("4.0")),
                super::lexer::token::Token::Minus,
                super::lexer::token::Token::Number(String::from("3")),
                super::lexer::token::Token::Plus,
                super::lexer::token::Token::Number(String::from("1")),
            ])
        );
    }

    #[test]
    fn float_without_digits() {
        assert_eq!(
            super::lexer::lexer::lex("2 + ."),
            None,
        );
    }

    #[test]
    fn arithmetic_without_parenthesis() {
        assert_eq!(
            super::lexer::lexer::lex("1 + 2 - 3 * .4 / 5.0 % 6"),
            Some(vec![
                super::lexer::token::Token::Number(String::from("1")),
                super::lexer::token::Token::Plus,
                super::lexer::token::Token::Number(String::from("2")),
                super::lexer::token::Token::Minus,
                super::lexer::token::Token::Number(String::from("3")),
                super::lexer::token::Token::Times,
                super::lexer::token::Token::Number(String::from(".4")),
                super::lexer::token::Token::Division,
                super::lexer::token::Token::Number(String::from("5.0")),
                super::lexer::token::Token::Modulus,
                super::lexer::token::Token::Number(String::from("6")),
            ])
        );
    }

    #[test]
    fn arithmetic_with_single_parenthesis() {
        assert_eq!(
            super::lexer::lexer::lex("1 + (1 + 1)"),
            Some(vec![
                super::lexer::token::Token::Number(String::from("1")),
                super::lexer::token::Token::Plus,
                super::lexer::token::Token::OpenParenthesis,
                super::lexer::token::Token::Number(String::from("1")),
                super::lexer::token::Token::Plus,
                super::lexer::token::Token::Number(String::from("1")),
                super::lexer::token::Token::CloseParenthesis,
           ])
        );
    }
}
