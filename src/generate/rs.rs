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

    output.push_str(&format!("pub enum {} ", def.name));
    output.push_str("{\n");
    def.values.iter().for_each(|value| {
        output.push_str(&format!("  {},\n", value));
    });
    output.push_str("}\n");

    output
}

fn print_object(module: &ModuleDef, message: &ObjectDef) -> String {
    let mut output = String::new();

    output.push_str(&format!("pub struct {} ", &message.name));
    output.push_str("{\n");
    output.push_str(&print_fields(module, &message.fields));
    output.push_str("}\n");

    output
}

fn print_fields(module: &ModuleDef, fields: &Vec<FieldDef>) -> String {
    let mut output = String::new();

    fields.iter().for_each(|field| {
        let shape = print_shape(module, &field.type_def);
        output.push_str(&format!(
            "    pub {}: {},\n",
            camel_to_snake(&field.name),
            if field.nullable {
                format!("Option<{}>", shape)
            } else {
                shape
            },
        ));
    });

    output
}

fn print_shape(module: &ModuleDef, shape: &TypeDef) -> String {
    match shape {
        TypeDef::Simple(def) => print_simple_type(module, def),
        TypeDef::List(inner) => format!("Vec<{}>", print_simple_type(module, inner)),
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
        SimpleTypeDef::Int32 { .. } => "i32".to_owned(),
        SimpleTypeDef::Int64 { .. } => "i64".to_owned(),
        SimpleTypeDef::Float32 { .. } => "f32".to_owned(),
        SimpleTypeDef::Float64 { .. } => "f64".to_owned(),
        SimpleTypeDef::String { .. } => "String".to_owned(),
        SimpleTypeDef::Instant { .. } => "Instant".to_owned(),
        SimpleTypeDef::Ref(name) => format!("{}", &module.resolve(&name)),
    }
}

fn camel_to_snake(name: &str) -> String {
    let mut result = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}
