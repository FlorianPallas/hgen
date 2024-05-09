use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub objects: Vec<Struct>,
    pub enums: Vec<Enum>,
    pub aliases: Vec<Alias>,
    pub externals: Vec<External>,
}

impl Schema {
    pub fn resolve(&self, name: &str) -> Option<Reference> {
        let struct_type = self.objects.iter().find(|o| o.name == name);
        if let Some(target) = struct_type {
            return Some(Reference::Struct(target));
        }

        let enum_type = self.enums.iter().find(|e| e.name == name);
        if let Some(target) = enum_type {
            return Some(Reference::Enum(target));
        }

        let alias_type = self.aliases.iter().find(|a| a.name == name);
        if let Some(target) = alias_type {
            return Some(Reference::Alias(target));
        }

        let external_type = self.externals.iter().find(|c| c.name == name);
        if let Some(target) = external_type {
            return Some(Reference::Extern(target));
        }

        return None;
    }
}

pub enum Reference<'a> {
    Struct(&'a Struct),
    Enum(&'a Enum),
    Extern(&'a External),
    Alias(&'a Alias),
}

impl Reference<'_> {
    pub fn name(&self) -> &str {
        match self {
            Reference::Struct(o) => &o.name,
            Reference::Enum(e) => &e.name,
            Reference::Extern(c) => &c.name,
            Reference::Alias(a) => &a.name,
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
    pub fields: Vec<(String, ())>,
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
