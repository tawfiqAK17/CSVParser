// this module is needed by both the query_engine and the json_parser modules
use std::fmt::Display;
use crate::log_error;

#[derive(Debug)]
pub enum Value {
    Literal(String),
    FieldName(String),
    Number(f32),
    List(Vec<String>),
    None,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Literal(_), Value::Literal(_)) => true,
            (Value::FieldName(_), Value::FieldName(_)) => true,
            (Value::Number(_), Value::Number(_)) => true,
            (Value::List(_), Value::List(_)) => true,
            (Value::None, Value::None) => true,
            _ => false,
        }
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal(val) => write!(f, "{val}"),
            Value::FieldName(val) => write!(f, "{val}"),
            Value::Number(val) => write!(f, "{val}"),
            Value::List(_) => write!(f, "list"),
            Value::None => write!(f, ""),
        }
    }
}

pub fn parse_literal(lexeme: &String) -> Option<String> {
    if lexeme.starts_with("\"") {
        if lexeme.ends_with("\"") {
            return Some(lexeme[1..lexeme.len() - 1].to_string());
        }
    }
    None
}

pub fn parse_field_name(lexeme: &String) -> Option<String> {
    if lexeme.starts_with("$") {
        return Some(lexeme[1..].to_string());
    }
    None
}

pub fn parse_number(lexeme: &String) -> Option<f32> {
    match lexeme.parse::<f32>() {
        Ok(val) => return Some(val),
        Err(_) => return None,
    }
}

pub fn parse_list(lexemes: &[String], idx: usize) -> Option<(Vec<String>, usize)> {
    let mut list_as_str: String = String::new();
    let mut list: Vec<String> = Vec::new();
    match lexemes.get(idx) {
        Some(lexeme) => {
            if lexeme.starts_with("[") {
                let mut i: usize = 0;
                while let Some(lexeme) = lexemes.get(idx + i) {
                    i += 1;
                    list_as_str += lexeme;
                    if lexeme.ends_with("]") {
                        break;
                    }
                }
                if !list_as_str.ends_with("]") {
                    // the list has no end
                    return None;
                }
                let list_vals: Vec<&str> = list_as_str[1..list_as_str.len() - 1].split(",").collect();
                let mut vals_type = Value::None;
                for val in list_vals {
                    // the value is a literal
                    if let Some(literal) = parse_literal(&val.to_string()) {
                        if list.is_empty() {
                            vals_type = Value::Literal("".to_string());
                        } else {
                            if vals_type != Value::Literal("".to_string()) {
                                log_error!("the list can only have values of the same type");
                                return None;
                            }
                        }
                        list.push(literal);
                    // the value is a number
                    } else if let Some(number) = parse_number(&val.to_string()) {
                        if list.is_empty() {
                            vals_type = Value::Number(0f32);
                        } else {
                            if vals_type != Value::Number(0f32) {
                                log_error!("the list can only have values of the same type");
                                return None;
                            }
                        }
                        list.push(number.to_string());
                    } else {
                        log_error!("the list can only have values of type number or string");
                        return None;
                    }
                }
                return Some((list, idx + i - 1));
            } else {
                return None;
            }
        }
        None => return None,
    }
}
