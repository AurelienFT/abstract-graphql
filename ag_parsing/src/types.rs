pub enum BuiltinTypes {
    Int,
    Float,
    String,
    Boolean,
    ID,
    ComplexType
}

pub struct Field {
    pub field_name: String,
    pub field_type: BuiltinTypes
}

pub struct Type {
    pub type_name: String,
    pub fields: Vec<Field>
}

pub fn extract_types(
    content: String
) -> Result<Vec<Type>, String> {
    
    Err("WIP".to_owned())
}