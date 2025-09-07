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

#[derive(Debug, PartialEq)]
pub struct Object {
    array: Option<Vec<Object>>,
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
                '[' => match self.parse_array() {
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
    fn get_next_and_white_space(&mut self) -> Option<char> {
        if let Some(c) = &self.file_as_string.chars().nth(self.index) {
            self.index += 1;
            return Some(*c);
        }
        return None;
    }
    fn peek_and_white_space(&self) -> Option<char> {
        if let Some(c) = &self.file_as_string.chars().nth(self.index) {
            return Some(*c);
        }
        return None;
    }

    fn parse_object(&mut self) -> ParseResult<Object> {
        match self.parse_single_val_obj() {
            ParseResult::Val(obj) => return ParseResult::Val(obj),
            ParseResult::None => {}
            ParseResult::Err => return ParseResult::Err,
        }
        if let Some(c) = self.peek() {
            if c == '{' {
                self.get_next();
                match self.parse_fields() {
                    ParseResult::Val(fields) => {
                        if let Some(c) = self.peek() {
                            if c == '}' {
                                self.get_next();
                                return ParseResult::Val(Object {
                                    array: None,
                                    fields: Some(fields),
                                    value: None,
                                });
                            }
                        } else {
                            log_error!("expecting a '}}' at the end of the json object");
                            return ParseResult::Err;
                        }
                    }
                    ParseResult::None => {
                        if let Some(c) = self.peek() {
                            if c == '}' {
                                return ParseResult::Val(Object {
                                    array: None,
                                    fields: None,
                                    value: None,
                                });
                            }
                        } else {
                            log_error!("expecting a '}}' at the end of the json object");
                            return ParseResult::Err;
                        }
                    }
                    ParseResult::Err => return ParseResult::Err,
                }
            }
        }
        match self.parse_array() {
            ParseResult::Val(arr) => {
                return ParseResult::Val(Object {
                    array: Some(arr),
                    fields: None,
                    value: None,
                });
            }
            ParseResult::None => return ParseResult::None,
            ParseResult::Err => return ParseResult::Err,
        }
    }

    fn parse_single_val_obj(&mut self) -> ParseResult<Object> {
        let saved_idx = self.index;
        let obj: Object;
        match self.parse_value() {
            ParseResult::Val(val) => {
                obj = Object {
                    array: None,
                    fields: None,
                    value: Some(val),
                }
            }
            ParseResult::None => return ParseResult::None,
            ParseResult::Err => return ParseResult::Err,
        }

        if let Some(c) = self.peek() {
            match c {
                ',' | ']' | '}' => return ParseResult::Val(obj),
                _ => {
                    self.index = saved_idx;
                    return ParseResult::None;
                }
            }
        } else {
            return ParseResult::Val(obj);
        }
    }

    fn parse_array(&mut self) -> ParseResult<Vec<Object>> {
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
                if c == ']' {
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
                if c != '"' && c != '\'' {
                    return ParseResult::None;
                }
            }
            None => return ParseResult::None,
        }
        match self.parse_key_val_pair() {
            ParseResult::Val((name, value)) => fields.insert(name, value),
            ParseResult::None => return ParseResult::None,
            ParseResult::Err => return ParseResult::Err,
        };
        while let Some(c) = self.peek() {
            if c == ',' {
                self.get_next();
                match self.parse_key_val_pair() {
                    ParseResult::Val((name, value)) => fields.insert(name, value),
                    ParseResult::None => return ParseResult::Val(fields),
                    ParseResult::Err => return ParseResult::Err,
                };
            } else {
                break;
            }
        }
        ParseResult::Val(fields)
    }
    fn parse_key_val_pair(&mut self) -> ParseResult<(String, Object)> {
        let name: String;
        let value: Object;
        match self.peek() {
            Some(c) => {
                if c != '"' && c != '\'' {
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
            match self.parse_object() {
                ParseResult::Val(obj) => value = obj,
                ParseResult::None => {
                    log_error!("expecting a value after '{name:}'");
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
    // fn parse_val_array(&mut self) -> ParseResult<Vec<Value>> {
    //     let mut array: Vec<Value> = Vec::new();
    //     if let Some(c) = self.peek() {
    //         if c != '[' {
    //             return ParseResult::None;
    //         }
    //         self.get_next();
    //     } else {
    //         return ParseResult::None;
    //     }
    //     match self.parse_value() {
    //         ParseResult::Val(val) => array.push(val),
    //         ParseResult::None => {
    //             if let Some(c) = self.peek() {
    //                 if c == ']' {
    //                     self.get_next();
    //                     return ParseResult::Val(array);
    //                 } else {
    //                     log_error!("expecting a ']' at the end of the array");
    //                     return ParseResult::Err;
    //                 }
    //             }
    //         }
    //         ParseResult::Err => return ParseResult::Err,
    //     }
    //     while let Some(c) = self.peek() {
    //         if c == ',' {
    //             self.get_next();
    //             let t = self.parse_value();
    //             match t {
    //                 ParseResult::Val(val) => array.push(val),
    //                 ParseResult::None => {}
    //                 ParseResult::Err => return ParseResult::Err,
    //             }
    //         } else {
    //             break;
    //         }
    //     }
    //     match self.peek() {
    //         Some(c) => {
    //             if c == ']' {
    //                 self.get_next();
    //                 return ParseResult::Val(array);
    //             } else {
    //                 log_error!("expecting a ']' at the end of the array");
    //                 return ParseResult::Err;
    //             }
    //         }
    //         None => {
    //             log_error!("expecting a ']' at the end of the array");
    //             return ParseResult::Err;
    //         }
    //     }
    // }
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

        while let Some(c) = self.peek_and_white_space() {
            if c == quote {
                self.get_next();
                return ParseResult::Val(literal);
            }
            literal.push(c);
            self.get_next_and_white_space();
        }

        log_error!("expecting a '{quote}' at the end of a literal");
        return ParseResult::Err;
    }
    fn parse_number(&mut self) -> ParseResult<f32> {
        let number_chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '-'];
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
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_parser(input: &str) -> Parser {
        Parser {
            index: 0,
            file_as_string: input.to_string(),
        }
    }

    #[test]
    fn test_parse_empty_array() {
        let mut parser = create_parser("[]");
        let result = parser.parse_object();

        assert_eq!(
            result,
            ParseResult::Val(Object {
                array: Some(vec![]),
                fields: None,
                value: None,
            })
        );
    }

    #[test]
    fn test_parse_string_array() {
        let mut parser = create_parser("['reading', 'coding']");
        let result = parser.parse_object();

        let expected = Object {
            array: Some(vec![
                Object {
                    array: None,
                    fields: None,
                    value: Some(Value::Literal("reading".to_string())),
                },
                Object {
                    array: None,
                    fields: None,
                    value: Some(Value::Literal("coding".to_string())),
                },
            ]),
            fields: None,
            value: None,
        };

        assert_eq!(result, ParseResult::Val(expected));
    }

    #[test]
    fn test_parse_mixed_array() {
        let mut parser = create_parser("[123, 'hello', true]");
        let result = parser.parse_object();

        let expected = Object {
            array: Some(vec![
                Object {
                    array: None,
                    fields: None,
                    value: Some(Value::Number(123.0)),
                },
                Object {
                    array: None,
                    fields: None,
                    value: Some(Value::Literal("hello".to_string())),
                },
                Object {
                    array: None,
                    fields: None,
                    value: Some(Value::Boolean(true)),
                },
            ]),
            fields: None,
            value: None,
        };

        assert_eq!(result, ParseResult::Val(expected));
    }

    #[test]
    fn test_parse_nested_array() {
        let mut parser = create_parser("[['nested'], 'item']");
        let result = parser.parse_object();

        let expected = Object {
            array: Some(vec![
                Object {
                    array: Some(vec![Object {
                        array: None,
                        fields: None,
                        value: Some(Value::Literal("nested".to_string())),
                    }]),
                    fields: None,
                    value: None,
                },
                Object {
                    array: None,
                    fields: None,
                    value: Some(Value::Literal("item".to_string())),
                },
            ]),
            fields: None,
            value: None,
        };

        assert_eq!(result, ParseResult::Val(expected));
    }

    #[test]
    fn test_parse_empty_object() {
        let mut parser = create_parser("{}");
        let result = parser.parse_object();

        assert_eq!(
            result,
            ParseResult::Val(Object {
                array: None,
                fields: None,
                value: None,
            })
        );
    }

    #[test]
    fn test_parse_simple_object() {
        let mut parser = create_parser(r#"{ "name": "Alice", "age": 25 }"#);
        let result = parser.parse_object();

        let mut fields = HashMap::new();
        fields.insert(
            "name".to_string(),
            Object {
                array: None,
                fields: None,
                value: Some(Value::Literal("Alice".to_string())),
            },
        );
        fields.insert(
            "age".to_string(),
            Object {
                array: None,
                fields: None,
                value: Some(Value::Number(25.0)),
            },
        );

        assert_eq!(
            result,
            ParseResult::Val(Object {
                array: None,
                fields: Some(fields),
                value: None,
            })
        );
    }

    #[test]
    fn test_parse_object_with_array() {
        let mut parser = create_parser(r#"{ "hobbies": ["reading", "coding"] }"#);
        let result = parser.parse_object();

        let mut fields = HashMap::new();
        fields.insert(
            "hobbies".to_string(),
            Object {
                array: Some(vec![
                    Object {
                        array: None,
                        fields: None,
                        value: Some(Value::Literal("reading".to_string())),
                    },
                    Object {
                        array: None,
                        fields: None,
                        value: Some(Value::Literal("coding".to_string())),
                    },
                ]),
                fields: None,
                value: None,
            },
        );

        assert_eq!(
            result,
            ParseResult::Val(Object {
                array: None,
                fields: Some(fields),
                value: None,
            })
        );
    }

    #[test]
    fn test_parse_boolean_values() {
        let mut parser = create_parser("[true, false]");
        let result = parser.parse_object();

        let expected = Object {
            array: Some(vec![
                Object {
                    array: None,
                    fields: None,
                    value: Some(Value::Boolean(true)),
                },
                Object {
                    array: None,
                    fields: None,
                    value: Some(Value::Boolean(false)),
                },
            ]),
            fields: None,
            value: None,
        };

        assert_eq!(result, ParseResult::Val(expected));
    }

    #[test]
    fn test_parse_null_value() {
        let mut parser = create_parser("[null]");
        let result = parser.parse_object();

        let expected = Object {
            array: Some(vec![Object {
                array: None,
                fields: None,
                value: Some(Value::Null),
            }]),
            fields: None,
            value: None,
        };

        assert_eq!(result, ParseResult::Val(expected));
    }

    #[test]
    fn test_parse_number_array() {
        let mut parser = create_parser("[1.5, -2.3, 0]");
        let result = parser.parse_object();

        let expected = Object {
            array: Some(vec![
                Object {
                    array: None,
                    fields: None,
                    value: Some(Value::Number(1.5)),
                },
                Object {
                    array: None,
                    fields: None,
                    value: Some(Value::Number(-2.3)),
                },
                Object {
                    array: None,
                    fields: None,
                    value: Some(Value::Number(0.0)),
                },
            ]),
            fields: None,
            value: None,
        };

        assert_eq!(result, ParseResult::Val(expected));
    }

    #[test]
    fn test_parse_invalid_input() {
        let mut parser = create_parser("[invalid");
        let result = parser.parse_object();

        assert_eq!(result, ParseResult::Err);
    }

    #[test]
    fn test_parse_unexpected_end() {
        let mut parser = create_parser("[");
        let result = parser.parse_object();

        assert_eq!(result, ParseResult::Err);
    }

    #[test]
    fn test_parse_empty_string() {
        let mut parser = create_parser("");
        let result = parser.parse_object();

        assert_eq!(result, ParseResult::None);
    }

    #[test]
    fn test_parse_whitespace_only() {
        let mut parser = create_parser("   \t\n  ");
        let result = parser.parse_object();

        assert_eq!(result, ParseResult::None);
    }
}
