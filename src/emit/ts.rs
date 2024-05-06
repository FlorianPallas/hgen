use crate::schema::*;

pub fn emit_schema(schema: &Schema) -> String {
    let mut output = String::new();

    output.push_str(
        "export type Writer = {
  writeBool: (value: boolean) => void;
  writeString: (value: string) => void;
  writeInt32: (value: number) => void;
  writeFloat32: (value: number) => void;
};

export type Reader = {
  readBool: () => boolean;
  readString: () => string;
  readInt32: () => number;
  readFloat32: () => number;
};",
    );

    output.push_str("\n\n");

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

    output.push_str(&format!(
        "  static $hWrite=(writer:Writer,value:{})=>{{",
        message.name
    ));
    message.fields.iter().for_each(|field| {
        output.push_str(&write_shape(schema, &field.name, &field.shape));
    });
    output.push_str("}");

    output.push_str("\n");

    output.push_str(&format!(
        "  static $hRead=(reader:Reader):{}=>({{",
        message.name
    ));
    output.push_str(
        &message
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{}:{}",
                    field.name,
                    read_shape(schema, &field.name, &field.shape)
                )
            })
            .collect::<Vec<_>>()
            .join(","),
    );
    output.push_str("})");

    output.push_str("\n");

    output.push_str("  static $hSchema={");
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

fn write_shape(schema: &Schema, name: &str, shape: &Shape) -> String {
    match shape {
        Shape::Simple(inner) => write_simple_shape(schema, &name, &inner),
        Shape::Nullable(inner) => {
            format!(
                "if(value.{}!==undefined){{writer.writeBool(true);{}}}else{{writer.writeBool(false)}};",
                name,
                write_shape(schema, &name, &inner),
            )
        }
        _ => "/* TODO */".to_owned(),
    }
}

fn read_shape(schema: &Schema, name: &str, shape: &Shape) -> String {
    match shape {
        Shape::Simple(inner) => read_simple_shape(schema, &name, &inner),
        Shape::Nullable(inner) => {
            format!(
                "reader.readBool()?{}:undefined",
                read_shape(schema, &name, &inner),
            )
        }
        _ => "/* TODO */".to_owned(),
    }
}

fn write_simple_shape(schema: &Schema, name: &str, shape: &SimpleShape) -> String {
    match shape {
        SimpleShape::Ref(type_name) => {
            format!("{}.$hWrite(writer,value.{});", type_name, name)
        }
        _ => format!("writer.write{}(value.{});", shape.to_str(), name),
    }
}

fn read_simple_shape(schema: &Schema, name: &str, shape: &SimpleShape) -> String {
    match shape {
        SimpleShape::Ref(type_name) => {
            format!("{}.$hRead(reader)", type_name)
        }
        _ => format!("reader.read{}()", shape.to_str()),
    }
}

fn emit_shape(schema: &Schema, shape: &Shape) -> String {
    match shape {
        Shape::Simple(primitive) => emit_simple_shape(schema, primitive),
        Shape::Nullable(inner) => format!("({} | undefined)", emit_shape(schema, inner)),
        Shape::List(inner) => format!("Array<{}>", emit_simple_shape(schema, inner)),
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
        SimpleShape::Bool { .. } => "boolean".to_owned(),
        SimpleShape::Int32 { .. } => "number".to_owned(),
        SimpleShape::Int64 { .. } => "number".to_owned(),
        SimpleShape::Float32 { .. } => "number".to_owned(),
        SimpleShape::Float64 { .. } => "number".to_owned(),
        SimpleShape::String { .. } => "string".to_owned(),
        SimpleShape::Ref(name) => schema.resolve(&name).unwrap().name().to_owned(),
    }
}

fn reflect_shape(module: &Schema, shape: &Shape) -> String {
    match shape {
        Shape::Simple(def) => reflect_simple_shape(module, def),
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
                "shape:'reference',ref:()=>{}.$hSchema",
                schema.resolve(&name).unwrap().name()
            )
        }
    }
}
