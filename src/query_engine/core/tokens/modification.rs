use super::ParseResult;
use super::value;
use super::value::Value;
use crate::log_error;

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
    additional_modifier: Option<Modifier>, // for the cases where the modifier does not accept a
    // rhs value, by adding this additional modifier the
    // modifications with no rhs can be nested
    rhs: Option<Box<Modification>>,
}

impl Modification {
    pub fn parse(lexemes: &[String], idx: usize) -> (ParseResult<Self>, usize) {
        match lexemes.get(idx) {
            Some(lexeme1) => {
                // lexeme1 must be a valid value for lhs
                let lhs: Value;
                let modifier: Option<Modifier>;
                let rhs: Option<Box<Modification>>;

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
                    Some(lexeme2) => {
                        match lexeme2.as_str() {
                            // lexeme2 must be a valid modifier
                            "+" | "-" | "*" | "/" | "%" | "^" | "||" => {
                                modifier = Some(Modifier::get_modifier_from_lexeme(lexeme2));
                                match lexemes.get(idx + 2) {
                                    Some(_) => {
                                        let (modification_parse_result, last_idx) =
                                            Modification::parse(lexemes, idx + 2);
                                        match modification_parse_result {
                                            ParseResult::Val(modification) => {
                                                rhs = Some(Box::new(modification))
                                            }
                                            ParseResult::None => {
                                                log_error!(
                                                    "the modifier {lexeme2} require a right hand side value"
                                                );
                                                return (ParseResult::Err, idx);
                                            }
                                            ParseResult::Err => return (ParseResult::Err, idx),
                                        }
                                        return (
                                            ParseResult::Val(Modification {
                                                lhs,
                                                modifier,
                                                additional_modifier: None,
                                                rhs,
                                            }),
                                            last_idx,
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
                                let mut additional_modifier: Option<Modifier> = None;
                                let mut rhs: Option<Box<Modification>> = None;
                                let mut last_idx = idx + 1;
                                match lexemes.get(idx + 2) {
                                    Some(lexeme) => match lexeme.as_str() {
                                        "+" | "-" | "*" | "/" | "%" | "^" | "||" => {
                                            let (modification_parse_result, last_idx1) =
                                                Modification::parse(lexemes, idx + 3);
                                            match modification_parse_result {
                                                ParseResult::Val(modification) => {
                                                    additional_modifier = Some(
                                                        Modifier::get_modifier_from_lexeme(lexeme),
                                                    );
                                                    rhs = Some(Box::new(modification));
                                                    last_idx = last_idx1;
                                                }
                                                ParseResult::None => {
                                                    log_error!(
                                                        "expecting a modification after the modifier '{}'",
                                                        lexeme
                                                    );
                                                    return (ParseResult::Err, idx);
                                                }
                                                ParseResult::Err => return (ParseResult::Err, idx),
                                            }
                                        }
                                        _ => {}
                                    },
                                    None => {}
                                }
                                return (
                                    ParseResult::Val(Modification {
                                        lhs,
                                        modifier,
                                        additional_modifier,
                                        rhs,
                                    }),
                                    last_idx,
                                );
                            }
                            _ => {
                                // the modifier was not provided
                                return (
                                    ParseResult::Val(Modification {
                                        lhs,
                                        modifier: None,
                                        additional_modifier: None,
                                        rhs: None,
                                    }),
                                    idx,
                                );
                            }
                        }
                    }
                    None => {
                        // the modifier was not provided
                        return (
                            ParseResult::Val(Modification {
                                lhs,
                                modifier: None,
                                additional_modifier: None,
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
                                            row[idx],
                                            field_name
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
                        Some(modifier) => match modifier.evaluate(fields, row) {
                            Some(lhs_as_str) => match lhs_as_str.parse::<f32>() {
                                Ok(number) => rhs = number,
                                Err(_) => return None,
                            },
                            None => {
                                log_error!("expecting a rhs value after the arithmetic operator");
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
                        Some(rhs_modifier) => rhs = rhs_modifier.evaluate(fields, row),
                        None => rhs = None,
                    }
                    match rhs {
                        Some(val) => match string_modifier {
                            StringModifier::Concatenate => return Some(lhs + &val),
                            StringModifier::ToUpperCase => match &self.additional_modifier {
                                Some(modifier) => match modifier {
                                    Modifier::StringModifier(string_modifier) => {
                                        match string_modifier {
                                            StringModifier::Concatenate => match &self.rhs {
                                                Some(rhs_modifier) => {
                                                    match rhs_modifier.evaluate(fields, row) {
                                                        Some(val) => {
                                                            return Some(lhs.to_uppercase() + &val);
                                                        }
                                                        None => return Some(lhs.to_uppercase()),
                                                    }
                                                }
                                                None => {
                                                    log_error!(
                                                        "missing a right hand side value for the string modifier"
                                                    );
                                                }
                                            },
                                            _ => {
                                                log_error!(
                                                    "to modifiers that accept the same number of parameters can not be next to each other"
                                                );
                                            }
                                        }
                                    }
                                    Modifier::ArithmeticModifier(_) => {
                                        log_error!(
                                            "an arithmetic modifier can not be applied to a non numeric value"
                                        );
                                        return None;
                                    }
                                },
                                None => return Some(lhs.to_uppercase()),
                            },
                            StringModifier::ToLowerCase => match &self.additional_modifier {
                                Some(modifier) => match modifier {
                                    Modifier::StringModifier(string_modifier) => {
                                        match string_modifier {
                                            StringModifier::Concatenate => match &self.rhs {
                                                Some(rhs_modifier) => {
                                                    match rhs_modifier.evaluate(fields, row) {
                                                        Some(val) => {
                                                            return Some(lhs.to_uppercase() + &val);
                                                        }
                                                        None => return Some(lhs.to_uppercase()),
                                                    }
                                                }
                                                None => {
                                                    log_error!(
                                                        "missing a right hand side value for the string modifier"
                                                    );
                                                }
                                            },
                                            _ => {
                                                log_error!(
                                                    "to modifiers that accept the same number of parameters can not be next to each other"
                                                );
                                            }
                                        }
                                    }
                                    Modifier::ArithmeticModifier(_) => {
                                        log_error!(
                                            "an arithmetic modifier can not be applied to a non numeric value"
                                        );
                                        return None;
                                    }
                                },
                                None => return Some(lhs.to_lowercase()),
                            },
                        },
                        None => match string_modifier {
                            StringModifier::ToUpperCase => return Some(lhs.to_uppercase()),
                            StringModifier::ToLowerCase => return Some(lhs.to_lowercase()),
                            _ => {
                                log_error!("messing the right hand side for the modifier");
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

// #[cfg(test)]
// mod modification_tests {
//     use super::*;

//     fn get_data() -> (Vec<String>, Vec<String>) {
//         let fields = vec!["name".to_string(), "age".to_string(), "city".to_string()];
//         let row = vec!["bob".to_string(), "45".to_string(), "London".to_string()];
//         (fields, row)
//     }

//     #[test]
//     fn arithmetic_plus() {
//         let (fields, row) = get_data();
//         let modification = Modification {
//             lhs: Value::FieldName("age".to_string()),
//             modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Plus)),
//             rhs: Some(Box::new(Modification {
//                 lhs: Value::Number(5.0),
//                 modifier: None,
//                 rhs: None,
//             })),
//         };
//         assert_eq!(modification.evaluate(&fields, &row), Some("50".to_string()));
//     }

//     #[test]
//     fn arithmetic_minus() {
//         let (fields, row) = get_data();
//         let modification = Modification {
//             lhs: Value::FieldName("age".to_string()),
//             modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Minus)),
//             rhs: Some(Box::new(Modification {
//                 lhs: Value::Number(5.0),
//                 modifier: None,
//                 rhs: None,
//             })),
//         };
//         assert_eq!(modification.evaluate(&fields, &row), Some("40".to_string()));
//     }

//     #[test]
//     fn arithmetic_multiply() {
//         let (fields, row) = get_data();
//         let modification = Modification {
//             lhs: Value::FieldName("age".to_string()),
//             modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Multiply)),
//             rhs: Some(Box::new(Modification {
//                 lhs: Value::Number(2.0),
//                 modifier: None,
//                 rhs: None,
//             })),
//         };
//         assert_eq!(modification.evaluate(&fields, &row), Some("90".to_string()));
//     }

//     #[test]
//     fn arithmetic_divide() {
//         let (fields, row) = get_data();
//         let modification = Modification {
//             lhs: Value::FieldName("age".to_string()),
//             modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Divide)),
//             rhs: Some(Box::new(Modification {
//                 lhs: Value::Number(5.0),
//                 modifier: None,
//                 rhs: None,
//             })),
//         };
//         assert_eq!(modification.evaluate(&fields, &row), Some("9".to_string()));
//     }

//     #[test]
//     fn arithmetic_modulo() {
//         let (fields, row) = get_data();
//         let modification = Modification {
//             lhs: Value::FieldName("age".to_string()),
//             modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Modulo)),
//             rhs: Some(Box::new(Modification {
//                 lhs: Value::Number(7.0),
//                 modifier: None,
//                 rhs: None,
//             })),
//         };
//         assert_eq!(modification.evaluate(&fields, &row), Some("3".to_string()));
//     }

//     #[test]
//     fn arithmetic_power() {
//         let (fields, row) = get_data();
//         let modification = Modification {
//             lhs: Value::FieldName("age".to_string()),
//             modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Power)),
//             rhs: Some(Box::new(Modification {
//                 lhs: Value::Number(2.0),
//                 modifier: None,
//                 rhs: None,
//             })),
//         };
//         assert_eq!(
//             modification.evaluate(&fields, &row),
//             Some("2025".to_string())
//         );
//     }

//     #[test]
//     fn string_concatenate() {
//         let (fields, row) = get_data();
//         let modification = Modification {
//             lhs: Value::FieldName("name".to_string()),
//             modifier: Some(Modifier::StringModifier(StringModifier::Concatenate)),
//             rhs: Some(Box::new(Modification {
//                 lhs: Value::Literal(" Smith".to_string()),
//                 modifier: None,
//                 rhs: None,
//             })),
//         };
//         assert_eq!(
//             modification.evaluate(&fields, &row),
//             Some("bob Smith".to_string())
//         );
//     }

//     #[test]
//     fn string_to_upper_case() {
//         let (fields, row) = get_data();
//         let modification = Modification {
//             lhs: Value::FieldName("city".to_string()),
//             modifier: Some(Modifier::StringModifier(StringModifier::ToUpperCase)),
//             rhs: None,
//         };
//         assert_eq!(
//             modification.evaluate(&fields, &row),
//             Some("LONDON".to_string())
//         );
//     }

//     #[test]
//     fn string_to_lower_case() {
//         let (fields, mut row) = get_data();
//         row[2] = "LONDON".to_string(); // Override city to be uppercase

//         let modification = Modification {
//             lhs: Value::FieldName("city".to_string()),
//             modifier: Some(Modifier::StringModifier(StringModifier::ToLowerCase)),
//             rhs: None,
//         };
//         assert_eq!(
//             modification.evaluate(&fields, &row),
//             Some("london".to_string())
//         );
//     }

//     #[test]
//     fn nested_arithmetic() {
//         let (fields, row) = get_data();
//         let modification = Modification {
//             lhs: Value::FieldName("age".to_string()),
//             modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Plus)),
//             rhs: Some(Box::new(Modification {
//                 lhs: Value::Number(5.0),
//                 modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Multiply)),
//                 rhs: Some(Box::new(Modification {
//                     lhs: Value::Number(2.0),
//                     modifier: None,
//                     rhs: None,
//                 })),
//             })),
//         };
//         // age + (5 * 2) = 45 + 10 = 55
//         assert_eq!(modification.evaluate(&fields, &row), Some("55".to_string()));
//     }

//     #[test]
//     fn invalid_operations() {
//         let (fields, row) = get_data();

//         // Arithmetic operation on string field
//         let modification = Modification {
//             lhs: Value::FieldName("name".to_string()),
//             modifier: Some(Modifier::ArithmeticModifier(ArithmeticModifier::Plus)),
//             rhs: Some(Box::new(Modification {
//                 lhs: Value::Number(5.0),
//                 modifier: None,
//                 rhs: None,
//             })),
//         };
//         assert_eq!(modification.evaluate(&fields, &row), None);

//         // Missing rhs for concatenation
//         let modification = Modification {
//             lhs: Value::FieldName("name".to_string()),
//             modifier: Some(Modifier::StringModifier(StringModifier::Concatenate)),
//             rhs: None,
//         };
//         assert_eq!(modification.evaluate(&fields, &row), None);
//     }
// }
