use std::fs::File;
use std::io::prelude::*;

pub fn write_typescript(tokens: Vec<ag_parsing::Token>) -> Result<(), String> {
    //TODO : Add custom scalars
    let mut content_typescript: String = "".to_owned();
    let mut content_types: String = "".to_owned();
    let mut content_scalars: String = "
export type Maybe<T> = T | null;

/* export basic scalars */
export type Scalars = {
    ID: string;
    String: string;
    Boolean: boolean;
    Int: number;
    Float: number;
"
    .to_owned();
    let scalars = ag_parsing::scalars::extract_scalars(tokens.clone())?;
    let types = ag_parsing::types::extract_types(tokens.clone(), scalars.clone())?;
    for scalar in scalars {
        content_scalars.push_str(&format!("    {}: any;\n", scalar.scalar_name.text));
    }
    for ty in types {
        content_types.push_str(&format!("export type {} = {{ \n", ty.type_name));
        for field in ty.fields {
            println!("field = {:#?}", field);
            content_types.push_str(&format!(
                "    {}: {};\n",
                field.field_name.text,
                if field.is_scalar {
                    if field.is_needed {
                        "Scalars['".to_string() + &field.field_type.text + "']"
                    } else {
                        "Maybe<Scalars['".to_string() + &field.field_type.text + "']>"
                    }
                } else {
                    if field.is_needed {
                        field.field_type.text
                    } else {
                        "Maybe<".to_string() + &field.field_type.text + ">"
                    }
                }
            ));
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
