mod types;

use regex::Regex;

#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    KEYWORD_TYPE,
    TYPE_NAME,
    KEYWORD_SCALAR,
    SCALAR_NAME,
    OPEN_BRACKET,
    CLOSE_BRACKET,
    FIELD_NAME,
    COLON,
    COMMA,
    FIELD_TYPE,
    UNKNOWN
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub ty: TokenType
}

pub fn tokenize(
    content: String
) -> Result<Vec<Token>, String> {
    let re = Regex::new(r"[a-zA-Z{}]+|:|!|,").unwrap();
    let mut tokens: Vec<Token> = Vec::new();
    let caps = re.captures_iter(&content);
    let mut prev_token : TokenType = TokenType::UNKNOWN;
    for cap in caps {
        let word = cap.get(0).unwrap().as_str();
        match word {
            "scalar" => prev_token = TokenType::KEYWORD_SCALAR,
            "type" => prev_token = TokenType::KEYWORD_TYPE,
            ":" => prev_token = TokenType::COLON,
            "," => prev_token = TokenType::COMMA,
            "{" => prev_token = TokenType::OPEN_BRACKET,
            "}" => prev_token = TokenType::CLOSE_BRACKET,
            _ => {
                match prev_token {
                    TokenType::KEYWORD_SCALAR => {
                        tokens.push(Token { text: word.to_string(), ty: TokenType::SCALAR_NAME });
                        prev_token = TokenType::SCALAR_NAME
                    },
                    TokenType::KEYWORD_TYPE => {
                        tokens.push(Token { text: word.to_string(), ty: TokenType::TYPE_NAME });
                        prev_token = TokenType::TYPE_NAME
                    },
                    TokenType::COLON => {
                        tokens.push(Token { text: word.to_string(), ty: TokenType::FIELD_TYPE });
                        prev_token = TokenType::FIELD_TYPE
                    },
                    TokenType::COMMA | TokenType::OPEN_BRACKET => {
                        tokens.push(Token { text: word.to_string(), ty: TokenType::FIELD_NAME });
                        prev_token = TokenType::FIELD_NAME
                    }
                    _ => {}
                }
            }
        }
    }
    println!("{:#?}", tokens);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
