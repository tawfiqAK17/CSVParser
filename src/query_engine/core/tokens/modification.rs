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

#[derive(Debug)]
pub struct Modification {
    lhs: Value,
    modifier: Modifier,
    rhs: Option<Value>,
}

impl Modification {
    pub fn evaluate(&self, fields: &Vec<String>, row: &Vec<String>) -> Option<String> {
        match &self.modifier {
            Modifier::ArithmeticModifier(arithmitec_modifier) => {
                let lhs: f32;
                let rhs: f32;
                match &self.lhs {
                    Value::Number(number) => lhs = *number,
                    Value::FieldName(field_name) => {
                        match fields.iter().position(|f| f == field_name) {
                            Some(idx) => match row[idx].parse::<f32>() {
                                Ok(val) => lhs = val,
                                Err(_) => {
                                    eprintln!(
                                        "the value {} of {} is not numerical",
                                        row[idx], field_name
                                    );
                                    return None;
                                }
                            },
                            None => {
                                eprintln!("no field named {}", field_name);
                                return None;
                            }
                        }
                    }
                    _ => {
                        eprintln!(
                            "an arithmetic operator can not be applied to a non numeric value"
                        );
                        return None;
                    }
                }
                match &self.rhs {
                    Some(rhs_val) => match rhs_val {
                        Value::Number(number) => rhs = *number,
                        Value::FieldName(field_name) => {
                            match fields.iter().position(|f| f == field_name) {
                                Some(idx) => match row[idx].parse::<f32>() {
                                    Ok(val) => rhs = val,
                                    Err(_) => {
                                        eprintln!(
                                            "the value {} of {} is not numerical",
                                            row[idx], field_name
                                        );
                                        return None;
                                    }
                                },
                                None => {
                                    eprintln!("no field named {}", field_name);
                                    return None;
                                }
                            }
                        }
                        _ => {
                            eprintln!(
                                "an arithmetic operator can not be applied to a non numeric value"
                            );
                            return None;
                        }
                    },
                    None => {
                        eprintln!("expecting a rhs value after the arithmetic operator");
                        return None;
                    }
                }
                match arithmitec_modifier {
                    ArithmeticModifier::Plus => return Some((lhs + rhs).to_string()),
                    ArithmeticModifier::Minus => return Some((lhs - rhs).to_string()),
                    ArithmeticModifier::Multiply => return Some((lhs * rhs).to_string()),
                    ArithmeticModifier::Divide => return Some((lhs / rhs).to_string()),
                    ArithmeticModifier::Modulo => return Some((lhs % rhs).to_string()),
                    ArithmeticModifier::Power => return Some((lhs.powf(rhs)).to_string()),
                }
            }
            Modifier::StringModifier(string_modifier) => {
                let lhs: String;
                let rhs: Option<String>;
                match &self.lhs {
                    Value::Literal(val) => lhs = val.clone(),
                    Value::FieldName(field_name) => {
                        match fields.iter().position(|f| f == field_name) {
                            Some(idx) => lhs = row[idx].clone(),
                            None => {
                                eprintln!("no field named {}", field_name);
                                return None;
                            }
                        }
                    }
                    _ => {
                        eprintln!("a string operator can not be applied to a non numeric value");
                        return None;
                    }
                }
                match &self.rhs {
                    Some(rhs_val) => match rhs_val {
                        Value::Literal(val) => rhs = Some(val.clone()),
                        Value::FieldName(field_name) => {
                            match fields.iter().position(|f| f == field_name) {
                                Some(idx) => rhs = Some(row[idx].clone()),
                                None => {
                                    eprintln!("no field named {}", field_name);
                                    return None;
                                }
                            }
                        }
                        _ => {
                            eprintln!(
                                "a string operator can not be applied to a non numeric value"
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
                        _ => {}
                    },
                }
            }
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
            modifier: Modifier::ArithmeticModifier(ArithmeticModifier::Plus),
            rhs: Some(Value::Number(5.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("50".to_string()));
    }

    #[test]
    fn arithmetic_minus() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("age".to_string()),
            modifier: Modifier::ArithmeticModifier(ArithmeticModifier::Minus),
            rhs: Some(Value::Number(5.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("40".to_string()));
    }

    #[test]
    fn arithmetic_multiply() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("age".to_string()),
            modifier: Modifier::ArithmeticModifier(ArithmeticModifier::Multiply),
            rhs: Some(Value::Number(2.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("90".to_string()));
    }

    #[test]
    fn arithmetic_divide() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("age".to_string()),
            modifier: Modifier::ArithmeticModifier(ArithmeticModifier::Divide),
            rhs: Some(Value::Number(5.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("9".to_string()));
    }

    #[test]
    fn arithmetic_modulo() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("age".to_string()),
            modifier: Modifier::ArithmeticModifier(ArithmeticModifier::Modulo),
            rhs: Some(Value::Number(7.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("3".to_string()));
    }

    #[test]
    fn arithmetic_power() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("age".to_string()),
            modifier: Modifier::ArithmeticModifier(ArithmeticModifier::Power),
            rhs: Some(Value::Number(2.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("2025".to_string()));
    }

    #[test]
    fn string_concatenate() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("name".to_string()),
            modifier: Modifier::StringModifier(StringModifier::Concatenate),
            rhs: Some(Value::Literal(" Smith".to_string())),
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("bob Smith".to_string()));
    }

    #[test]
    fn string_to_upper_case() {
        let (fields, row) = get_data();
        let modification = Modification {
            lhs: Value::FieldName("city".to_string()),
            modifier: Modifier::StringModifier(StringModifier::ToUpperCase),
            rhs: None,
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("LONDON".to_string()));
    }

    #[test]
    fn string_to_lower_case() {
        let (fields, mut row) = get_data();
        row[2] = "LONDON".to_string(); // Override city to be uppercase
        
        let modification = Modification {
            lhs: Value::FieldName("city".to_string()),
            modifier: Modifier::StringModifier(StringModifier::ToLowerCase),
            rhs: None,
        };
        assert_eq!(modification.evaluate(&fields, &row), Some("london".to_string()));
    }

    #[test]
    fn invalid_operations() {
        let (fields, row) = get_data();
        
        // Arithmetic operation on string field
        let modification = Modification {
            lhs: Value::FieldName("name".to_string()),
            modifier: Modifier::ArithmeticModifier(ArithmeticModifier::Plus),
            rhs: Some(Value::Number(5.0)),
        };
        assert_eq!(modification.evaluate(&fields, &row), None);
        
        // Missing rhs for concatenation
        let modification = Modification {
            lhs: Value::FieldName("name".to_string()),
            modifier: Modifier::StringModifier(StringModifier::Concatenate),
            rhs: None,
        };
        assert_eq!(modification.evaluate(&fields, &row), None);
    }
}
