use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub imports: Vec<String>,
    pub models: HashMap<String, Model>,
    pub services: HashMap<String, Service>,
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
    pub methods: Vec<Method>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub inputs: Vec<(String, Annotated<Shape>)>,
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
    pub fields: Vec<(String, Annotated<Shape>)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enum {
    pub values: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Annotated<T> {
    inner: T,
    pub data: HashMap<String, String>,
}

impl<T> Annotated<T> {
    pub fn new(inner: T, data: HashMap<String, String>) -> Self {
        Self { inner, data }
    }
}

impl<T: Default> Default for Annotated<T> {
    fn default() -> Self {
        Self {
            inner: T::default(),
            data: Default::default(),
        }
    }
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
    Set(Box<Shape>),
    Map(Box<Shape>, Box<Shape>),
    Reference(String),
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        match self {
            Shape::Primitive(inner) => format!("Primitive({})", inner.to_string()),
            Shape::Nullable(inner) => format!("Nullable({})", inner.to_string()),
            Shape::List(inner) => format!("List({})", inner.to_string()),
            Shape::Set(inner) => format!("Set({})", inner.to_string()),
            Shape::Map(key, value) => format!("Map({}, {})", key.to_string(), value.to_string()),
            Shape::Reference(inner) => format!("Reference({})", inner),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Primitive {
    Unit,
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
            Primitive::Unit => "Unit",
            Primitive::String => "String",
            Primitive::Bool => "Bool",
            Primitive::Int32 => "Int32",
            Primitive::Int64 => "Int64",
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
            "Int32" => Ok(Primitive::Int32),
            "Int64" => Ok(Primitive::Int64),
            "Float32" => Ok(Primitive::Float32),
            "Float64" => Ok(Primitive::Float64),
            _ => Err("Invalid primitive"),
        }
    }
}
