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
        assert_eq!(
            parse(vec![Token::Identifier(String::from("x")),]),
            Some(Node::Program(vec![Node::Identifier(Token::Identifier(
                String::from("x"),
            ),),],),)
        );
    }

    #[test]
    fn number() {
        assert_eq!(
            parse(vec![Token::Number(String::from("4")),]),
            Some(Node::Program(vec![Node::Number(Token::Number(
                String::from("4"),
            ),),],),)
        );
    }

    #[test]
    fn basic_assignation() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("x")),
                Token::Equal,
                Token::Number(String::from("4")),
            ]),
            Some(Node::Program(vec![Node::Assignment(
                Box::new(Node::Identifier(Token::Identifier(String::from("x"),),),),
                Box::new(Node::Number(Token::Number(String::from("4"),),),),
            ),],),)
        );
    }

    #[test]
    fn float_assignation() {
        assert_eq!(
            parse(vec![
                Token::Identifier(String::from("x")),
                Token::Equal,
                Token::Number(String::from("4.0")),
            ]),
            Some(Node::Program(vec![Node::Assignment(
                Box::new(Node::Identifier(Token::Identifier(String::from("x"),),),),
                Box::new(Node::Number(Token::Number(String::from("4.0"),),),),
            ),],),)
        );
    }

    #[test]
    fn sum() {
        assert_eq!(
            parse(vec![
                Token::Number(String::from("2")),
                Token::Plus,
                Token::Number(String::from("2.0")),
            ]),
            Some(Node::Program(vec![Node::Addition(
                Box::new(Node::Number(Token::Number(String::from("2"),),),),
                Box::new(Node::Number(Token::Number(String::from("2.0"),),),),
            ),],),)
        );
    }

    #[test]
    fn two_sums() {
        assert_eq!(
            parse(vec![
                Token::Number(String::from("2")),
                Token::Plus,
                Token::Number(String::from("2.0")),
                Token::Plus,
                Token::Number(String::from("3.0")),
            ]),
            Some(Node::Program(vec![Node::Addition(
                Box::new(Node::Addition(
                    Box::new(Node::Number(Token::Number(String::from("2"),),),),
                    Box::new(Node::Number(Token::Number(String::from("2.0"),),),),
                ),),
                Box::new(Node::Number(Token::Number(String::from("3.0"),),),),
            ),],),)
        );
    }

    #[test]
    fn product() {
        assert_eq!(
            parse(vec![
                Token::Number(String::from("2")),
                Token::Times,
                Token::Number(String::from("2.0")),
            ]),
            Some(Node::Program(vec![Node::Multiplication(
                Box::new(Node::Number(Token::Number(String::from("2"),),),),
                Box::new(Node::Number(Token::Number(String::from("2.0"),),),),
            ),],),)
        );
    }

    #[test]
    fn two_products() {
        assert_eq!(
            parse(vec![
                Token::Number(String::from("2")),
                Token::Times,
                Token::Number(String::from("2.0")),
                Token::Times,
                Token::Number(String::from("3.0")),
            ]),
            Some(Node::Program(vec![Node::Multiplication(
                Box::new(Node::Multiplication(
                    Box::new(Node::Number(Token::Number(String::from("2"),),),),
                    Box::new(Node::Number(Token::Number(String::from("2.0"),),),),
                ),),
                Box::new(Node::Number(Token::Number(String::from("3.0"),),),),
            ),],),)
        );
    }

    #[test]
    fn product_and_sum() {
        assert_eq!(
            parse(vec![
                Token::Number(String::from("2")),
                Token::Plus,
                Token::Number(String::from("2")),
                Token::Times,
                Token::Number(String::from("2")),
            ]),
            Some(Node::Program(vec![Node::Addition(
                Box::new(Node::Number(Token::Number(String::from("2"),),),),
                Box::new(Node::Multiplication(
                    Box::new(Node::Number(Token::Number(String::from("2"),),),),
                    Box::new(Node::Number(Token::Number(String::from("2"),),),),
                ),),
            ),],),)
        );
    }

    #[test]
    fn sum_and_product() {
        assert_eq!(
            parse(vec![
                Token::Number(String::from("2")),
                Token::Times,
                Token::Number(String::from("2")),
                Token::Plus,
                Token::Number(String::from("2")),
            ]),
            Some(Node::Program(vec![Node::Addition(
                Box::new(Node::Multiplication(
                    Box::new(Node::Number(Token::Number(String::from("2"),),),),
                    Box::new(Node::Number(Token::Number(String::from("2"),),),),
                ),),
                Box::new(Node::Number(Token::Number(String::from("2"),),),),
            ),],),)
        );
    }
}
