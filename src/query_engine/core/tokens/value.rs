#[derive(Debug)]
pub enum Value {
    Literal(String),
    FieldName(String),
    Number(f32),
    Boolean(bool),
    List(List),
    None,
}

#[derive(Debug)]
struct List {}

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
pub fn parse_boolean(lexeme: &String) -> Option<bool> {
    if lexeme == "true" {
        return Some(true);
    }
    if lexeme == "false" {
        return Some(false);
    }
    None
}
