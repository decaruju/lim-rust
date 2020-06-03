use super::token::Token;

pub fn lex(code: &str) -> Option<Vec<Token>> {
    let mut characters: Vec<char> = code.chars().collect();
    let mut tokens = vec![];
    while !characters.is_empty() {
        let character = characters.remove(0);
        if let Some(mut token) = Token::start_of(character) {
            while !characters.is_empty() && token.continues(characters[0])? {
                token.append(characters.remove(0));
            }
            if token.is_complete() {
                token.clean();
                tokens.push(token);
            } else {
                return None;
            }
        }
    }
    Some(tokens)
}
