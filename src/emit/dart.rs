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

    output.push_str(&format!("class {} ", &def.name));
    output.push_str("{\n");

    // Emit fields
    def.fields.iter().for_each(|(name, field)| {
        output.push_str(&format!(
            "  {} {};\n",
            emit_shape(schema, &field.shape),
            name
        ));
    });
    output.push_str("\n");

    // Emit constructor
    output.push_str(format!("  {}({{\n", &def.name).as_str());
    def.fields.iter().for_each(|(name, field)| {
        let optional = match &field.shape {
            Shape::Nullable(_) => true,
            _ => false,
        };

        output.push_str(&format!(
            "    {}this.{},\n",
            if optional { "" } else { "required " },
            name,
        ));
    });
    output.push_str("  });\n");
    output.push_str("\n");

    // Emit serialization methods
    output.push_str(&format!(
        "  static void $hWrite(Writer writer,value:{}) => {{}}\n",
        def.name
    ));
    output.push_str(&format!(
        "  factory {}.$hRead(Reader reader) => {{}}\n",
        def.name
    ));

    // Emit reflection fields
    output.push_str("  static Schema $hSchema = {};\n");

    // TODO: emit utility methods
    // toString
    // equals
    // hashCode
    // ...

    output.push_str("}\n");

    output
}

fn emit_shape(schema: &Schema, shape: &Shape) -> String {
    match shape {
        Shape::Primitive(primitive) => match primitive {
            Primitive::Bool { .. } => "bool".to_owned(),
            Primitive::Int32 { .. } => "int".to_owned(),
            Primitive::Int64 { .. } => "int".to_owned(),
            Primitive::Float32 { .. } => "double".to_owned(),
            Primitive::Float64 { .. } => "double".to_owned(),
            Primitive::String { .. } => "String".to_owned(),
        },
        Shape::Nullable(inner) => format!("{}?", emit_shape(schema, inner)),
        Shape::List(inner) => format!("List<{}>", emit_shape(schema, inner)),
        Shape::Set(inner) => format!("Set<{}>", emit_shape(schema, inner)),
        Shape::Map(key, value) => format!(
            "Map<{}, {}>",
            emit_shape(schema, key),
            emit_shape(schema, value)
        ),
        Shape::Reference(name) => schema.resolve(&name).unwrap().to_owned(),
    }
}
