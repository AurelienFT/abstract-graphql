use std::fs::File;
use std::io::prelude::*;

pub fn write_typescript(
    tokens: Vec<ag_parsing::Token>
) -> Result<String, String> {
    //TODO : Add custom scalars
    let mut content_typescript : String = "".to_owned();
    let mut content_types : String = "".to_owned();
    let mut content_scalars : String = "
/* export basic scalars */
export type Scalars = {
    ID: string;
    String: string;
    Boolean: boolean;
    Int: number;
    Float: number;
".to_owned();
    let mut in_scope : bool = false;
    for token in tokens {
        //Want to do with an tuple and or but the feature is still experimental (https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)
        match token.ty {
            ag_parsing::TokenType::SCALAR_NAME => {
                content_scalars.push_str(&format!("    {}: any;\n", token.text));
            }
            ag_parsing::TokenType::TYPE_NAME => {
                in_scope = true;
                content_types.push_str(&format!("export type {} = ", token.text));
            },
            ag_parsing::TokenType::OPEN_BRACKET => {
                content_types.push_str(&format!("{}\n", token.text));
            },
            ag_parsing::TokenType::CLOSE_BRACKET => {
                content_types.push_str(&format!("{};", token.text));
                in_scope = false;
            },
            ag_parsing::TokenType::FIELD_NAME => {
                content_types.push_str(&format!("    {}: ", token.text));
            },
            ag_parsing::TokenType::FIELD_TYPE => {
                content_types.push_str(&format!("Scalars['{}'],\n", token.text))
            },
            _ => {

            }
        }
    }
    content_scalars.push_str("};\n\n");
    content_typescript.push_str(content_scalars.as_str());
    content_typescript.push_str(content_types.as_str());
    let mut file = File::create("index.ts").unwrap();
    file.write_all(content_typescript.as_bytes()).unwrap();
    Ok("WIP".to_owned())
}