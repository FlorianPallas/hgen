use super::map::OrderedHashMap;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub imports: Vec<String>,
    pub models: OrderedHashMap<String, Model>,
    pub services: OrderedHashMap<String, Service>,
}

impl Default for Schema {
    fn default() -> Self {
        Self {
            imports: Default::default(),
            models: Default::default(),
            services: Default::default(),
        }
    }
}

impl Schema {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(&mut self, other: Schema) {
        self.imports.extend(other.imports);
        self.models.extend(other.models);
        self.services.extend(other.services);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub methods: Vec<Annotated<Method>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub inputs: OrderedHashMap<String, Annotated<Shape>>,
    pub output: Annotated<Shape>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Model {
    Struct(Struct),
    Enum(Enum),
    Alias(Alias),
    External(External),
}

impl ToString for Model {
    fn to_string(&self) -> String {
        match self {
            Model::Struct(_) => "Struct",
            Model::Enum(_) => "Enum",
            Model::Alias(_) => "Alias",
            Model::External(_) => "External",
        }
        .to_owned()
    }
}

impl From<Struct> for Model {
    fn from(value: Struct) -> Self {
        Model::Struct(value)
    }
}

impl From<Enum> for Model {
    fn from(value: Enum) -> Self {
        Model::Enum(value)
    }
}

impl From<Alias> for Model {
    fn from(value: Alias) -> Self {
        Model::Alias(value)
    }
}

impl From<External> for Model {
    fn from(value: External) -> Self {
        Model::External(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct {
    pub fields: OrderedHashMap<String, Annotated<Shape>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enum {
    pub values: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Annotated<T> {
    pub inner: T,
    pub metadata: OrderedHashMap<String, String>,
}

impl Deref for Annotated<Shape> {
    type Target = Shape;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Annotated<Shape> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alias {
    pub shape: Annotated<Shape>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct External {
    pub shape: Annotated<Shape>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Primitive(Primitive),
    Nullable(Box<Shape>),
    List(Box<Shape>),
    Map(Box<Shape>, Box<Shape>),
    Reference(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Primitive {
    Unit,
    String,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Float32,
    Float64,
}

impl ToString for Primitive {
    fn to_string(&self) -> String {
        match self {
            Primitive::Unit => "Unit",
            Primitive::String => "String",
            Primitive::Bool => "Bool",
            Primitive::Int8 => "Int8",
            Primitive::Int16 => "Int16",
            Primitive::Int32 => "Int32",
            Primitive::Int64 => "Int64",
            Primitive::Int128 => "Int128",
            Primitive::Float32 => "Float32",
            Primitive::Float64 => "Float64",
        }
        .to_owned()
    }
}

impl TryFrom<&str> for Primitive {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Unit" => Ok(Primitive::Unit),
            "String" => Ok(Primitive::String),
            "Bool" => Ok(Primitive::Bool),
            "Int8" => Ok(Primitive::Int8),
            "Int16" => Ok(Primitive::Int16),
            "Int32" => Ok(Primitive::Int32),
            "Int64" => Ok(Primitive::Int64),
            "Int128" => Ok(Primitive::Int128),
            "Float32" => Ok(Primitive::Float32),
            "Float64" => Ok(Primitive::Float64),
            _ => Err("Invalid primitive"),
        }
    }
}
