use std::collections::HashMap;
pub mod core;
use core::parser;
use core::evaluator;

pub fn query(query: String, fields: &Vec<String>, columns: &HashMap<String, Vec<String>>) -> Result<(), String> {
  let tokens = parser::parse(query)?;
  
  Ok(())
}
