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

    output.push_str(&format!("class {} ", &object.name));
    output.push_str("{\n");

    // Emit fields
    object.fields.iter().for_each(|field| {
        output.push_str(&format!(
            "  {} {};\n",
            emit_shape(schema, &field.shape),
            field.name
        ));
    });
    output.push_str("\n");

    // Emit constructor
    output.push_str(format!("  {}({{\n", &object.name).as_str());
    object.fields.iter().for_each(|field| {
        output.push_str(&format!("    required this.{},\n", field.name,));
    });
    output.push_str("  });\n");
    output.push_str("\n");

    // Emit serialization methods
    output.push_str(&format!(
        "  static void $hWrite(Writer writer,value:{}) => {{}}\n",
        object.name
    ));
    output.push_str(&format!(
        "  factory {}.$hRead(Reader reader) => {{}}\n",
        object.name
    ));

    // Emit reflection fields
    output.push_str("  static Schema $hSchema = {};\n");

    output.push_str("}\n");

    output
}

fn emit_shape(schema: &Schema, shape: &Shape) -> String {
    match shape {
        Shape::Simple(def) => emit_simple_shape(schema, def),
        Shape::Nullable(inner) => format!("{}?", emit_shape(schema, inner)),
        Shape::List(inner) => format!("List<{}>", emit_simple_shape(schema, inner)),
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
        SimpleShape::Int32 { .. } => "int".to_owned(),
        SimpleShape::Int64 { .. } => "int".to_owned(),
        SimpleShape::Float32 { .. } => "double".to_owned(),
        SimpleShape::Float64 { .. } => "double".to_owned(),
        SimpleShape::String { .. } => "String".to_owned(),
        SimpleShape::Ref(name) => schema.resolve(&name).unwrap().name().to_owned(),
    }
}
