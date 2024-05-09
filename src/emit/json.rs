use crate::lang::schema::*;

pub fn emit_schema(_name: &str, schema: &Schema) -> String {
    let mut output = String::new();

    output.push_str("{");

    // emit models
    output.push_str(&format!(
        "\"models\":{{{}}}",
        schema
            .models
            .iter()
            .map(emit_model)
            .collect::<Vec<_>>()
            .join(",")
    ));

    output.push_str("}");

    output
}

fn emit_model(model: &Model) -> String {
    match model {
        Model::Struct(inner) => emit_struct(inner),
        Model::Enum(inner) => emit_enum(inner),
        Model::Alias(inner) => emit_alias(inner),
        Model::External(inner) => emit_external(inner),
    }
}

fn emit_struct(def: &Struct) -> String {
    let mut output = String::new();

    output.push_str(&format!("\"{}\":{{", def.name));
    output.push_str("\"type\":\"struct\",");

    output.push_str(&format!(
        "\"fields\":{{{}}}",
        def.fields
            .iter()
            .map(|(name, field)| { format!("\"{}\":{}", name, emit_shape(&field.shape)) })
            .collect::<Vec<_>>()
            .join(",")
    ));

    output.push_str("}");

    output
}

fn emit_enum(def: &Enum) -> String {
    let mut output = String::new();

    output.push_str(&format!("\"{}\":{{", def.name));
    output.push_str("\"type\":\"enum\",");

    output.push_str(&format!(
        "\"fields\":[{}]",
        def.values
            .iter()
            .map(|name| { format!("\"{}\"", name) })
            .collect::<Vec<_>>()
            .join(",")
    ));

    output.push_str("}");

    output
}

fn emit_alias(alias: &Alias) -> String {
    format!(
        "\"{}\":{{\"type\":\"alias\",\"inner\":{}}}",
        alias.name,
        emit_shape(&alias.def.shape)
    )
}

fn emit_external(external: &External) -> String {
    format!(
        "\"{}\":{{\"type\":\"external\",\"inner\":{}}}",
        external.name,
        emit_shape(&external.def.shape)
    )
}

fn emit_shape(shape: &Shape) -> String {
    match shape {
        Shape::Primitive(primitive) => format!(
            "{{\"type\":\"{}\"}}",
            match primitive {
                Primitive::Bool { .. } => "bool",
                Primitive::Int32 { .. } => "int32",
                Primitive::Int64 { .. } => "int64",
                Primitive::Float32 { .. } => "float32",
                Primitive::Float64 { .. } => "float64",
                Primitive::String { .. } => "string",
            }
        ),
        Shape::Nullable(inner) => {
            format!("{{\"type\":\"nullable\",\"inner\":{}}}", emit_shape(inner))
        }
        Shape::List(inner) => format!("{{\"type\":\"list\",\"inner\":{}}}", emit_shape(inner)),
        Shape::Set(inner) => format!("{{\"type\":\"set\",\"inner\":{}}}", emit_shape(inner)),
        Shape::Map(key, value) => format!(
            "{{\"type\":\"map\",\"key\":{},\"value\":{}}}",
            emit_shape(key),
            emit_shape(value),
        ),
        Shape::Reference(name) => format!("{{\"type\":\"reference\",\"name\":\"{}\"}}", name),
    }
}
