use crate::model::*;

pub fn generate(module: &ModuleDef) -> String {
    print_module(module)
}

fn print_module(module: &ModuleDef) -> String {
    let mut output = String::new();

    output.push_str("import 'package:json_annotation/json_annotation.dart';\n\n");
    output.push_str(&format!("part '{}.g.dart';\n\n", module.name));

    output.push_str(
        &module
            .objects
            .iter()
            .map(|model| print_object(module, model))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    output.push_str("\n");

    output.push_str(
        &module
            .enums
            .iter()
            .map(|def| print_enum(module, def))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    output
}

fn print_enum(_: &ModuleDef, def: &EnumDef) -> String {
    let mut output = String::new();

    output.push_str("@JsonEnum()\n");
    output.push_str(&format!("enum {} ", def.name));
    output.push_str("{\n");
    def.values.iter().for_each(|value| {
        output.push_str(&format!("  {},\n", value));
    });
    output.push_str("}\n");

    output
}

fn print_object(module: &ModuleDef, message: &ObjectDef) -> String {
    let mut output = String::new();

    output.push_str("@JsonSerializable()\n");
    output.push_str(&format!("class {} ", &message.name));
    output.push_str("{\n");
    output.push_str(&print_fields(module, &message.fields));
    output.push_str("\n");
    output.push_str(format!("  {}({{\n", &message.name).as_str());
    message.fields.iter().for_each(|field| {
        output.push_str(&format!(
            "    {}this.{},\n",
            if field.nullable { "" } else { "required " },
            field.name,
        ));
    });
    output.push_str("  });\n");
    output.push_str("\n");
    output.push_str(
        format!(
            "  factory {}.fromJson(Map<String, dynamic> json) => _${}FromJson(json);\n",
            &message.name, &message.name
        )
        .as_str(),
    );
    output.push_str(
        format!(
            "  Map<String, dynamic> toJson() => _${}ToJson(this);\n",
            &message.name
        )
        .as_str(),
    );
    output.push_str("}\n");

    output
}

fn print_fields(module: &ModuleDef, fields: &Vec<FieldDef>) -> String {
    let mut output = String::new();

    fields.iter().for_each(|field| {
        output.push_str(&format!(
            "  {}{} {};\n",
            print_shape(module, &field.type_def),
            if field.nullable { "?" } else { "" },
            field.name
        ));
    });

    output
}

fn print_shape(module: &ModuleDef, shape: &TypeDef) -> String {
    match shape {
        TypeDef::Simple(def) => print_simple_type(module, def),
        TypeDef::List(inner) => format!("List<{}>", print_simple_type(module, inner)),
        TypeDef::Set(inner) => format!("Set<{}>", print_simple_type(module, inner)),
        TypeDef::Map(key, value) => format!(
            "Map<{}, {}>",
            print_simple_type(module, key),
            print_simple_type(module, value)
        ),
    }
}

fn print_simple_type(module: &ModuleDef, def: &SimpleTypeDef) -> String {
    match def {
        SimpleTypeDef::Bool { .. } => "bool".to_owned(),
        SimpleTypeDef::Int32 { .. } => "int".to_owned(),
        SimpleTypeDef::Int64 { .. } => "int".to_owned(),
        SimpleTypeDef::Float32 { .. } => "double".to_owned(),
        SimpleTypeDef::Float64 { .. } => "double".to_owned(),
        SimpleTypeDef::String { .. } => "String".to_owned(),
        SimpleTypeDef::Instant { .. } => "DateTime".to_owned(),
        SimpleTypeDef::Ref(name) => format!("{}", &module.resolve(&name)),
    }
}
