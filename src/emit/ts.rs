use std::collections::HashMap;

use crate::lang::schema::*;

pub fn emit_schema(module_name: &str, schema: &Schema) -> String {
    let mut output = String::new();

    // emit header
    output.push_str("// AUTOGENERATED FILE - DO NOT EDIT\n");
    output.push_str("\n");

    // emit models
    output.push_str(
        &schema
            .models
            .iter()
            .map(|m| emit_model(m, module_name))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    output.push_str("\n");

    // emit metadata
    output.push_str("// prettier-ignore\n");
    output.push_str(&format!(
        "export const $schema = {}",
        reflect_schema(schema)
    ));

    output
}

fn reflect_schema(schema: &Schema) -> String {
    let mut output = String::new();

    output.push_str("{");

    // emit models
    output.push_str(&format!(
        "models:{{{}}}",
        schema
            .models
            .iter()
            .map(reflect_model)
            .collect::<Vec<_>>()
            .join(",")
    ));

    // emit services
    output.push_str(",");
    output.push_str(&format!(
        "services:{{{}}}",
        schema
            .services
            .iter()
            .map(reflect_service)
            .collect::<Vec<_>>()
            .join(",")
    ));

    output.push_str("} as const;");

    output
}

fn emit_model(model: &Model, module_name: &str) -> String {
    match model {
        Model::Struct(s) => emit_struct(s),
        Model::Enum(e) => emit_enum(e),
        Model::Alias(a) => emit_alias(a),
        Model::External(_) => format!(
            "import {{ {} }} from './{}.external';\n",
            model.name(),
            module_name
        ),
    }
}

fn reflect_model(model: &Model) -> String {
    match model {
        Model::Struct(def) => reflect_struct(def),
        Model::Enum(def) => reflect_enum(def),
        Model::Alias(inner) => {
            format!(
                "{}:{{type:'Alias',inner:{}}}",
                inner.name,
                reflect_type(&inner.def)
            )
        }
        Model::External(inner) => {
            format!(
                "{}:{{type:'External',inner:{}}}",
                inner.name,
                reflect_type(&inner.def)
            )
        }
    }
}

fn reflect_service(service: &Service) -> String {
    let mut output = String::new();

    output.push_str(&format!("{}:{{", service.name));
    output.push_str("type:'Service',");

    output.push_str(&format!(
        "methods:{{{}}}",
        service
            .methods
            .iter()
            .map(reflect_method)
            .collect::<Vec<_>>()
            .join(",")
    ));

    output.push_str("}");

    output
}

fn reflect_method(method: &Method) -> String {
    format!(
        "{}:{{inputs:{{{}}},output:{}}}",
        method.name,
        method
            .inputs
            .iter()
            .map(|(name, def)| format!("{}:{}", name, reflect_type(def)))
            .collect::<Vec<_>>()
            .join(","),
        reflect_type(&method.output)
    )
}

fn reflect_type(def: &Type) -> String {
    format!("{}", reflect_shape(&def.shape))
}

fn reflect_struct(def: &Struct) -> String {
    format!(
        "{}:{{type:'Struct',fields:{{{}}}}}",
        def.name,
        def.fields
            .iter()
            .map(|(name, def)| format!("{}:{}", name, reflect_shape(&def.shape)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn reflect_enum(def: &Enum) -> String {
    format!(
        "{}:{{type:'Enum',fields:{{{}}}}}",
        def.name,
        def.values
            .iter()
            .map(|name| format!("{}:''", name))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn reflect_shape(shape: &Shape) -> String {
    match shape {
        Shape::Primitive(inner) => format!("{{type:'{}'}}", inner.to_string()),
        Shape::Nullable(inner) => {
            format!("{{type:'Nullable',inner:{}}}", reflect_shape(inner))
        }
        Shape::List(inner) => format!("{{type:'List',inner:{}}}", reflect_shape(inner)),
        Shape::Set(inner) => format!("{{type:'Set',inner:{}}}", reflect_shape(inner)),
        Shape::Map(key, value) => {
            format!(
                "{{type:'Map',key:{},value:{}}}",
                reflect_shape(key),
                reflect_shape(value)
            )
        }
        Shape::Reference(inner) => format!("{{type:'Reference',name:'{}'}}", inner),
    }
}

fn emit_alias(alias: &Alias) -> String {
    format!("export type {} = {};\n", alias.name, emit_type(&alias.def))
}

fn emit_enum(message: &Enum) -> String {
    let mut output = String::new();

    output.push_str(&format!("export enum {} ", message.name));
    output.push_str("{\n");
    message.values.iter().for_each(|name| {
        output.push_str(&format!("  {} = '{}',\n", name, name));
    });
    output.push_str("}\n");

    output
}

fn emit_struct(message: &Struct) -> String {
    let mut output = String::new();

    output.push_str(&format!("export class {} ", message.name));
    output.push_str("{\n");
    message.fields.iter().for_each(|(name, def)| {
        output.push_str(&format!("  {}: {};\n", name, emit_type(&def)));
    });
    output.push_str("}\n");

    output
}

fn emit_type(def: &Type) -> String {
    let mut output = String::new();

    output.push_str(&emit_shape(&def.shape));
    if !def.data.is_empty() {
        output.push_str(" & ");
        output.push_str(&emit_metadata(&def.data));
    }

    output
}

fn emit_metadata(data: &HashMap<String, String>) -> String {
    format!(
        "{{\n{}\n}}",
        data.iter()
            .map(|(k, v)| format!("  {}: '{}'", k, v))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn emit_shape(shape: &Shape) -> String {
    match shape {
        Shape::Primitive(primitive) => match primitive {
            Primitive::Bool { .. } => "boolean".to_owned(),
            Primitive::Int32 { .. } => "number".to_owned(),
            Primitive::Int64 { .. } => "number".to_owned(),
            Primitive::Float32 { .. } => "number".to_owned(),
            Primitive::Float64 { .. } => "number".to_owned(),
            Primitive::String { .. } => "string".to_owned(),
        },
        Shape::Nullable(inner) => format!("({} | null)", emit_shape(inner)),
        Shape::List(inner) => format!("({}[])", emit_shape(inner)),
        Shape::Set(inner) => format!("Set<{}>", emit_shape(inner)),
        Shape::Map(key, value) => format!("Map<{}, {}>", emit_shape(key), emit_shape(value)),
        Shape::Reference(name) => name.to_owned(),
    }
}
