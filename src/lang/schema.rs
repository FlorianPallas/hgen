use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub models: Vec<Model>,
    pub services: Vec<Service>,
}

impl Schema {
    pub fn resolve(&self, name: &str) -> Option<&str> {
        self.models
            .iter()
            .find(|m| m.name() == name)
            .map(|m| m.name())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub inputs: Vec<(String, Type)>,
    pub output: Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Model {
    Struct(Struct),
    Enum(Enum),
    Alias(Alias),
    External(External),
}

impl Model {
    pub fn name(&self) -> &str {
        match self {
            Model::Struct(s) => &s.name,
            Model::Enum(e) => &e.name,
            Model::Alias(a) => &a.name,
            Model::External(c) => &c.name,
        }
    }
}

impl ToString for Model {
    fn to_string(&self) -> String {
        match self {
            Model::Struct(_) => "Struct".to_string(),
            Model::Enum(_) => "Enum".to_string(),
            Model::Alias(_) => "Alias".to_string(),
            Model::External(_) => "External".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    pub shape: Shape,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alias {
    pub name: String,
    pub def: Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct External {
    pub name: String,
    pub def: Type,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Primitive(Primitive),
    Nullable(Box<Shape>),
    List(Box<Shape>),
    Set(Box<Shape>),
    Map(Box<Shape>, Box<Shape>),
    Reference(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Primitive {
    String,
    Bool,
    Int32,
    Int64,
    Float32,
    Float64,
}

impl ToString for Primitive {
    fn to_string(&self) -> String {
        match self {
            Primitive::String => "String".to_string(),
            Primitive::Bool => "Bool".to_string(),
            Primitive::Int32 => "Int32".to_string(),
            Primitive::Int64 => "Int64".to_string(),
            Primitive::Float32 => "Float32".to_string(),
            Primitive::Float64 => "Float64".to_string(),
        }
    }
}
