use std::collections::HashMap;

use crate::lang::schema::*;

pub fn emit_schema(name: &str, schema: &Schema) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "import {{ {} }} from './{}.extern';\n\n",
        schema
            .extern_types
            .iter()
            .map(|t| t.name.clone())
            .collect::<Vec<_>>()
            .join(", "),
        name
    ));

    output.push_str(
        &schema
            .aliases
            .iter()
            .map(|a| emit_alias(schema, a))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    output.push_str(
        &schema
            .enums
            .iter()
            .map(|e| emit_enum(schema, e))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    output.push_str(
        &schema
            .objects
            .iter()
            .map(|s| emit_object(schema, s))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    output
}

fn emit_alias(schema: &Schema, alias: &Alias) -> String {
    format!(
        "export type {} = {};\n\n",
        alias.name,
        emit_type(schema, &alias.def.shape, &alias.def.data)
    )
}

fn emit_enum(_schema: &Schema, message: &Enum) -> String {
    let mut output = String::new();

    output.push_str(&format!("export enum {} ", message.name));
    output.push_str("{\n");
    message.fields.iter().for_each(|(name, _)| {
        output.push_str(&format!("  {},\n", name));
    });
    output.push_str("}\n");

    output
}

fn emit_object(schema: &Schema, message: &Struct) -> String {
    let mut output = String::new();

    output.push_str(&format!("export class {} ", message.name));
    output.push_str("{\n");
    message.fields.iter().for_each(|(name, def)| {
        let data = def.data.clone();
        let (optional, shape): (bool, &Shape) = match &def.shape {
            Shape::Optional(s) => (true, s),
            _ => (false, &def.shape),
        };

        output.push_str(&format!(
            "  {}{}: {};\n",
            name,
            if optional { "?" } else { "" },
            emit_type(schema, shape, &data)
        ));
    });
    output.push_str("}\n");

    output
}

fn emit_type(schema: &Schema, shape: &Shape, data: &HashMap<String, String>) -> String {
    let mut output = String::new();

    output.push_str(&emit_shape(schema, shape));
    if !data.is_empty() {
        output.push_str(" & ");
        output.push_str(&emit_metadata(schema, data));
    }

    output
}

fn emit_metadata(_schema: &Schema, data: &HashMap<String, String>) -> String {
    format!(
        "{{\n{}\n}}",
        data.iter()
            .map(|(k, v)| format!("  {}: '{}'", k, v))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn emit_shape(schema: &Schema, shape: &Shape) -> String {
    match shape {
        Shape::Primitive(primitive) => match primitive {
            Primitive::Bool { .. } => "boolean".to_owned(),
            Primitive::Int32 { .. } => "number".to_owned(),
            Primitive::Int64 { .. } => "number".to_owned(),
            Primitive::Float32 { .. } => "number".to_owned(),
            Primitive::Float64 { .. } => "number".to_owned(),
            Primitive::String { .. } => "string".to_owned(),
        },
        Shape::Optional(inner) => format!("({} | undefined)", emit_shape(schema, inner)),
        Shape::List(inner) => format!("Array<{}>", emit_shape(schema, inner)),
        Shape::Set(inner) => format!("Set<{}>", emit_shape(schema, inner)),
        Shape::Map(key, value) => format!(
            "Map<{}, {}>",
            emit_shape(schema, key),
            emit_shape(schema, value)
        ),
        Shape::Reference(name) => schema
            .resolve(&name)
            .expect(&format!("Failed to resolve type reference \"{}\"", name))
            .name()
            .to_owned(),
    }
}
