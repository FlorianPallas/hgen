use super::{map::OrderedHashMap, parser::parse_schema};

#[derive(Debug, Clone, PartialEq)]
pub struct Schema<'a> {
    pub models: OrderedHashMap<&'a str, Model<'a>>,
    pub services: OrderedHashMap<&'a str, Service<'a>>,
}

impl<'a> Schema<'a> {
    pub fn parse(source: &'a str) -> Self {
        parse_schema(source)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Model<'a> {
    Struct(Struct<'a>),
    Enum(Enum<'a>),
    Alias(Alias<'a>),
    External(External<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Service<'a> {
    pub methods: OrderedHashMap<&'a str, Annotated<'a, ServiceMethod<'a>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceMethod<'a> {
    pub inputs: OrderedHashMap<&'a str, Shape<'a>>,
    pub output: Option<Shape<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alias<'a> {
    pub shape: Annotated<'a, Shape<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct External<'a> {
    pub shape: Annotated<'a, Shape<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct<'a> {
    pub fields: OrderedHashMap<&'a str, Annotated<'a, Shape<'a>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum<'a> {
    pub fields: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Shape<'a> {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    String,
    List(Box<Shape<'a>>),
    Map(Box<Shape<'a>>, Box<Shape<'a>>),
    Reference(&'a str),
    Nullable(Box<Shape<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotated<'a, T> {
    pub metadata: OrderedHashMap<&'a str, Literal<'a>>,
    pub inner: T,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(&'a str),
    Object(OrderedHashMap<&'a str, Literal<'a>>),
    Array(Vec<Literal<'a>>),
}
