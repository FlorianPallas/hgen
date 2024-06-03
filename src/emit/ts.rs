use crate::lang::schema::*;

pub fn emit_schema(file_name: &str, schema: &Schema) -> String {
    let mut output = String::new();

    // emit header
    output.push_str("// AUTOGENERATED FILE - DO NOT EDIT\n");
    output.push_str("\n");

    // emit models
    output.push_str(
        &schema
            .models
            .iter()
            .map(|(name, def)| emit_model(name, def, file_name))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    output.push_str("\n");

    // emit consumers
    output.push_str(
        &schema
            .services
            .iter()
            .map(|(name, shape)| emit_consumer(name, shape))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    output.push_str("\n");

    // emit providers
    output.push_str(
        &schema
            .services
            .iter()
            .map(|(name, def)| emit_provider(name, def))
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

fn emit_provider(name: &str, service: &Service) -> String {
    let mut output = String::new();

    output.push_str(&format!("export interface {}Provider ", name));
    output.push_str("{\n");
    output.push_str(
        &service
            .methods
            .iter()
            .map(emit_provider_method)
            .collect::<Vec<_>>()
            .join("\n"),
    );
    output.push_str("}\n");

    output
}

fn emit_provider_method(method: &Method) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "  {}({}): Promise<{}>;\n",
        method.name,
        method
            .inputs
            .iter()
            .map(|(name, shape)| format!("{}: {}", name, emit_shape(shape)))
            .collect::<Vec<_>>()
            .join(", "),
        emit_shape(&method.output)
    ));

    output
}

fn emit_consumer(name: &str, service: &Service) -> String {
    let mut output = String::new();

    output.push_str(&format!("export class {}Consumer ", name));
    output.push_str("{\n");
    output.push_str(
        "  constructor(\n    protected request: (method: string, inputs: any) => Promise<any>\n  ) {}\n\n",
    );
    output.push_str(
        &service
            .methods
            .iter()
            .map(emit_consumer_method)
            .collect::<Vec<_>>()
            .join("\n"),
    );
    output.push_str("}\n");

    output
}

fn emit_consumer_method(method: &Method) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "  {}({}): Promise<{}> {{\n",
        method.name,
        method
            .inputs
            .iter()
            .map(|(name, shape)| format!("{}: {}", name, emit_shape(shape)))
            .collect::<Vec<_>>()
            .join(", "),
        emit_shape(&method.output)
    ));
    output.push_str(&format!(
        "    return this.request(\"{}\", {{ {} }});\n",
        method.name,
        method
            .inputs
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    ));
    output.push_str("  }\n");

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
            .map(|(name, def)| reflect_model(name, def))
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
            .map(|(name, def)| reflect_service(name, def))
            .collect::<Vec<_>>()
            .join(",")
    ));

    output.push_str("} as const;");

    output
}

fn emit_model(name: &str, def: &Model, file_name: &str) -> String {
    match def {
        Model::Struct(s) => emit_struct(name, s),
        Model::Enum(e) => emit_enum(name, e),
        Model::Alias(a) => emit_alias(name, a),
        Model::External(_) => format!("import {{ {} }} from './{}.external';\n", name, file_name),
    }
}

fn reflect_model(name: &str, def: &Model) -> String {
    match def {
        Model::Struct(inner) => format!("{}:{{{}}}", name, reflect_struct(inner)),
        Model::Enum(inner) => format!("{}:{{{}}}", name, reflect_enum(inner)),
        Model::Alias(inner) => {
            format!(
                "{}:{{type:'alias',inner:{}}}",
                name,
                reflect_annotated_shape(&inner.shape)
            )
        }
        Model::External(inner) => {
            format!(
                "{}:{{type:'external',inner:{}}}",
                name,
                reflect_annotated_shape(&inner.shape)
            )
        }
    }
}

fn reflect_service(name: &str, def: &Service) -> String {
    let mut output = String::new();

    output.push_str(&format!("{}:{{", name));
    output.push_str("type:'service',");

    output.push_str(&format!(
        "methods:{{{}}}",
        def.methods
            .iter()
            .map(reflect_method)
            .collect::<Vec<_>>()
            .join(",")
    ));

    output.push_str("}");

    output
}

fn reflect_method(def: &Method) -> String {
    format!(
        "{}:{{inputs:{{{}}},output:{{{}}}}}",
        def.name,
        def.inputs
            .iter()
            .map(|(name, shape)| format!("{}:{{{}}}", name, reflect_annotated_shape(shape)))
            .collect::<Vec<_>>()
            .join(","),
        reflect_annotated_shape(&def.output)
    )
}

fn reflect_struct(def: &Struct) -> String {
    format!(
        "type:'struct',fields:{{{}}}",
        def.fields
            .iter()
            .map(|(name, shape)| format!("{}:{{{}}}", name, reflect_annotated_shape(shape)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn reflect_enum(def: &Enum) -> String {
    format!(
        "type:'enum',fields:{{{}}}",
        def.values
            .iter()
            .map(|value| format!("{}:''", value))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn reflect_shape(shape: &Shape) -> String {
    match shape {
        Shape::Primitive(inner) => format!("type:'{}'", inner.to_string().to_lowercase()),
        Shape::Nullable(inner) => {
            format!("type:'nullable',inner:{{{}}}", reflect_shape(inner))
        }
        Shape::List(inner) => format!("type:'list',inner:{{{}}}", reflect_shape(inner)),
        Shape::Map(key, value) => {
            format!(
                "type:'map',key:{{{}}},value:{{{}}}",
                reflect_shape(key),
                reflect_shape(value)
            )
        }
        Shape::Reference(inner) => format!("type:'reference',name:'{}'", inner),
    }
}

fn reflect_annotated_shape(shape: &Annotated<Shape>) -> String {
    format!(
        "{},data:{{{}}}",
        reflect_shape(shape),
        shape
            .data
            .iter()
            .map(|(k, v)| format!("{}:'{}'", k, v))
            .collect::<Vec<_>>()
            .join(","),
    )
}

fn emit_alias(name: &str, alias: &Alias) -> String {
    format!("export type {} = {};\n", name, emit_shape(&alias.shape))
}

fn emit_enum(name: &str, message: &Enum) -> String {
    let mut output = String::new();

    output.push_str(&format!("export enum {} ", name));
    output.push_str("{\n");
    message.values.iter().for_each(|name| {
        output.push_str(&format!("  {} = '{}',\n", name, name));
    });
    output.push_str("}\n");

    output
}

fn emit_struct(name: &str, message: &Struct) -> String {
    let mut output = String::new();

    output.push_str(&format!("export class {} ", name));
    output.push_str("{\n");
    message.fields.iter().for_each(|(name, shape)| {
        output.push_str(&format!("  {}: {};\n", name, emit_shape(shape)));
    });
    output.push_str("}\n");

    output
}

fn emit_shape(shape: &Shape) -> String {
    match shape {
        Shape::Primitive(primitive) => match primitive {
            Primitive::Unit { .. } => "void".to_owned(),
            Primitive::Bool { .. } => "boolean".to_owned(),
            Primitive::Int8 { .. } => "number".to_owned(),
            Primitive::Int16 { .. } => "number".to_owned(),
            Primitive::Int32 { .. } => "number".to_owned(),
            Primitive::Int64 { .. } => "bigint".to_owned(),
            Primitive::Int128 { .. } => "bigint".to_owned(),
            Primitive::Float32 { .. } => "number".to_owned(),
            Primitive::Float64 { .. } => "number".to_owned(),
            Primitive::String { .. } => "string".to_owned(),
        },
        Shape::Nullable(inner) => format!("{} | null", emit_shape(inner)),
        Shape::List(inner) => format!("({}[])", emit_shape(inner)),
        Shape::Map(key, value) => format!("Map<{}, {}>", emit_shape(key), emit_shape(value)),
        Shape::Reference(name) => name.to_owned(),
    }
}
