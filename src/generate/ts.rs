use crate::model::*;

pub fn generate(module: &ModuleDef) -> String {
    print_module(module)
}

fn print_module(module: &ModuleDef) -> String {
    let mut output = String::new();

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

    output.push_str(&format!("export enum {} ", def.name));
    output.push_str("{\n");
    def.values.iter().for_each(|value| {
        output.push_str(&format!("  {},\n", value));
    });
    output.push_str("}\n");

    output
}

fn print_object(module: &ModuleDef, message: &ObjectDef) -> String {
    let mut output = String::new();

    output.push_str(&format!("export class {} ", message.name));
    output.push_str("{\n");

    message.fields.iter().for_each(|field| {
        output.push_str(&format!(
            "  {}{}: {};\n",
            field.name,
            if field.nullable { "?" } else { "" },
            print_shape(module, &field.type_def)
        ));
    });

    output.push_str("\n");
    output.push_str("  // prettier-ignore\n");
    output.push_str("  static $fields = {");
    output.push_str(
        &message
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{}:{{{},nullable:{},data:{{{}}}}}",
                    field.name,
                    print_field_meta(module, &field.type_def),
                    if field.nullable { "true" } else { "false" },
                    field
                        .metadata
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

fn print_field_meta(module: &ModuleDef, shape: &TypeDef) -> String {
    match shape {
        TypeDef::Simple(def) => print_simple_meta(module, def),
        TypeDef::List(inner) => {
            format!(
                "type:'array',inner:{{{}}}",
                print_simple_meta(module, inner)
            )
        }
        TypeDef::Set(inner) => {
            format!("type:'set',inner:{{{}}}", print_simple_meta(module, inner))
        }
        TypeDef::Map(key, value) => {
            format!(
                "type:'map',key:{{{}}},value:{{{}}}",
                print_simple_meta(module, key),
                print_simple_meta(module, value)
            )
        }
    }
}

fn print_simple_meta(module: &ModuleDef, def: &SimpleTypeDef) -> String {
    match def {
        SimpleTypeDef::Bool { .. } => "type:'bool'".to_owned(),
        SimpleTypeDef::Int32 { .. } => "type:'int32'".to_owned(),
        SimpleTypeDef::Int64 { .. } => "type:'int64'".to_owned(),
        SimpleTypeDef::Float32 { .. } => "type:'float32'".to_owned(),
        SimpleTypeDef::Float64 { .. } => "type:'float64'".to_owned(),
        SimpleTypeDef::String { .. } => "type:'string'".to_owned(),
        SimpleTypeDef::Instant { .. } => "type:'instant'".to_owned(),
        SimpleTypeDef::Ref(name) => {
            format!("type:'reference',ref:'{}'", &module.resolve(&name))
        }
    }
}

fn print_shape(module: &ModuleDef, shape: &TypeDef) -> String {
    match shape {
        TypeDef::Simple(primitive) => print_simple_type(module, primitive),
        TypeDef::List(inner) => format!("Array<{}>", print_simple_type(module, inner)),
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
        SimpleTypeDef::Bool { .. } => "boolean".to_owned(),
        SimpleTypeDef::Int32 { .. } => "number".to_owned(),
        SimpleTypeDef::Int64 { .. } => "number".to_owned(),
        SimpleTypeDef::Float32 { .. } => "number".to_owned(),
        SimpleTypeDef::Float64 { .. } => "number".to_owned(),
        SimpleTypeDef::String { .. } => "string".to_owned(),
        SimpleTypeDef::Instant { .. } => "Date".to_owned(),
        SimpleTypeDef::Ref(name) => format!("{}", &module.resolve(&name)),
    }
}
