#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(String),
    Equal,
    Plus,
    Minus,
    Times,
    Division,
    Modulus,
    OpenParenthesis,
    CloseParenthesis,
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
        } else if character == '(' {
            Some(Token::OpenParenthesis)
        } else if character == ')' {
            Some(Token::CloseParenthesis)
        } else if character.is_digit(10) || character == '.' {
            Some(Token::Number(character.to_string()))
        } else if character.is_alphabetic() || character == '_' {
            Some(Token::Identifier(character.to_string()))
        } else {
            None
        }
    }

    pub fn continues(&self, character: char) -> Option<bool> {
        match self {
            Token::Identifier(_) => Some(character.is_alphabetic() || character.is_digit(10) || character == '_'),
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
            _ => unreachable!(),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            Token::Number(string) => {
                string.len() > 1 || string.chars().next().unwrap() != '.'
            }
            _ => true
        }
    }
}
