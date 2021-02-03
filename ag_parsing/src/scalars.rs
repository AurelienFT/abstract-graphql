use crate::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Scalar {
    pub scalar_name: Token,
}

pub fn extract_scalars(tokens: Vec<Token>) -> Result<Vec<Scalar>, String> {
    let mut scalars: Vec<Scalar> = Vec::new();
    for token in tokens {
        if token.ty == TokenType::SCALAR_NAME {
            scalars.push(Scalar { scalar_name: token });
        }
    }
    Ok(scalars)
}
