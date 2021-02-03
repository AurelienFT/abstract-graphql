use crate::{extract_scope, is_scalar, scalars::Scalar, Token, TokenType};

#[derive(Clone, Debug)]
pub struct Field {
    pub field_name: Token,
    pub field_type: Token,
    pub is_needed: bool,
    pub is_scalar: bool,
}

pub struct Type {
    pub type_name: String,
    pub fields: Vec<Field>,
}

fn extract_fields(tokens: Vec<Token>, scalars: Vec<Scalar>) -> Result<Vec<Field>, String> {
    let mut fields: Vec<Field> = Vec::new();
    for i in 0..tokens.len() - 1 {
        if tokens[i].ty == TokenType::FIELD_NAME && tokens[i + 1].ty == TokenType::FIELD_TYPE {
            if i + 2 < tokens.len() && tokens[i + 2].ty == TokenType::KEYWORD_NEEDED {
                fields.push(Field {
                    field_name: tokens[i].clone(),
                    field_type: tokens[i + 1].clone(),
                    is_needed: true,
                    is_scalar: is_scalar(tokens[i + 1].clone(), scalars.clone()),
                });
            } else {
                fields.push(Field {
                    field_name: tokens[i].clone(),
                    field_type: tokens[i + 1].clone(),
                    is_needed: false,
                    is_scalar: is_scalar(tokens[i + 1].clone(), scalars.clone()),
                });
            }
        }
    }
    Ok(fields)
}

pub fn extract_types(tokens: Vec<Token>, scalars: Vec<Scalar>) -> Result<Vec<Type>, String> {
    let mut types: Vec<Type> = Vec::new();
    for i in 0..tokens.len() {
        if tokens[i].ty == TokenType::TYPE_NAME {
            let scope = extract_scope(tokens[i + 1..].to_vec())?;
            let fields = extract_fields(scope, scalars.clone())?;
            types.push(Type {
                type_name: tokens[i].text.clone(),
                fields: fields,
            })
        }
    }
    Ok(types)
}
