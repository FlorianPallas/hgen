use crate::lang::schema::*;

pub fn emit_schema(_name: &str, schema: &Schema) -> String {
    serde_json::to_string_pretty(schema).unwrap()
}
