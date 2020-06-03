#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(String),
    Literal(String, char),
    Equal,
    Plus,
    Minus,
    Times,
    Division,
    Modulus,
    Comma,
    Period,
    SemiColon,
    NewLine,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
}

impl Token {
    pub fn start_of(character: char) -> Option<Token> {
        if character == '=' {
            Some(Token::Equal)
        } else if character == '+' {
            Some(Token::Plus)
        } else if character == '-' {
            Some(Token::Minus)
        } else if character == '*' {
            Some(Token::Times)
        } else if character == '/' {
            Some(Token::Division)
        } else if character == '%' {
            Some(Token::Modulus)
        } else if character == ',' {
            Some(Token::Comma)
        } else if character == '.' {
            Some(Token::Period)
        } else if character == ';' {
            Some(Token::SemiColon)
        } else if character == '\n' {
            Some(Token::NewLine)
        } else if character == '(' {
            Some(Token::OpenParenthesis)
        } else if character == ')' {
            Some(Token::CloseParenthesis)
        } else if character == '{' {
            Some(Token::OpenBrace)
        } else if character == '}' {
            Some(Token::CloseBrace)
        } else if character == '[' {
            Some(Token::OpenBracket)
        } else if character == ']' {
            Some(Token::CloseBracket)
        } else if character == '"' || character == '\'' {
            Some(Token::Literal(character.to_string(), character))
        } else if character.is_digit(10) {
            Some(Token::Number(character.to_string()))
        } else if character.is_alphabetic() || character == '_' {
            Some(Token::Identifier(character.to_string()))
        } else {
            None
        }
    }

    pub fn continues(&self, character: char) -> Option<bool> {
        match self {
            Token::Period => Some(character.is_digit(10)),
            Token::Identifier(_) => {
                Some(character.is_alphabetic() || character.is_digit(10) || character == '_')
            }
            Token::Literal(string, delimiter) => {
                let escaped_delimiter = format!("\\{}", delimiter);
                Some(
                    string.matches(*delimiter).count() - string.matches(&escaped_delimiter).count()
                        != 2,
                )
            }
            Token::Number(string) => {
                if character.is_digit(10) {
                    Some(true)
                } else if string.contains(".") {
                    if character == '.' || string.len() == 1 {
                        None
                    } else {
                        Some(false)
                    }
                } else {
                    Some(character == '.')
                }
            }
            _ => Some(false),
        }
    }

    pub fn append(&mut self, character: char) {
        match self {
            Token::Identifier(string) => string.push(character),
            Token::Number(string) => string.push(character),
            Token::Literal(string, _) => string.push(character),
            Token::Period => {
                *self = Token::Number(String::from(format!(".{}", character)));
            }
            _ => unreachable!(),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            Token::Number(string) => string.len() > 1 || string.chars().next().unwrap() != '.',
            Token::Literal(string, delimiter) => {
                let escaped_delimiter = format!("\\{}", delimiter);
                string.matches(*delimiter).count() - string.matches(&escaped_delimiter).count() == 2
            }
            _ => true,
        }
    }

    pub fn clean(&mut self) {
        match self {
            Token::Literal(string, delimiter) => {
                string.remove(0);
                string.remove(string.len() - 1);
                let escaped_delimiter = format!("\\{}", delimiter);
                *string = string.replace(&escaped_delimiter, &delimiter.to_string());
            }
            _ => {}
        }
    }
}
