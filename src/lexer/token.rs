#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Equal,
    Plus,
    Number(String),
}

impl Token {
    pub fn start_of(character: char) -> Option<Token> {
        if character == '=' {
            Some(Token::Equal)
        } else if character == '+' {
            Some(Token::Plus)
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
            Token::Equal => Some(false),
            Token::Plus => Some(false),
            Token::Identifier(_) => Some(character.is_alphabetic() || character.is_digit(10) || character == '_'),
            Token::Number(string) => {
                if character.is_digit(10) {
                    Some(true)
                } else if string.contains(".") && character == '.' {
                    None
                } else {
                    Some(character == '.')
                }
            }
        }
    }

    pub fn append(&mut self, character: char) {
        match self {
            Token::Equal => panic!("Cannot append to ="),
            Token::Plus => panic!("Cannot append to +"),
            Token::Identifier(string) => string.push(character),
            Token::Number(string) => string.push(character),
        }
    }
}
