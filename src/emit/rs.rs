use crate::lang::schema::*;

pub fn emit_schema(_name: &str, schema: &Schema) -> String {
    let mut output = String::new();

    output.push_str(
        &schema
            .models
            .iter()
            .map(|model| match model {
                Model::Struct(inner) => emit_struct(inner),
                Model::Enum(inner) => emit_enum(inner),
                Model::Alias(inner) => format!(
                    "pub type {} = {};\n",
                    inner.name,
                    emit_shape(&inner.def.shape)
                ),
                Model::External(inner) => format!("use external::{};\n", inner.name),
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );

    output
}

fn emit_struct(def: &Struct) -> String {
    let mut output = String::new();

    output.push_str(&format!("pub struct {} ", &def.name));
    output.push_str("{\n");
    def.fields.iter().for_each(|(name, field)| {
        output.push_str(&format!(
            "    pub {}: {},\n",
            name.to_snake_case(),
            emit_shape(&field.shape)
        ));
    });
    output.push_str("}\n");

    output
}

fn emit_enum(def: &Enum) -> String {
    let mut output = String::new();

    output.push_str(&format!("pub enum {} ", &def.name));
    output.push_str("{\n");
    def.values.iter().for_each(|value| {
        output.push_str(&format!("    {},\n", value,));
    });
    output.push_str("}\n");

    output
}

fn emit_shape(def: &Shape) -> String {
    match def {
        Shape::Primitive(primitive) => match primitive {
            Primitive::Bool { .. } => "bool".to_owned(),
            Primitive::Int32 { .. } => "i32".to_owned(),
            Primitive::Int64 { .. } => "i64".to_owned(),
            Primitive::Float32 { .. } => "f32".to_owned(),
            Primitive::Float64 { .. } => "f64".to_owned(),
            Primitive::String { .. } => "String".to_owned(),
        },
        Shape::Nullable(inner) => format!("Option<{}>", emit_shape(inner)),
        Shape::List(inner) => format!("Vec<{}>", emit_shape(inner)),
        Shape::Set(inner) => format!("Set<{}>", emit_shape(inner)),
        Shape::Map(key, value) => format!("Map<{}, {}>", emit_shape(key), emit_shape(value)),
        Shape::Reference(name) => name.to_owned(),
    }
}

pub trait ToSnakeCase {
    fn to_snake_case(&self) -> String;
}

impl ToSnakeCase for str {
    fn to_snake_case(&self) -> String {
        let mut result = String::new();
        for (i, c) in self.chars().enumerate() {
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
}

impl ToSnakeCase for String {
    fn to_snake_case(&self) -> String {
        self.as_str().to_snake_case()
    }
}
