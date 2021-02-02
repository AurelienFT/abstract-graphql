pub mod types;

use regex::Regex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    SEPARATOR,
    FIELD_TYPE,
    KEYWORD_NEEDED,
    UNKNOWN
}

#[derive(Debug, Clone)]
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
            "{" => {
                tokens.push(Token { text: word.to_string(), ty: TokenType::OPEN_BRACKET });
                prev_token = TokenType::OPEN_BRACKET
            },
            "}" => {
                tokens.push(Token { text: word.to_string(), ty: TokenType::CLOSE_BRACKET });
                prev_token = TokenType::CLOSE_BRACKET
            },
            "!" => {
                tokens.push(Token { text: word.to_string(), ty: TokenType::KEYWORD_NEEDED });
                prev_token = TokenType::KEYWORD_NEEDED
            }
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
                    TokenType::FIELD_TYPE | TokenType::OPEN_BRACKET | TokenType::KEYWORD_NEEDED => {
                        tokens.push(Token { text: word.to_string(), ty: TokenType::FIELD_NAME });
                        prev_token = TokenType::FIELD_NAME
                    },
                    _ => {}
                }
            }
        }
    }
    println!("{:#?}", tokens);
    Ok(tokens)
}

fn extract_scope (
    tokens: Vec<Token>
 ) -> Result<Vec<Token>, String> {
    if tokens.len() == 0 {
        return Err("Extract scope: Vector without any length".to_string());
    }
    if tokens[0].ty != TokenType::OPEN_BRACKET {
        return Err("Extract scope: First character is not an open bracket.".to_string());   
    }
    let close_bracket = tokens.iter().position(|token| token.ty == TokenType::CLOSE_BRACKET);
    match close_bracket {
        Some(bracket) => {
            return Ok(tokens[1..bracket].to_vec());
        }
        None => {
            return Err("Extract scope: Not any closing bracket.".to_string());   
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
