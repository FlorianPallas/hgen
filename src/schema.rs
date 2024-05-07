use std::collections::HashMap;

#[derive(Debug)]
pub struct Schema {
    pub name: String,
    pub objects: Vec<Object>,
    pub custom_shapes: Vec<CustomShape>,
}

impl Schema {
    pub fn resolve(&self, name: &str) -> Option<Reference> {
        let object = self.objects.iter().find(|o| o.name == name);
        if let Some(target) = object {
            return Some(Reference::Object(target));
        }

        let custom = self.custom_shapes.iter().find(|c| c.name == name);
        if let Some(target) = custom {
            return Some(Reference::Custom(target));
        }

        return None;
    }
}

pub enum Reference<'a> {
    Object(&'a Object),
    Custom(&'a CustomShape),
}

impl Reference<'_> {
    pub fn name(&self) -> &str {
        match self {
            Reference::Object(o) => &o.name,
            Reference::Custom(c) => &c.name,
        }
    }
}

#[derive(Debug)]
pub struct Object {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub shape: Shape,
    pub data: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Simple(SimpleShape),
    Nullable(Box<Shape>),
    List(Box<SimpleShape>),
    Set(Box<SimpleShape>),
    Map(Box<SimpleShape>, Box<SimpleShape>),
}

#[derive(Debug, PartialEq)]
pub enum SimpleShape {
    String,
    Bool,
    Int32,
    Int64,
    Float32,
    Float64,
    Ref(String),
}

impl SimpleShape {
    pub fn from_str(name: &str) -> Self {
        match name {
            "String" => SimpleShape::String,
            "Bool" => SimpleShape::Bool,
            "Int32" => SimpleShape::Int32,
            "Int64" => SimpleShape::Int64,
            "Float32" => SimpleShape::Float32,
            "Float64" => SimpleShape::Float64,
            _ => SimpleShape::Ref(name.to_owned()),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            SimpleShape::String => "String",
            SimpleShape::Bool => "Bool",
            SimpleShape::Int32 => "Int32",
            SimpleShape::Int64 => "Int64",
            SimpleShape::Float32 => "Float32",
            SimpleShape::Float64 => "Float64",
            SimpleShape::Ref(name) => name,
        }
    }
}

#[derive(Debug)]
pub struct CustomShape {
    pub name: String,
}
