use std::ops::Deref;

use crate::lang::schema::*;

pub fn emit_schema(module_name: &str, schema: &Schema) -> String {
    let mut output = String::new();

    // emit header
    output.push_str("// AUTOGENERATED FILE - DO NOT EDIT\n");
    output.push_str("\n");
    output.push_str(&format!("import '{}.external.dart';\n", module_name));
    output.push_str("\n");
    output.push_str("abstract class RequestHandler {\n");
    output.push_str(
        "  Future<dynamic> request(String service, String method, Map<String, dynamic> params);\n",
    );
    output.push_str("}\n");
    output.push_str("\n");

    // emit models
    output.push_str(
        &schema
            .models
            .iter()
            .map(|(name, def)| emit_model(name, def))
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

    // emit serialization
    output.push_str(
        &schema
            .models
            .iter()
            .map(|(name, def)| serialize_model(name, def))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    output.push_str("\n");
    output.push_str("\n");

    // emit deserialization
    output.push_str(
        &schema
            .models
            .iter()
            .map(|(name, def)| deserialize_model(name, def))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    output.push_str("\n");
    output.push_str("\n");

    output
}

fn emit_consumer(name: &str, service: &Service) -> String {
    let mut output = String::new();

    output.push_str(&format!("class {}Consumer ", name));
    output.push_str("{\n\n");
    output.push_str("  final RequestHandler handler;\n");
    output.push_str(&format!("  final String name = \"{}\";", name));
    output.push_str("\n\n");
    output.push_str(&format!("  {}Consumer(this.handler);\n", name));
    output.push_str("\n");

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
        "  Future<{}> {}({}) async {{\n",
        emit_shape(&method.output),
        method.name,
        method
            .inputs
            .iter()
            .map(|(name, shape)| format!("{} {}", emit_shape(shape), name))
            .collect::<Vec<_>>()
            .join(", "),
    ));
    output.push_str(&format!(
        "    var response = await handler.request(name, \"{}\", <String, dynamic> {{ {} }});\n{}",
        method.name,
        method
            .inputs
            .iter()
            .map(|(name, _)| format!("\"{}\": {}", name, name))
            .collect::<Vec<_>>()
            .join(", "),
        match *method.output {
            Shape::Primitive(Primitive::Unit) => "".to_owned(),
            _ => format!(
                "    return {};\n",
                deserialize_shape("response", &method.output)
            ),
        },
    ));
    output.push_str("  }\n");

    output
}

fn emit_alias(name: &str, alias: &Alias) -> String {
    format!("typedef {} = {};\n", name, emit_shape(&alias.shape))
}

fn emit_model(name: &str, def: &Model) -> String {
    match def {
        Model::Struct(inner) => emit_struct(name, inner),
        Model::Enum(inner) => emit_enum(name, inner),
        Model::Alias(inner) => emit_alias(name, inner),
        // no need to emit external models, they are already imported
        Model::External(_) => "".to_owned(),
    }
}

fn emit_enum(name: &str, def: &Enum) -> String {
    let mut output = String::new();

    output.push_str(&format!("enum {} ", name));
    output.push_str("{\n");

    // Emit variants
    output.push_str(
        &def.values
            .iter()
            .map(|v| format!("  {}", v))
            .collect::<Vec<_>>()
            .join(",\n"),
    );
    output.push_str(";\n");
    output.push_str("\n");

    // Emit toJson method
    output.push_str(&format!("  String toJson() => ${}ToJson(this);\n", name));

    // Emit fromJson method
    output.push_str(&format!(
        "  factory {}.fromJson(String json) => ${}FromJson(json);\n",
        name, name
    ));

    output.push_str("}\n");

    output
}

fn emit_struct(name: &str, def: &Struct) -> String {
    let mut output = String::new();

    output.push_str(&format!("class {} ", name));
    output.push_str("{\n");

    // Emit fields
    def.fields.iter().for_each(|(name, shape)| {
        output.push_str(&format!("  {} {};\n", emit_shape(shape), name));
    });
    output.push_str("\n");

    // Emit constructor
    output.push_str(format!("  {}({{\n", name).as_str());
    def.fields.iter().for_each(|(name, shape)| {
        let optional = match shape.deref() {
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

    // Emit toJson method
    output.push_str(&format!(
        "  Map<String, dynamic> toJson() => ${}ToJson(this);\n",
        name
    ));

    // Emit fromJson method
    output.push_str(&format!(
        "  factory {}.fromJson(Map<String, dynamic> json) => ${}FromJson(json);\n",
        name, name
    ));

    output.push_str("}\n");

    output
}

fn emit_shape(shape: &Shape) -> String {
    match shape {
        Shape::Primitive(primitive) => match primitive {
            Primitive::Unit { .. } => "void",
            Primitive::Bool { .. } => "bool",
            Primitive::Int8 { .. } => "int",
            Primitive::Int16 { .. } => "int",
            Primitive::Int32 { .. } => "int",
            Primitive::Int64 { .. } => "int",
            Primitive::Int128 { .. } => todo!("Int128 not supported"),
            Primitive::Float32 { .. } => "double",
            Primitive::Float64 { .. } => "double",
            Primitive::String { .. } => "String",
        }
        .to_owned(),
        Shape::Nullable(inner) => format!("{}?", emit_shape(inner)),
        Shape::List(inner) => format!("List<{}>", emit_shape(inner)),
        Shape::Map(key, value) => format!("Map<{}, {}>", emit_shape(key), emit_shape(value)),
        Shape::Reference(name) => name.to_owned(),
    }
}

fn serialize_model(name: &str, model: &Model) -> String {
    match model {
        Model::Struct(inner) => serialize_struct(name, inner),
        Model::Enum(inner) => serialize_enum(name, inner),
        Model::Alias(inner) => serialize_alias(name, inner),
        Model::External(_) => format!(
            "/* Map<String, dynamic> ${}ToJson({} instance) => ? */",
            name, name
        ),
    }
}

fn deserialize_model(name: &str, model: &Model) -> String {
    match model {
        Model::Struct(inner) => deserialize_struct(name, inner),
        Model::Enum(inner) => deserialize_enum(name, inner),
        Model::Alias(inner) => deserialize_alias(name, inner),
        Model::External(_) => format!(
            "/* {} ${}FromJson(Map<String, dynamic> json) => ? */",
            name, name
        ),
    }
}

fn serialize_struct(name: &str, def: &Struct) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "Map<String, dynamic> ${}ToJson({} instance) => <String, dynamic>{{",
        name, name
    ));
    output.push_str(
        &def.fields
            .iter()
            .map(|(name, shape)| {
                format!(
                    "'{}':{}",
                    name,
                    serialize_shape(&format!("instance.{}", name), shape)
                )
            })
            .collect::<Vec<_>>()
            .join(","),
    );
    output.push_str("};");

    output
}

fn serialize_enum(name: &str, def: &Enum) -> String {
    let mut output = String::new();

    output.push_str(&format!("dynamic ${}ToJson({} instance)=>", name, name));
    output.push_str("switch(instance){");
    output.push_str(
        &def.values
            .iter()
            .map(|v| format!("{}.{}=>'{}'", name, v, v))
            .collect::<Vec<_>>()
            .join(","),
    );
    output.push_str("};");

    output
}

fn deserialize_enum(name: &str, def: &Enum) -> String {
    let mut output = String::new();

    output.push_str(&format!("{} ${}FromJson(String value)=>", name, name));
    output.push_str("switch(value){");
    output.push_str(
        &def.values
            .iter()
            .map(|v| format!("'{}'=>{}.{},", v, name, v))
            .collect::<Vec<_>>()
            .join(""),
    );
    output.push_str(&format!("_=>throw'Unknown {} value: $value'", name));
    output.push_str("};");

    output
}

fn serialize_alias(name: &str, alias: &Alias) -> String {
    format!(
        "dynamic ${}ToJson({} instance) => {};",
        name,
        name,
        serialize_shape("instance", &alias.shape)
    )
}

fn deserialize_alias(name: &str, alias: &Alias) -> String {
    format!(
        "{} ${}FromJson(dynamic json) => {};",
        name,
        name,
        deserialize_shape("json", &alias.shape)
    )
}

fn deserialize_struct(name: &str, def: &Struct) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "{} ${}FromJson(Map<String,dynamic>json)=>{}(",
        name, name, name
    ));
    output.push_str(
        &def.fields
            .iter()
            .map(|(name, shape)| {
                format!(
                    "{}:{}",
                    name,
                    deserialize_shape(&format!("json['{}']", name), shape)
                )
            })
            .collect::<Vec<_>>()
            .join(","),
    );
    output.push_str(");");

    output
}

fn serialize_shape(name: &str, shape: &Shape) -> String {
    match shape {
        Shape::Nullable(inner) => format!(
            "{} == null ? null : {}",
            name,
            serialize_shape(&format!("{} as {}", name, emit_shape(inner)), inner),
        ),
        Shape::Reference(type_name) => {
            format!("${}ToJson({})", type_name, name)
        }
        _ => format!("{}", name),
    }
}

fn deserialize_shape(field_name: &str, shape: &Shape) -> String {
    match shape {
        Shape::Primitive(_) => format!("{} as {}", field_name, emit_shape(shape)),
        Shape::Nullable(inner) => format!(
            "{} == null ? null : {}",
            field_name,
            deserialize_shape(field_name, inner),
        ),
        Shape::List(inner) => format!(
            "({} as List<dynamic>).map((e) => {}).toList()",
            field_name,
            deserialize_shape("e", inner)
        ),
        Shape::Map(key, value) => format!(
            "({} as Map<String,dynamic>).map((k,v) => MapEntry({},{}))",
            field_name,
            deserialize_shape("k", key),
            deserialize_shape("v", value),
        ),
        Shape::Reference(name) => format!("${}FromJson({})", name, field_name),
    }
}
