use crate::lang::schema::*;

pub fn emit_schema(_name: &str, schema: &Schema) -> String {
    let mut output = String::new();

    output.push_str(
        &schema
            .models
            .iter()
            .map(|model| match model {
                Model::Struct(inner) => emit_struct(schema, inner),
                _ => "/* UNSUPPORTED */".to_owned(),
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );

    output
}

fn emit_struct(schema: &Schema, def: &Struct) -> String {
    let mut output = String::new();

    output.push_str(&format!("pub struct {} ", &def.name));
    output.push_str("{\n");
    def.fields.iter().for_each(|(name, field)| {
        output.push_str(&format!(
            "    pub {}: {},\n",
            camel_to_snake(&name),
            emit_shape(schema, &field.shape)
        ));
    });
    output.push_str("}\n");

    output
}

fn emit_shape(schema: &Schema, shape: &Shape) -> String {
    match shape {
        Shape::Primitive(primitive) => match primitive {
            Primitive::Bool { .. } => "bool".to_owned(),
            Primitive::Int32 { .. } => "i32".to_owned(),
            Primitive::Int64 { .. } => "i64".to_owned(),
            Primitive::Float32 { .. } => "f32".to_owned(),
            Primitive::Float64 { .. } => "f64".to_owned(),
            Primitive::String { .. } => "String".to_owned(),
        },
        Shape::Nullable(inner) => format!("Option<{}>", emit_shape(schema, inner)),
        Shape::List(inner) => format!("Vec<{}>", emit_shape(schema, inner)),
        Shape::Set(inner) => format!("Set<{}>", emit_shape(schema, inner)),
        Shape::Map(key, value) => format!(
            "Map<{}, {}>",
            emit_shape(schema, key),
            emit_shape(schema, value)
        ),
        Shape::Reference(name) => schema.resolve(&name).unwrap().to_owned(),
    }
}

fn camel_to_snake(name: &str) -> String {
    let mut result = String::new();
    for (i, c) in name.chars().enumerate() {
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
