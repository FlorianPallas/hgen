use crate::lang::schema::*;

pub fn emit_schema(_name: &str, schema: &Schema) -> String {
    ron::ser::to_string_pretty(schema, ron::ser::PrettyConfig::default()).unwrap()
}
