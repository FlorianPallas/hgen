use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum TypeDef {
    Simple(SimpleTypeDef),
    List(Box<SimpleTypeDef>),
    Set(Box<SimpleTypeDef>),
    Map(Box<SimpleTypeDef>, Box<SimpleTypeDef>),
}

#[derive(Debug, PartialEq)]
pub enum SimpleTypeDef {
    String,
    Bool,
    Int32,
    Int64,
    Float32,
    Float64,
    Instant,
    Ref(String),
}

impl SimpleTypeDef {
    pub fn from_str(name: &str) -> Self {
        match name {
            "String" => SimpleTypeDef::String,
            "Bool" => SimpleTypeDef::Bool,
            "Int32" => SimpleTypeDef::Int32,
            "Int64" => SimpleTypeDef::Int64,
            "Float32" => SimpleTypeDef::Float32,
            "Float64" => SimpleTypeDef::Float64,
            "Instant" => SimpleTypeDef::Instant,
            _ => SimpleTypeDef::Ref(name.to_owned()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldDef {
    pub name: String,
    pub type_def: TypeDef,
    pub nullable: bool,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug)]
pub struct ObjectDef {
    pub name: String,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug)]
pub struct EnumDef {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug)]
pub struct ModuleDef {
    pub name: String,
    pub objects: Vec<ObjectDef>,
    pub enums: Vec<EnumDef>,
}

impl ModuleDef {
    pub fn resolve(&self, name: &str) -> String {
        self.objects
            .iter()
            .map(|o| o.name.clone())
            .chain(self.enums.iter().map(|o| o.name.clone()))
            .find(|m| m == name)
            .expect(format!("Could not resolve {}", name).as_str())
    }
}
