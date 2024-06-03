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
            .map(|(name, def)| emit_model(name, def))
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
            .map(|(name, def)| emit_service(name, def))
            .collect::<Vec<_>>()
            .join(",")
    ));

    output.push_str("}");

    output
}

fn emit_model(name: &str, model: &Model) -> String {
    match model {
        Model::Struct(inner) => emit_struct(name, inner),
        Model::Enum(inner) => emit_enum(name, inner),
        Model::Alias(inner) => emit_alias(name, inner),
        Model::External(inner) => emit_external(name, inner),
    }
}

fn emit_struct(name: &str, def: &Struct) -> String {
    let mut output = String::new();

    output.push_str(&format!("\"{}\":{{", name));
    output.push_str("\"type\":\"Struct\",");

    output.push_str(&format!(
        "\"fields\":{{{}}}",
        def.fields
            .iter()
            .map(|(name, shape)| { format!("\"{}\":{}", name, emit_shape(shape)) })
            .collect::<Vec<_>>()
            .join(",")
    ));

    output.push_str("}");

    output
}

fn emit_enum(name: &str, def: &Enum) -> String {
    let mut output = String::new();

    output.push_str(&format!("\"{}\":{{", name));
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

fn emit_alias(name: &str, alias: &Alias) -> String {
    format!(
        "\"{}\":{{\"type\":\"Alias\",\"inner\":{}}}",
        name,
        emit_shape(&alias.shape)
    )
}

fn emit_external(name: &str, external: &External) -> String {
    format!(
        "\"{}\":{{\"type\":\"External\",\"inner\":{}}}",
        name,
        emit_shape(&external.shape)
    )
}

fn emit_shape(shape: &Shape) -> String {
    match shape {
        Shape::Primitive(primitive) => {
            format!("{{\"type\":\"{}\"}}", primitive.to_string())
        }
        Shape::Nullable(inner) => {
            format!("{{\"type\":\"Nullable\",\"inner\":{}}}", emit_shape(inner))
        }
        Shape::List(inner) => format!("{{\"type\":\"List\",\"inner\":{}}}", emit_shape(inner)),
        Shape::Map(key, value) => format!(
            "{{\"type\":\"Map\",\"key\":{},\"value\":{}}}",
            emit_shape(key),
            emit_shape(value),
        ),
        Shape::Reference(name) => format!("{{\"type\":\"Reference\",\"name\":\"{}\"}}", name),
    }
}

fn emit_service(name: &str, service: &Service) -> String {
    let mut output = String::new();

    output.push_str(&format!("\"{}\":{{", name));
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
            .map(|(name, shape)| format!("\"{}\":{}", name, emit_shape(shape)))
            .collect::<Vec<_>>()
            .join(","),
        emit_shape(&method.output)
    )
}
