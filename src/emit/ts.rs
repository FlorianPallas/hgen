use crate::schema::*;

pub fn emit_schema(schema: &Schema) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "import {{ {} }} from './{}.custom';",
        schema
            .custom_shapes
            .iter()
            .map(|t| t.name.clone())
            .collect::<Vec<_>>()
            .join(", "),
        schema.name
    ));

    output.push_str("\n\n");

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

fn emit_object(schema: &Schema, message: &Object) -> String {
    let mut output = String::new();

    output.push_str(&format!("export class {} ", message.name));
    output.push_str("{\n");

    message.fields.iter().for_each(|field| {
        output.push_str(&format!(
            "  {}: {};\n",
            field.name,
            emit_shape(schema, &field.shape)
        ));
    });

    output.push_str("\n");
    output.push_str("  // prettier-ignore\n");
    output.push_str("  static $hGEN = {");
    output.push_str(
        &message
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{}:{{{},data:{{{}}}}}",
                    field.name,
                    reflect_shape(schema, &field.shape),
                    field
                        .data
                        .iter()
                        .map(|(k, v)| format!("{}:'{}'", k, v))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            })
            .collect::<Vec<_>>()
            .join(","),
    );
    output.push_str("} as const\n");

    output.push_str("}\n");

    output
}

fn emit_shape(module: &Schema, shape: &Shape) -> String {
    match shape {
        Shape::Primitive(primitive) => emit_simple_shape(module, primitive),
        Shape::Nullable(inner) => format!("({} | undefined)", emit_shape(module, inner)),
        Shape::List(inner) => format!("Array<{}>", emit_simple_shape(module, inner)),
        Shape::Set(inner) => format!("Set<{}>", emit_simple_shape(module, inner)),
        Shape::Map(key, value) => format!(
            "Map<{}, {}>",
            emit_simple_shape(module, key),
            emit_simple_shape(module, value)
        ),
    }
}

fn emit_simple_shape(module: &Schema, def: &SimpleShape) -> String {
    match def {
        SimpleShape::Bool { .. } => "boolean".to_owned(),
        SimpleShape::Int32 { .. } => "number".to_owned(),
        SimpleShape::Int64 { .. } => "number".to_owned(),
        SimpleShape::Float32 { .. } => "number".to_owned(),
        SimpleShape::Float64 { .. } => "number".to_owned(),
        SimpleShape::String { .. } => "string".to_owned(),
        SimpleShape::Ref(name) => module.resolve(&name).unwrap().name().to_owned(),
    }
}

fn reflect_shape(module: &Schema, shape: &Shape) -> String {
    match shape {
        Shape::Primitive(def) => reflect_simple_shape(module, def),
        Shape::Nullable(inner) => {
            format!("type:'optional',inner:{{{}}}", reflect_shape(module, inner))
        }
        Shape::List(inner) => {
            format!(
                "type:'list',inner:{{{}}}",
                reflect_simple_shape(module, inner)
            )
        }
        Shape::Set(inner) => {
            format!(
                "type:'set',inner:{{{}}}",
                reflect_simple_shape(module, inner)
            )
        }
        Shape::Map(key, value) => {
            format!(
                "type:'map',key:{{{}}},value:{{{}}}",
                reflect_simple_shape(module, key),
                reflect_simple_shape(module, value)
            )
        }
    }
}

fn reflect_simple_shape(schema: &Schema, shape: &SimpleShape) -> String {
    match shape {
        SimpleShape::Bool { .. } => "shape:'bool'".to_owned(),
        SimpleShape::Int32 { .. } => "shape:'int32'".to_owned(),
        SimpleShape::Int64 { .. } => "shape:'int64'".to_owned(),
        SimpleShape::Float32 { .. } => "shape:'float32'".to_owned(),
        SimpleShape::Float64 { .. } => "shape:'float64'".to_owned(),
        SimpleShape::String { .. } => "shape:'string'".to_owned(),
        SimpleShape::Ref(name) => {
            format!(
                "shape:'reference',ref:()=>{}.$hGEN",
                schema.resolve(&name).unwrap().name()
            )
        }
    }
}
