use crate::schema::*;

pub fn emit_schema(schema: &Schema) -> String {
    let mut output = String::new();

    output.push_str(
        &schema
            .objects
            .iter()
            .map(|model| emit_object(schema, model))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    output
}

fn emit_object(schema: &Schema, object: &Object) -> String {
    let mut output = String::new();

    output.push_str(&format!("pub struct {} ", &object.name));
    output.push_str("{\n");
    object.fields.iter().for_each(|field| {
        output.push_str(&format!(
            "    pub {}: {},\n",
            camel_to_snake(&field.name),
            emit_shape(schema, &field.shape)
        ));
    });
    output.push_str("}\n");

    output
}

fn emit_shape(schema: &Schema, shape: &Shape) -> String {
    match shape {
        Shape::Primitive(def) => emit_simple_shape(schema, def),
        Shape::Nullable(inner) => format!("Option<{}>", emit_shape(schema, inner)),
        Shape::List(inner) => format!("Vec<{}>", emit_simple_shape(schema, inner)),
        Shape::Set(inner) => format!("Set<{}>", emit_simple_shape(schema, inner)),
        Shape::Map(key, value) => format!(
            "Map<{}, {}>",
            emit_simple_shape(schema, key),
            emit_simple_shape(schema, value)
        ),
    }
}

fn emit_simple_shape(schema: &Schema, shape: &SimpleShape) -> String {
    match shape {
        SimpleShape::Bool { .. } => "bool".to_owned(),
        SimpleShape::Int32 { .. } => "i32".to_owned(),
        SimpleShape::Int64 { .. } => "i64".to_owned(),
        SimpleShape::Float32 { .. } => "f32".to_owned(),
        SimpleShape::Float64 { .. } => "f64".to_owned(),
        SimpleShape::String { .. } => "String".to_owned(),
        SimpleShape::Ref(name) => schema.resolve(&name).unwrap().name().to_owned(),
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
