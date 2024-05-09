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

    // emit services
    output.push_str(",");
    output.push_str(&format!(
        "\"services\":{{{}}}",
        schema
            .services
            .iter()
            .map(emit_service)
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
    output.push_str("\"type\":\"Struct\",");

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
    output.push_str("\"type\":\"Enum\",");

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
        "\"{}\":{{\"type\":\"Alias\",\"inner\":{}}}",
        alias.name,
        emit_shape(&alias.def.shape)
    )
}

fn emit_external(external: &External) -> String {
    format!(
        "\"{}\":{{\"type\":\"External\",\"inner\":{}}}",
        external.name,
        emit_shape(&external.def.shape)
    )
}

fn emit_shape(shape: &Shape) -> String {
    match shape {
        Shape::Primitive(primitive) => format!(
            "{{\"type\":\"{}\"}}",
            match primitive {
                Primitive::Bool { .. } => "Bool",
                Primitive::Int32 { .. } => "Int32",
                Primitive::Int64 { .. } => "Int64",
                Primitive::Float32 { .. } => "Float32",
                Primitive::Float64 { .. } => "Float64",
                Primitive::String { .. } => "String",
            }
        ),
        Shape::Nullable(inner) => {
            format!("{{\"type\":\"Nullable\",\"inner\":{}}}", emit_shape(inner))
        }
        Shape::List(inner) => format!("{{\"type\":\"List\",\"inner\":{}}}", emit_shape(inner)),
        Shape::Set(inner) => format!("{{\"type\":\"Set\",\"inner\":{}}}", emit_shape(inner)),
        Shape::Map(key, value) => format!(
            "{{\"type\":\"Map\",\"key\":{},\"value\":{}}}",
            emit_shape(key),
            emit_shape(value),
        ),
        Shape::Reference(name) => format!("{{\"type\":\"Reference\",\"name\":\"{}\"}}", name),
    }
}

fn emit_service(service: &Service) -> String {
    let mut output = String::new();

    output.push_str(&format!("\"{}\":{{", service.name));
    output.push_str("\"type\":\"Service\",");

    output.push_str(&format!(
        "\"methods\":{{{}}}",
        service
            .methods
            .iter()
            .map(emit_method)
            .collect::<Vec<_>>()
            .join(",")
    ));

    output.push_str("}");

    output
}

fn emit_method(method: &Method) -> String {
    format!(
        "\"{}\":{{\"inputs\":{{{}}},\"output\":{}}}",
        method.name,
        method
            .inputs
            .iter()
            .map(|(name, def)| format!("\"{}\":{}", name, emit_shape(&def.shape)))
            .collect::<Vec<_>>()
            .join(","),
        emit_shape(&method.output.shape)
    )
}
