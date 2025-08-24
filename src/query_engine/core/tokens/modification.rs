use super::ParseResult;
use crate::log_error;
use super::value;
use super::value::Value;

#[derive(Debug)]
enum ArithmeticModifier {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
}
#[derive(Debug)]
enum StringModifier {
    Concatenate,
    ToUpperCase,
    ToLowerCase,
}
#[derive(Debug)]
enum Modifier {
    ArithmeticModifier(ArithmeticModifier),
    StringModifier(StringModifier),
}

impl Modifier {
    pub fn get_modifier_from_lexeme(lexeme: &String) -> Self {
        match lexeme.as_str() {
            // Arithmetic modifiers
            "+" => Modifier::ArithmeticModifier(ArithmeticModifier::Plus),
            "-" => Modifier::ArithmeticModifier(ArithmeticModifier::Minus),
            "*" => Modifier::ArithmeticModifier(ArithmeticModifier::Multiply),
            "/" => Modifier::ArithmeticModifier(ArithmeticModifier::Divide),
            "%" => Modifier::ArithmeticModifier(ArithmeticModifier::Modulo),
            "^" => Modifier::ArithmeticModifier(ArithmeticModifier::Power),

            // String modifiers
            "||" => Modifier::StringModifier(StringModifier::Concatenate),
            "to-upper" => Modifier::StringModifier(StringModifier::ToUpperCase),
            "to-lower" => Modifier::StringModifier(StringModifier::ToLowerCase),
            _ => unreachable!("invalid modifier {lexeme}"),
        }
    }
}
#[derive(Debug)]
pub struct Modification {
    lhs: Value,
    modifier: Option<Modifier>,
    rhs: Option<Value>,
}

impl Modification {
    pub fn parse(lexemes: &[String], idx: usize) -> (ParseResult<Self>, usize) {
        match lexemes.get(idx) {
            Some(lexeme1) => {
                // lexeme1 must be a valid value for lhs
                let lhs: Value;
                let modifier: Option<Modifier>;
                let rhs: Option<Value>;
                if let Some(field_name) = value::parse_field_name(lexeme1) {
                    lhs = Value::FieldName(field_name);
                } else if let Some(number) = value::parse_number(lexeme1) {
                    lhs = Value::Number(number);
                } else if let Some(literal) = value::parse_literal(lexeme1) {
                    lhs = Value::Literal(literal);
                } else {
                    return {
                        log_error!("{lexeme1} is not a valid value to be modified");
                        return (ParseResult::Err, idx);
                    };
                }
                match lexemes.get(idx + 1) {
                    Some(lexeme2) => match lexeme2.as_str() {
                        // lexeme2 must be a valid modifier
                        "+" | "-" | "*" | "/" | "%" | "^" | "||" => {
                            modifier = Some(Modifier::get_modifier_from_lexeme(lexeme2));
                            match lexemes.get(idx + 2) {
                                Some(lexeme3) => {
                                    // lexeme3 must be a valid value for the rhs
                                    if let Some(field_name) = value::parse_field_name(lexeme3) {
                                        rhs = Some(Value::FieldName(field_name));
                                    } else if let Some(number) = value::parse_number(lexeme3) {
                                        rhs = Some(Value::Number(number));
                                    } else if let Some(literal) = value::parse_literal(lexeme3) {
                                        rhs = Some(Value::Literal(literal));
                                    } else {
                                        log_error!("{lexeme3} is not a valid value to be modified");
                                        return (ParseResult::Err, idx);
                                    }
                                    return (
                                        ParseResult::Val(Modification { lhs, modifier, rhs }),
                                        idx + 2,
                                    );
                                }
                                None => {
                                    // the rhs was not provided
                                    log_error!(
                                        "the modifier {lexeme2} require a right hand side value"
                                    );
                                    return (ParseResult::Err, idx);
                                }
                            }
                        }
                        "to-lower" | "to-upper" => {
                            modifier = Some(Modifier::get_modifier_from_lexeme(lexeme2));
                            return (
                                ParseResult::Val(Modification {
                                    lhs,
                                    modifier,
                                    rhs: None,
                                }),
                                idx + 1,
                            );
                        }
                        _ => {
                            // the modifier was not provided
                            return (
                                ParseResult::Val(Modification {
                                    lhs,
                                    modifier: None,
                                    rhs: None,
                                }),
                                idx,
                            );
                        }
                    },
                    None => {
                        // the modifier was not provided
                        return (
                            ParseResult::Val(Modification {
                                lhs,
                                modifier: None,
                                rhs: None,
                            }),
                            idx,
                        );
                    }
                }
            }
            None => return (ParseResult::None, idx),
        }
    }
    pub fn evaluate(&self, fields: &Vec<String>, row: &Vec<String>) -> Option<String> {
        match &self.modifier {
            Some(modifier) => match modifier {
                Modifier::ArithmeticModifier(arithmitec_modifier) => {
                    let lhs: f32;
                    let rhs: f32;
                    match &self.lhs {
                        // extracting the value of the lhs
                        Value::Number(number) => lhs = *number,
                        Value::FieldName(field_name) => {
                            match fields.iter().position(|f| f == field_name) {
                                Some(idx) => match row[idx].parse::<f32>() {
                                    Ok(val) => lhs = val,
                                    Err(_) => {
                                        log_error!(
                                            "the value {} of the field {} is not numerical",
                                            row[idx], field_name
                                        );
                                        return None;
                                    }
                                },
                                None => {
                                    log_error!("no field named {}", field_name);
                                    return None;
                                }
                            }
                        }
                        _ => {
                            log_error!(
                                "an arithmetic operator can not be applied to a non numeric value"
                            );
                            return None;
                        }
                    }
                    match &self.rhs {
                        // extracting the value of the rhs
                        Some(rhs_val) => match rhs_val {
                            Value::Number(number) => rhs = *number,
                            Value::FieldName(field_name) => {
                                match fields.iter().position(|f| f == field_name) {
                                    Some(idx) => match row[idx].parse::<f32>() {
                                        Ok(val) => rhs = val,
                                        Err(_) => {
                                            log_error!(
                                                "the value {} of {} is not numerical",
                                                row[idx], field_name
                                            );
                                            return None;
                                        }
                                    },
                                    None => {
                                        log_error!("no field named {}", field_name);
                                        return None;
                                    }
                                }
                            }
                            _ => {
                                log_error!(
                                    "an arithmetic operator can not be applied to a non numeric value"
                                );
                                return None;
                            }
                        },
                        None => {
                            log_error!("expecting a rhs value after the arithmetic operator");
                            return None;
                        }
                    }
                    match arithmitec_modifier {
                        ArithmeticModifier::Plus => return Some((lhs + rhs).to_string()),
                        ArithmeticModifier::Minus => return Some((lhs - rhs).to_string()),
                        ArithmeticModifier::Multiply => return Some((lhs * rhs).to_string()),
                        ArithmeticModifier::Divide => {
                            if rhs == 0f32 {
                                log_error!("can not divide by 0");
                                return Some(lhs.to_string());
                            }
                            return Some((lhs / rhs).to_string());
                        }
                        ArithmeticModifier::Modulo => return Some((lhs % rhs).to_string()),
                        ArithmeticModifier::Power => return Some((lhs.powf(rhs)).to_string()),
                    }
                }
                Modifier::StringModifier(string_modifier) => {
                    let lhs: String;
                    let rhs: Option<String>;
                    match &self.lhs {
                      // extracting the lhs
                        Value::Literal(val) => lhs = val.clone(),
                        Value::FieldName(field_name) => {
                            match fields.iter().position(|f| f == field_name) {
                                Some(idx) => lhs = row[idx].clone(),
                                None => {
                                    log_error!("no field named {}", field_name);
                                    return None;
                                }
                            }
                        }
                        _ => {
                            log_error!(
                                "a string operator can not be applied to a non numeric value"
                            );
                            return None;
                        }
                    }
                    match &self.rhs {
                      // extracting the rhs
                        Some(rhs_val) => match rhs_val {
                            Value::Literal(val) => rhs = Some(val.clone()),
                            Value::FieldName(field_name) => {
                                match fields.iter().position(|f| f == field_name) {
                                    Some(idx) => rhs = Some(row[idx].clone()),
                                    None => {
                                        log_error!("no field named {}", field_name);
                                        return None;
                                    }
                                }
                            }
                            _ => {
                                log_error!(
                                    "a string operator can not be applied to a value that is not a string"
                                );
                                return None;
                            }
                        },
                        None => {
                            rhs = None;
                        }
                    }
                    match rhs {
                        Some(val) => match string_modifier {
                            StringModifier::Concatenate => return Some(lhs + &val),
                            _ => {}
                        },
                        None => match string_modifier {
                            StringModifier::ToUpperCase => return Some(lhs.to_uppercase()),
                            StringModifier::ToLowerCase => return Some(lhs.to_lowercase()),
                            _ => {
                                log_error!("messing the rhs for the modifier");
                                return None;
                            }
                        },
                    }
                }
            },
            None => match &self.lhs {
              // the case where the modification is just a value
                Value::FieldName(val) => match fields.iter().position(|f| f == val) {
                    Some(idx) => return Some(row[idx].clone()),
                    None => {
                        log_error!("no field named {}", val);
                        return None;
                    }
                },
                Value::Literal(val) => return Some(val.clone()),
                Value::Number(val) => return Some(val.to_string()),
                _ => {
                    log_error!("the value {} can not be assigned to a field", self.lhs);
                    return None;
                }
            },
        }
        None
    }
}

#[cfg(test)]
mod modification_tests {
    use super::*;

    fn get_data() -> (Vec<String>, Vec<String>) {
        let fields = vec!["name".to_string(), "age".to_string(), "city".to_string()];
        let row = vec!["bob".to_string(), "45".to_string(), "London".to_string()];
        (fields, row)
    }

    #[test]
    fn arithmetic_plus() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("age".to_string()),
            modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Plus)),
            rhs: Some(Value::Number(5.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("50".to_string()));
    }

    #[test]
    fn arithmetic_minus() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("age".to_string()),
            modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Minus)),
            rhs: Some(Value::Number(5.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("40".to_string()));
    }

    #[test]
    fn arithmetic_multiply() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("age".to_string()),
            modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Multiply)),
            rhs: Some(Value::Number(2.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("90".to_string()));
    }

    #[test]
    fn arithmetic_divide() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("age".to_string()),
            modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Divide)),
            rhs: Some(Value::Number(5.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("9".to_string()));
    }

    #[test]
    fn arithmetic_modulo() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("age".to_string()),
            modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Modulo)),
            rhs: Some(Value::Number(7.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("3".to_string()));
    }

    #[test]
    fn arithmetic_power() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("age".to_string()),
            modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Power)),
            rhs: Some(Value::Number(2.0)),
        };
        assert_eq!(
            modification.evaluate(&fields, &row),
            Some("2025".to_string())
        );
    }

    #[test]
    fn string_concatenate() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("name".to_string()),
            modifier: Some(Modifier::StringModifier(StringModifier::Concatenate)),
            rhs: Some(Value::Literal(" Smith".to_string())),
        };
        assert_eq!(
            modification.evaluate(&fields, &row),
            Some("bob Smith".to_string())
        );
    }

    #[test]
    fn string_to_upper_case() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("city".to_string()),
            modifier: Some(Modifier::StringModifier(StringModifier::ToUpperCase)),
            rhs: None,
        };
        assert_eq!(
            modification.evaluate(&fields, &row),
            Some("LONDON".to_string())
        );
    }

    #[test]
    fn string_to_lower_case() {
        let (fields, mut row) = get_data();
        row[2] = "LONDON".to_string(); // Override city to be uppercase

        let modification = Modification {
            lhs: Value::FieldName("city".to_string()),
            modifier: Some(Modifier::StringModifier(StringModifier::ToLowerCase)),
            rhs: None,
        };
        assert_eq!(
            modification.evaluate(&fields, &row),
            Some("london".to_string())
        );
    }

    #[test]
    fn invalid_operations() {
        let (fields, row) = get_data();

        // Arithmetic operation on string field
        let modification = Modification {
            lhs: Value::FieldName("name".to_string()),
            modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Plus)),
            rhs: Some(Value::Number(5.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), None);

        // Missing rhs for concatenation
        let modification = Modification {
            lhs: Value::FieldName("name".to_string()),
            modifier: Some(Modifier::StringModifier(StringModifier::Concatenate)),
            rhs: None,
        };
        assert_eq!(modification.evaluate(&fields, &row), None);
    }
}
