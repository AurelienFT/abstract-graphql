use crate::{Token, TokenType, extract_scope};

#[derive(Clone, Debug)]
pub struct Field {
    pub field_name: Token,
    pub field_type: Token,
    pub is_needed: bool
}

pub struct Type {
    pub type_name: String,
    pub fields: Vec<Field>
}

fn extract_fields(
    tokens: Vec<Token>
) -> Result<Vec<Field>, String> {
    let mut fields : Vec<Field> = Vec::new();
    for i in 0..tokens.len() - 1 {
        if tokens[i].ty == TokenType::FIELD_NAME && tokens[i + 1].ty == TokenType::FIELD_TYPE {
            if i + 2 < tokens.len() && tokens[i + 2].ty == TokenType::KEYWORD_NEEDED {
                fields.push(
                    Field {
                        field_name: tokens[i].clone(),
                        field_type: tokens[i + 1].clone(),
                        is_needed: true
                    }
                );
            } else {
                fields.push(
                    Field {
                        field_name: tokens[i].clone(),
                        field_type: tokens[i + 1].clone(),
                        is_needed: false
                    }
                );
            }
        }
    }
    Ok(fields)
}

pub fn extract_types(
    tokens: Vec<Token>
) -> Result<Vec<Type>, String> {
    let mut types : Vec<Type> = Vec::new();
    for i in 0..tokens.len() {
        if tokens[i].ty == TokenType::TYPE_NAME {
            let scope = extract_scope(tokens[i+1..].to_vec())?;
            println!("sended to extract fields = {:#?}", scope);
            let fields = extract_fields(scope)?;
            println!("fields = {:#?}", fields);
            types.push(
                Type {
                    type_name: tokens[i].text.clone(),
                    fields: fields
                }
            )
        }
    }
    Ok(types)
}