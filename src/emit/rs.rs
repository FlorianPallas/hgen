use crate::lang::schema::*;

pub fn emit_schema(_name: &str, schema: &Schema) -> String {
    let mut output = String::new();

    output.push_str("use serde::{Serialize, Deserialize};\n");

    output.push_str(
        &schema
            .models
            .iter()
            .map(|(name, def)| emit_model(name, def))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    output
}

fn emit_model(name: &str, def: &Model) -> String {
    match def {
        Model::Struct(inner) => emit_struct(name, inner),
        Model::Enum(inner) => emit_enum(name, inner),
        Model::Alias(inner) => format!("pub type {} = {};\n", name, emit_shape(&inner.shape.inner)),
        Model::External(_) => format!("use external::{};\n", name),
    }
}

fn emit_struct(name: &str, def: &Struct) -> String {
    let mut output = String::new();

    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    output.push_str(&format!("pub struct {} ", name));
    output.push_str("{\n");
    def.fields.iter().for_each(|(name, shape)| {
        output.push_str(&format!(
            "    pub {}: {},\n",
            name.to_snake_case(),
            emit_shape(&shape.inner)
        ));
    });
    output.push_str("}\n");

    output
}

fn emit_enum(name: &str, def: &Enum) -> String {
    let mut output = String::new();

    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    output.push_str(&format!("pub enum {} ", name));
    output.push_str("{\n");
    def.fields.iter().for_each(|value| {
        output.push_str(&format!("    {},\n", value,));
    });
    output.push_str("}\n");

    output
}

fn emit_shape(def: &Shape) -> String {
    match def {
        Shape::Bool => "bool".to_owned(),
        Shape::Int8 => "i8".to_owned(),
        Shape::Int16 => "i16".to_owned(),
        Shape::Int32 => "i32".to_owned(),
        Shape::Int64 => "i64".to_owned(),
        Shape::Float32 => "f32".to_owned(),
        Shape::Float64 => "f64".to_owned(),
        Shape::String => "String".to_owned(),
        Shape::Nullable(inner) => format!("Option<{}>", emit_shape(inner)),
        Shape::List(inner) => format!("Vec<{}>", emit_shape(inner)),
        Shape::Map(key, value) => format!("Map<{}, {}>", emit_shape(key), emit_shape(value)),
        Shape::Reference(name) => (*name).to_owned(),
    }
}

pub trait ToSnakeCase {
    fn to_snake_case(&self) -> String;
}

impl ToSnakeCase for str {
    fn to_snake_case(&self) -> String {
        let mut result = String::new();
        for (i, c) in self.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
            } else {
                result.push(c);
            }
        }
        result
    }
}

impl ToSnakeCase for String {
    fn to_snake_case(&self) -> String {
        self.as_str().to_snake_case()
    }
}
