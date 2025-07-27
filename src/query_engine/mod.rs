use std::collections::HashMap;
pub mod core;

pub fn query(query: String, fields: Vec<String>, columns: HashMap<String, Vec<String>>) {
  core::parser::parse(query);
}
