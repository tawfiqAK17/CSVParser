use crate::log_error;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
enum ParseResult<T> {
    Val(T),
    None,
    Err,
}

#[derive(Debug, PartialEq)]
enum Value {
    Literal(String),
    Number(f32),
    Array(Vec<Value>),
    Boolean(bool),
    Null,
}
#[derive(Debug)]
pub struct Parser {
    index: usize,
    file_as_string: String,
}

#[derive(Debug)]
pub struct Object {
    array: Option<Vec<Object>>, // in case the fill is just an arr the
    // Option<String> will be set to None
    fields: Option<HashMap<String, Object>>,
    value: Option<Value>,
}

impl Parser {
    pub fn parse(&mut self, path: &str) -> Option<Object> {
        let mut file: File;
        match File::open(path) {
            Ok(f) => file = f,
            Err(_) => {
                log_error!("can not open the file {path}");
                return None;
            }
        }
        match file.read_to_string(&mut self.file_as_string) {
            Ok(_) => {}
            Err(_) => {
                log_error!("there was a problem converting the json file to a string");
                return None;
            }
        };
        match self.peek() {
            Some(c) => match c {
                '{' => match self.parse_object() {
                    ParseResult::Val(obj) => return Some(obj),
                    _ => return None,
                },
                '[' => match self.parse_obj_array() {
                    ParseResult::Val(array) => {
                        return Some(Object {
                            array: Some(array),
                            fields: None,
                            value: None,
                        });
                    }
                    _ => return None,
                },
                _ => match self.parse_value() {
                    ParseResult::Val(value) => {
                        return Some(Object {
                            array: None,
                            fields: None,
                            value: Some(value),
                        });
                    }
                    _ => return None,
                },
            },
            None => None,
        }
    }
    fn get_next(&mut self) -> Option<char> {
        while let Some(c) = &self.file_as_string.chars().nth(self.index) {
            if c.is_whitespace() {
                self.index += 1;
                continue;
            }
            self.index += 1;
            return Some(*c);
        }
        return None;
    }
    fn peek(&self) -> Option<char> {
        let mut tmp_idx = self.index;
        while let Some(c) = &self.file_as_string.chars().nth(tmp_idx) {
            if c.is_whitespace() {
                tmp_idx += 1;
                continue;
            }
            return Some(*c);
        }
        return None;
    }
    fn parse_object(&mut self) -> ParseResult<Object> {
        match self.parse_fields() {
            ParseResult::Val(fields) => {
                return ParseResult::Val(Object {
                    array: None,
                    fields: Some(fields),
                    value: None,
                });
            }
            _ => return ParseResult::None,
        }
    }

    fn parse_obj_array(&mut self) -> ParseResult<Vec<Object>> {
        match self.peek() {
            Some(c) => {
                if c != '[' {
                    return ParseResult::None;
                }
                self.get_next();
            }
            None => return ParseResult::None,
        }

        let mut array: Vec<Object> = Vec::new();
        match self.parse_object() {
            ParseResult::Val(obj) => array.push(obj),
            ParseResult::None => {
                if let Some(c) = self.peek() {
                    if c == ']' {
                        self.get_next();
                        return ParseResult::Val(array);
                    } else {
                        log_error!("expecting a ']' at the end of the array");
                        return ParseResult::Err;
                    }
                }
            }
            ParseResult::Err => return ParseResult::Err,
        }
        while let Some(c) = self.peek() {
            if c == ',' {
                self.get_next();
                match self.parse_object() {
                    ParseResult::Val(obj) => array.push(obj),
                    ParseResult::None => {}
                    ParseResult::Err => return ParseResult::Err,
                }
            } else {
                break;
            }
        }
        match self.peek() {
            Some(c) => {
                if c != ']' {
                    self.get_next();
                    return ParseResult::Val(array);
                }
            }
            None => {
                log_error!("expecting a ']' at the end of the array");
                return ParseResult::Err;
            }
        }
        ParseResult::None
    }
    fn parse_fields(&mut self) -> ParseResult<HashMap<String, Object>> {
        let mut fields: HashMap<String, Object> = HashMap::new();
        match self.peek() {
            Some(c) => {
                if c != '"' {
                    return ParseResult::None;
                }
                self.get_next();
            }
            None => return ParseResult::None,
        }
        match self.parse_key_val_pair() {
            ParseResult::Val((name, value)) => fields.insert(
                name,
                Object {
                    array: None,
                    fields: None,
                    value: Some(value),
                },
            ),
            ParseResult::None => return ParseResult::None,
            ParseResult::Err => return ParseResult::Err,
        };
        while let Some(c) = self.peek() {
            if c == ',' {
                self.get_next();
                match self.parse_key_val_pair() {
                    ParseResult::Val((name, value)) => fields.insert(
                        name,
                        Object {
                            array: None,
                            fields: None,
                            value: Some(value),
                        },
                    ),
                    ParseResult::None => return ParseResult::Val(fields),
                    ParseResult::Err => return ParseResult::Err,
                };
            } else {
                break;
            }
        }
        ParseResult::Val(fields)
    }

    fn parse_key_val_pair(&mut self) -> ParseResult<(String, Value)> {
        let name: String;
        let value: Value;
        match self.peek() {
            Some(c) => {
                if c != '"' || c != '\'' {
                    return ParseResult::None;
                }
            }
            None => return ParseResult::None,
        }
        match self.parse_value() {
            ParseResult::Val(val) => match val {
                Value::Literal(literal) => name = literal,
                _ => {
                    log_error!("expecting the name of the field to be a literal");
                    return ParseResult::Err;
                }
            },
            ParseResult::None => {
                log_error!("expecting a name for the field");
                return ParseResult::Err;
            }
            ParseResult::Err => return ParseResult::Err,
        }

        if let Some(c) = self.peek() {
            if c != ':' {
                log_error!("expecting a ':' after the field name '{name}'");
                return ParseResult::Err;
            }
            self.get_next();
            match self.parse_value() {
                ParseResult::Val(val) => value = val,
                ParseResult::None => {
                    log_error!("expecting a value after '{name}' ':'");
                    return ParseResult::Err;
                }
                ParseResult::Err => return ParseResult::Err,
            }
            return ParseResult::Val((name, value));
        } else {
            log_error!("expecting a ':' after the field name '{name}'");
            return ParseResult::Err;
        }
    }

    fn parse_value(&mut self) -> ParseResult<Value> {
        match self.parse_literal() {
            ParseResult::Val(literal) => return ParseResult::Val(Value::Literal(literal)),
            ParseResult::None => {}
            ParseResult::Err => return ParseResult::Err,
        };
        match self.parse_val_array() {
            ParseResult::Val(val_array) => return ParseResult::Val(Value::Array(val_array)),
            ParseResult::None => {}
            ParseResult::Err => return ParseResult::Err,
        };
        match self.parse_number() {
            ParseResult::Val(number) => return ParseResult::Val(Value::Number(number)),
            ParseResult::None => {}
            ParseResult::Err => return ParseResult::Err,
        };
        match self.parse_boolean() {
            ParseResult::Val(boolean) => return ParseResult::Val(Value::Boolean(boolean)),
            ParseResult::None => {}
            ParseResult::Err => return ParseResult::Err,
        };
        match self.parse_null() {
            ParseResult::Val(_) => return ParseResult::Val(Value::Null),
            ParseResult::None => {}
            ParseResult::Err => return ParseResult::Err,
        };
        ParseResult::None
    }
    fn parse_val_array(&mut self) -> ParseResult<Vec<Value>> {
        let mut array: Vec<Value> = Vec::new();
        if let Some(c) = self.peek() {
            if c != '[' {
                return ParseResult::None;
            }
            self.get_next();
        } else {
            return ParseResult::None;
        }
        match self.parse_value() {
            ParseResult::Val(val) => array.push(val),
            ParseResult::None => {
                if let Some(c) = self.peek() {
                    if c == ']' {
                        self.get_next();
                        return ParseResult::Val(array);
                    } else {
                        log_error!("expecting a ']' at the end of the array");
                        return ParseResult::Err;
                    }
                }
            }
            ParseResult::Err => return ParseResult::Err,
        }
        while let Some(c) = self.peek() {
            if c == ',' {
                self.get_next();
                let t = self.parse_value();
                match t {
                    ParseResult::Val(val) => array.push(val),
                    ParseResult::None => {}
                    ParseResult::Err => return ParseResult::Err,
                }
            } else {
                break;
            }
        }
        match self.peek() {
            Some(c) => {
                if c == ']' {
                    self.get_next();
                    return ParseResult::Val(array);
                } else {
                    log_error!("expecting a ']' at the end of the array");
                    return ParseResult::Err;
                }
            }
            None => {
                log_error!("expecting a ']' at the end of the array");
                return ParseResult::Err;
            }
        }
    }
    fn parse_literal(&mut self) -> ParseResult<String> {
        let mut literal = String::new();
        let quote: char;
        if let Some(c) = self.peek() {
            if c != '\"' && c != '\'' {
                return ParseResult::None;
            }
            quote = c;
            self.get_next();
        } else {
            return ParseResult::None;
        }

        while let Some(c) = self.peek() {
            if c == quote {
                self.get_next();
                return ParseResult::Val(literal);
            }
            literal.push(c);
            self.get_next();
        }

        log_error!("expecting a '{quote}' at the end of a literal");
        return ParseResult::Err;
    }
    fn parse_number(&mut self) -> ParseResult<f32> {
        let number_chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
        let mut number_as_string = String::new();
        while let Some(c) = self.peek() {
            if number_chars.contains(&c) {
                self.get_next();
                number_as_string.push(c);
            } else {
                break;
            }
        }

        match number_as_string.parse::<f32>() {
            Ok(number) => return ParseResult::Val(number),
            Err(_) => {
                if number_as_string.is_empty() {
                    return ParseResult::None;
                } else {
                    return ParseResult::Err;
                }
            }
        }
    }
    fn parse_boolean(&mut self) -> ParseResult<bool> {
        let mut boolean_as_string = String::new();
        if let Some(c) = self.peek() {
            if c == 'f' {
                for _ in 0..5 {
                    if let Some(c) = self.peek() {
                        self.get_next();
                        boolean_as_string.push(c);
                    } else {
                        return ParseResult::Err;
                    }
                }
                if boolean_as_string == "false".to_string() {
                    return ParseResult::Val(false);
                }
                return ParseResult::Err;
            } else if c == 't' {
                for _ in 0..4 {
                    if let Some(c) = self.peek() {
                        self.get_next();
                        boolean_as_string.push(c);
                    } else {
                        return ParseResult::Err;
                    }
                }
                if boolean_as_string == "true".to_string() {
                    return ParseResult::Val(true);
                }
                return ParseResult::Err;
            } else {
                return ParseResult::None;
            }
        } else {
            return ParseResult::None;
        }
    }
    fn parse_null(&mut self) -> ParseResult<()> {
        let mut null_as_string = String::new();
        if let Some(c) = self.peek() {
            if c == 'n' {
                for _ in 0..4 {
                    if let Some(c) = self.peek() {
                        self.get_next();
                        null_as_string.push(c);
                    } else {
                        return ParseResult::Err;
                    }
                }
                if null_as_string == "null".to_string() {
                    return ParseResult::Val(());
                }
                return ParseResult::Err;
            } else {
                return ParseResult::None;
            }
        }
        return ParseResult::None;
    }
}

#[cfg(test)]
mod json_parser_tests {
    use super::*;

    #[test]
    fn parse_literal_test() {
        let mut parser = Parser {
            index: 0,
            file_as_string: "'baz'".to_string(),
        };
        assert_eq!(parser.parse_literal(), ParseResult::Val("baz".to_string()));
    }
    #[test]
    fn parse_number_test() {
        let mut parser = Parser {
            index: 0,
            file_as_string: "40.8".to_string(),
        };
        assert_eq!(parser.parse_number(), ParseResult::Val(40.8));
    }
    #[test]
    fn parse_boolean_test() {
        let mut parser = Parser {
            index: 0,
            file_as_string: "true".to_string(),
        };
        assert_eq!(parser.parse_boolean(), ParseResult::Val(true));
    }
    #[test]
    fn parse_null_test() {
        let mut parser = Parser {
            index: 0,
            file_as_string: "null".to_string(),
        };
        assert_eq!(parser.parse_null(), ParseResult::Val(()));
    }
    #[test]
    fn parse_val_array_test() {
        let mut parser = Parser {
            index: 0,
            file_as_string: "['foo', 'baz', 'bar']".to_string(),
        };
        let expected_arr: Vec<Value> = vec![
            Value::Literal("foo".to_string()),
            Value::Literal("baz".to_string()),
            Value::Literal("bar".to_string()),
        ];
        assert_eq!(parser.parse_val_array(), ParseResult::Val(expected_arr));
    }
}
