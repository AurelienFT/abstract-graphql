use std::fs::File;
use std::io::prelude::*;

pub fn write_typescript(
    tokens: Vec<ag_parsing::Token>
) -> Result<(), String> {
    //TODO : Add custom scalars
    let mut content_typescript : String = "".to_owned();
    let mut content_types : String = "".to_owned();
    let mut content_scalars : String = "
export type Maybe<T> = T | null;

/* export basic scalars */
export type Scalars = {
    ID: string;
    String: string;
    Boolean: boolean;
    Int: number;
    Float: number;
".to_owned();
    let types = ag_parsing::types::extract_types(tokens.clone())?;
    //move in a scalar extractor
    for token in tokens {
        match token.ty {
            ag_parsing::TokenType::SCALAR_NAME => {
                content_scalars.push_str(&format!("    {}: any;\n", token.text));
            }
            _ => {

            }
        }
    }
    for ty in types {
        content_types.push_str(&format!("export type {} = {{ \n", ty.type_name));
        for field in ty.fields {
            if field.is_needed {
                content_types.push_str(&format!("    {}: Scalars['{}'];\n", field.field_name.text, field.field_type.text));
            } else {
                content_types.push_str(&format!("    {}?: Maybe<Scalars['{}']>;\n", field.field_name.text, field.field_type.text));
            }
        }
        content_types.push_str("};\n");
    }
    content_scalars.push_str("};\n\n");
    content_typescript.push_str(content_scalars.as_str());
    content_typescript.push_str(content_types.as_str());
    let mut file = File::create("index.ts").unwrap();
    file.write_all(content_typescript.as_bytes()).unwrap();
    Ok(())
}