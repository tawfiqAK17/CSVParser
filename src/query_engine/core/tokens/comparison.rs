use super::ParseResult;
use super::value;
use super::value::Value;
use crate::log_error;
use std::io::SeekFrom;
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug)]
pub enum ComparisonOps {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    BetweenOp(Value, Value),
    Is,
    IsNot,
    Contains,
    In,
    StartsWith,
    EndsWith,
}
impl Display for ComparisonOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonOps::Equal => write!(f, "==")?,
            ComparisonOps::NotEqual => write!(f, "!=")?,
            ComparisonOps::LessThan => write!(f, "<")?,
            ComparisonOps::GreaterThan => write!(f, ">")?,
            ComparisonOps::LessThanOrEqual => write!(f, "<=")?,
            ComparisonOps::GreaterThanOrEqual => write!(f, ">=")?,
            ComparisonOps::BetweenOp(_, _) => write!(f, "between")?,
            ComparisonOps::Is => write!(f, "is")?,
            ComparisonOps::IsNot => write!(f, "isnot")?,
            ComparisonOps::Contains => write!(f, "contains")?,
            ComparisonOps::In => write!(f, "in")?,
            ComparisonOps::StartsWith => write!(f, "starts-with")?,
            ComparisonOps::EndsWith => write!(f, "ends-with")?,
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct Comparison {
    field_name: String,
    comparison_op: ComparisonOps,
    rhs: Value,
}

impl Comparison {
    pub fn parse(lexemes: &[String], mut idx: usize) -> (ParseResult<Self>, usize) {
        if let Some(lexeme) = lexemes.get(idx) {
            // the first value must be a field name
            if let Some(field_name) = value::parse_field_name(lexeme) {
                idx += 1;

                if let Some(lexeme) = lexemes.get(idx) {
                    let comparison_op: ComparisonOps;
                    let rhs: Value;

                    // the second lexeme must be a comparison operator
                    match lexeme.as_str() {
                        "==" => comparison_op = ComparisonOps::Equal,
                        "!=" => comparison_op = ComparisonOps::NotEqual,
                        ">=" => comparison_op = ComparisonOps::GreaterThanOrEqual,
                        "<=" => comparison_op = ComparisonOps::LessThanOrEqual,
                        ">" => comparison_op = ComparisonOps::GreaterThan,
                        "<" => comparison_op = ComparisonOps::LessThan,
                        "between" => {
                            let val1: Value;
                            let val2: Value;
                            // the first parameter for the between operator
                            if let Some(val) = lexemes.get(idx + 1) {
                                if let Some(field_name) = value::parse_field_name(val) {
                                    val1 = Value::FieldName(field_name);
                                } else if let Some(number) = value::parse_number(val) {
                                    val1 = Value::Number(number);
                                } else {
                                    log_error!(
                                        "the between operator can only accept a field name or a number as its parameters"
                                    );
                                    return (ParseResult::Err, idx - 1);
                                }
                            } else {
                                log_error!(
                                    "the between operator expect two parameters: between param1 and param2"
                                );
                                return (ParseResult::Err, idx - 1);
                            }
                            // the and key word
                            if let Some(val) = lexemes.get(idx + 2) {
                                if val == "and" {
                                    // the second parameter for the between operator
                                    if let Some(val) = lexemes.get(idx + 3) {
                                        if let Some(field_name) = value::parse_field_name(val) {
                                            val2 = Value::FieldName(field_name);
                                        } else if let Some(number) = value::parse_number(val) {
                                            val2 = Value::Number(number);
                                        } else {
                                            log_error!(
                                                "the between operator can only accept a field name or a number as its parameters",
                                            );
                                            return (ParseResult::Err, idx - 1);
                                        }
                                        return (
                                            ParseResult::Val(Comparison {
                                                field_name,
                                                comparison_op: ComparisonOps::BetweenOp(val1, val2),
                                                rhs: Value::None,
                                            }),
                                            idx + 3,
                                        );
                                    } else {
                                        log_error!(
                                            "the between operator expect two parameters: between param1 and param2"
                                        );
                                        return (ParseResult::Err, idx - 1);
                                    }
                                } else {
                                    // the val is not the key word 'and'
                                    log_error!(
                                        "the between operator expect two parameters: between param1 and param2"
                                    );
                                    return (ParseResult::Err, idx - 1);
                                }
                            } else {
                                log_error!(
                                    "the between operator expect two parameters: between param1 and param2"
                                );
                                return (ParseResult::Err, idx - 1);
                            }
                        }
                        "is" => comparison_op = ComparisonOps::Is,
                        "isnot" => comparison_op = ComparisonOps::IsNot,
                        "contains" => comparison_op = ComparisonOps::Contains,
                        "starts-with" => comparison_op = ComparisonOps::StartsWith,
                        "ends-with" => comparison_op = ComparisonOps::EndsWith,
                        "in" => comparison_op = ComparisonOps::In,
                        _ => {
                            log_error!("expecting a comparison operator after the field name");
                            return (ParseResult::Err, idx);
                        }
                    }
                    idx += 1;
                    if let Some(lexeme) = lexemes.get(idx) {
                        // the third lexeme is the rhs which can be a literal a field name or a
                        // number
                        if let Some(literal) = value::parse_literal(lexeme) {
                            if literal.is_empty() {
                                rhs = Value::None;
                            } else {
                                rhs = Value::Literal(literal);
                            }
                        } else if let Some(field_name) = value::parse_field_name(lexeme) {
                            rhs = Value::FieldName(field_name);
                        } else if let Some(number) = value::parse_number(lexeme) {
                            rhs = Value::Number(number);
                        } else if let Some((list, last_idx)) = value::parse_list(lexemes, idx) {
                            rhs = Value::List(list);
                            idx = last_idx;
                        } else {
                            log_error!(
                                "{} can not be considered as a valid value to compare to",
                                lexeme
                            );
                            return (ParseResult::Err, idx);
                        }
                        return (
                            ParseResult::Val(Comparison {
                                field_name,
                                comparison_op,
                                rhs,
                            }),
                            idx,
                        );
                    } else {
                    }
                    log_error!(
                        "expecting a value after the comparison operator {}",
                        comparison_op
                    );
                    return (ParseResult::Err, idx);
                } else {
                    log_error!("expecting a comparison operator after the field name");
                    return (ParseResult::Err, idx);
                }
            } else {
                log_error!("expecting a field name for the comparison");
                return (ParseResult::Err, idx);
            }
        } else {
            return (ParseResult::None, idx);
        }
    }
    pub fn evaluate(&self, fields: &Vec<String>, row: &Vec<String>) -> bool {
        match &self.comparison_op {
            ComparisonOps::Equal => return self.equal(&self.rhs, fields, row),
            ComparisonOps::NotEqual => return !self.equal(&self.rhs, fields, row),
            ComparisonOps::LessThan => return self.less_than(&self.rhs, fields, row),
            ComparisonOps::GreaterThan => return self.greater_than(&self.rhs, fields, row),
            ComparisonOps::LessThanOrEqual => {
                return self.less_than(&self.rhs, fields, row)
                    || self.equal(&self.rhs, fields, row);
            }
            ComparisonOps::GreaterThanOrEqual => {
                return self.greater_than(&self.rhs, fields, row)
                    || self.equal(&self.rhs, fields, row);
            }
            ComparisonOps::BetweenOp(val1, val2) => {
                return (self.greater_than(val1, fields, row) || self.equal(val1, fields, row))
                    && (self.less_than(val2, fields, row) || self.equal(val2, fields, row));
            }
            ComparisonOps::Is => return self.compair_strings(fields, row, |a, b| return a == b),
            ComparisonOps::IsNot => return self.compair_strings(fields, row, |a, b| return a != b),
            ComparisonOps::Contains => {
                return self.compair_strings(fields, row, |a, b| return a.contains(b));
            }
            ComparisonOps::StartsWith => {
                return self.compair_strings(fields, row, |a, b| return a.starts_with(b));
            }
            ComparisonOps::EndsWith => {
                return self.compair_strings(fields, row, |a, b| return a.ends_with(b));
            }
            ComparisonOps::In => match &self.rhs {
                Value::List(list) => match fields.iter().position(|f| *f == self.field_name) {
                    Some(idx) => return list.contains(&row[idx]),
                    None => {
                        log_error!("no field named {}", self.field_name);
                        return false;
                    }
                },
                _ => {
                    log_error!("the value after the in operator should be a list");
                    return false;
                }
            },
        }
    }
    fn equal(&self, value: &Value, fields: &Vec<String>, row: &Vec<String>) -> bool {
        match value {
            Value::FieldName(field) => match self.n_compaire_to_field(field, fields, row) {
                Some(order) => match order {
                    Ordering::Equal => return true,
                    _ => return false,
                },
                None => return false,
            },
            Value::Number(val) => match self.compaire_to_number(val.clone(), fields, row) {
                Some(order) => match order {
                    Ordering::Equal => return true,
                    _ => return false,
                },
                None => return false,
            },
            Value::None => self.is_none(fields, row),
            _ => todo!(),
        }
    }
    fn less_than(&self, value: &Value, fields: &Vec<String>, row: &Vec<String>) -> bool {
        match value {
            Value::FieldName(field) => match self.n_compaire_to_field(field, fields, row) {
                Some(order) => match order {
                    Ordering::Less => return true,
                    _ => return false,
                },
                None => return false,
            },
            Value::Number(val) => match self.compaire_to_number(val.clone(), fields, row) {
                Some(order) => match order {
                    Ordering::Less => return true,
                    _ => return false,
                },
                None => return false,
            },
            Value::None => return false,
            _ => unreachable!(
                "the value passed to 'less_than' function can only be a number or a field name"
            ),
        }
    }
    fn greater_than(&self, value: &Value, fields: &Vec<String>, row: &Vec<String>) -> bool {
        match value {
            Value::FieldName(field) => match self.n_compaire_to_field(field, fields, row) {
                Some(order) => match order {
                    Ordering::Greater => return true,
                    _ => return false,
                },
                None => return false,
            },
            Value::Number(val) => match self.compaire_to_number(val.clone(), fields, row) {
                Some(order) => match order {
                    Ordering::Greater => return true,
                    _ => return false,
                },
                None => return false,
            },
            Value::None => return false,
            _ => unreachable!(
                "the value passed to 'less_than' function can only be a number or a field name"
            ),
        }
    }

    fn is_none(&self, fields: &Vec<String>, row: &Vec<String>) -> bool {
        match fields.iter().position(|f| *f == self.field_name) {
            Some(idx) => return row[idx] == "",
            None => {
                log_error!("no field named {}", self.field_name);
                return false;
            }
        }
    }
    // numerical comparison between two fields
    fn n_compaire_to_field(
        &self,
        field: &String,
        fields: &Vec<String>,
        row: &Vec<String>,
    ) -> Option<Ordering> {
        let lhs_idx: usize;
        let rhs_idx: usize;
        match fields.iter().position(|f| *f == self.field_name) {
            Some(idx) => lhs_idx = idx,
            None => {
                log_error!("no field named {}", self.field_name);
                return None;
            }
        }
        match fields.iter().position(|f| f == field) {
            Some(idx) => rhs_idx = idx,
            None => {
                log_error!("no field named {}", self.field_name);
                return None;
            }
        }
        let lhs: f32;
        let rhs: f32;
        match row[lhs_idx].parse::<f32>() {
            Ok(val) => lhs = val,
            Err(_) => {
                return None;
            }
        }
        match row[rhs_idx].parse::<f32>() {
            Ok(val) => rhs = val,
            Err(_) => {
                return None;
            }
        }
        if lhs - rhs == 0f32 {
            return Some(Ordering::Equal);
        }
        if lhs - rhs > 0f32 {
            return Some(Ordering::Greater);
        }
        return Some(Ordering::Less);
    }
    // numerical comparison between the self.field and a number
    fn compaire_to_number(
        &self,
        number: f32,
        fields: &Vec<String>,
        row: &Vec<String>,
    ) -> Option<Ordering> {
        let field_idx: usize;
        match fields.iter().position(|f| *f == self.field_name) {
            Some(idx) => field_idx = idx,
            None => {
                log_error!("no field named {}", self.field_name);
                return None;
            }
        }
        let field_val: f32;
        match row[field_idx].parse::<f32>() {
            Ok(val) => field_val = val,
            Err(_) => {
                log_error!(
                    "{} is not a numerical value it has been evaluated as infinity",
                    row[field_idx]
                );
                return None;
            }
        }
        if field_val - number == 0f32 {
            return Some(Ordering::Equal);
        }
        if field_val - number > 0f32 {
            return Some(Ordering::Greater);
        }
        return Some(Ordering::Less);
    }
    // this function will return the result of a string comparison between the self.field_name value and the
    // self.rhs value
    fn compair_strings<F>(&self, fields: &Vec<String>, row: &Vec<String>, comparison: F) -> bool
    where
        F: Fn(&String, &String) -> bool,
    {
        match &self.rhs {
            Value::Literal(val) => match fields.iter().position(|f| *f == self.field_name) {
                Some(idx) => return comparison(&row[idx], &val.to_string()),
                None => {
                    log_error!("no field named {}", self.field_name);
                    return false;
                }
            },
            Value::FieldName(field_name) => {
                match fields.iter().position(|f| *f == self.field_name) {
                    Some(idx1) => match fields.iter().position(|f| *f == *field_name) {
                        Some(idx2) => {
                            return comparison(&row[idx1], &row[idx2]);
                        }
                        None => {
                            log_error!("no field named {}", field_name);
                            return false;
                        }
                    },
                    None => {
                        log_error!("no field named {}", self.field_name);
                        return false;
                    }
                }
            }
            Value::None => return self.is_none(fields, row),

            _ => {
                log_error!(
                    "the value {} can not be compared to the value at field '{}'",
                    self.rhs,
                    self.field_name
                );
                return false;
            }
        }
    }
}

#[cfg(test)]
mod numbers_comparison_tests {
    use super::*;

    fn get_test_data() -> (Vec<String>, Vec<String>) {
        let fields = vec!["name".to_string(), "age".to_string(), "points".to_string()];
        let row = vec!["bob".to_string(), "45".to_string(), "60".to_string()];
        (fields, row)
    }

    // Field-to-value comparisons
    #[test]
    fn test_equal_operator_field_to_value() {
        let (fields, row) = get_test_data();

        let comparison = Comparison {
            field_name: "age".to_string(),
            comparison_op: ComparisonOps::Equal,
            rhs: Value::Number(45.0),
        };
        assert!(comparison.evaluate(&fields, &row));
    }

    // Field-to-field comparisons
    #[test]
    fn test_equal_operator_field_to_field() {
        let (fields, row) = get_test_data();

        let comparison = Comparison {
            field_name: "age".to_string(),
            comparison_op: ComparisonOps::Equal,
            rhs: Value::FieldName("age".to_string()),
        };
        assert!(comparison.evaluate(&fields, &row));

        let comparison = Comparison {
            field_name: "age".to_string(),
            comparison_op: ComparisonOps::Equal,
            rhs: Value::FieldName("points".to_string()),
        };
        assert!(!comparison.evaluate(&fields, &row));
    }

    #[test]
    fn test_less_than_operator_field_to_field() {
        let (fields, row) = get_test_data();

        // age (45) < points (60)
        let comparison = Comparison {
            field_name: "age".to_string(),
            comparison_op: ComparisonOps::LessThan,
            rhs: Value::FieldName("points".to_string()),
        };
        assert!(comparison.evaluate(&fields, &row));

        // points (60) < age (45) should be false
        let comparison = Comparison {
            field_name: "points".to_string(),
            comparison_op: ComparisonOps::LessThan,
            rhs: Value::FieldName("age".to_string()),
        };
        assert!(!comparison.evaluate(&fields, &row));
    }

    #[test]
    fn test_greater_than_operator_field_to_field() {
        let (fields, row) = get_test_data();

        // points (60) > age (45)
        let comparison = Comparison {
            field_name: "points".to_string(),
            comparison_op: ComparisonOps::GreaterThan,
            rhs: Value::FieldName("age".to_string()),
        };
        assert!(comparison.evaluate(&fields, &row));

        // age (45) > points (60) should be false
        let comparison = Comparison {
            field_name: "age".to_string(),
            comparison_op: ComparisonOps::GreaterThan,
            rhs: Value::FieldName("points".to_string()),
        };
        assert!(!comparison.evaluate(&fields, &row));
    }

    #[test]
    fn test_between_operator_with_fields() {
        let (fields, row) = get_test_data();

        // Test age (45) between points (60) and literal 40
        let comparison = Comparison {
            field_name: "age".to_string(),
            comparison_op: ComparisonOps::BetweenOp(
                Value::FieldName("points".to_string()),
                Value::Number(40.0),
            ),
            rhs: Value::None,
        };
        assert!(!comparison.evaluate(&fields, &row));

        // Test points (60) between age (45) and literal 70
        let comparison = Comparison {
            field_name: "points".to_string(),
            comparison_op: ComparisonOps::BetweenOp(
                Value::FieldName("age".to_string()),
                Value::Number(70.0),
            ),
            rhs: Value::None,
        };
        assert!(comparison.evaluate(&fields, &row));
    }

    #[test]
    fn test_mixed_comparisons() {
        let (fields, row) = get_test_data();

        // Test age (45) == points (60) - 15
        let comparison = Comparison {
            field_name: "age".to_string(),
            comparison_op: ComparisonOps::Equal,
            rhs: Value::Literal("($points - 15)".to_string()),
        };
        // This would require your evaluation to handle expressions
        // assert!(comparison.evaluate(&fields, &row));
    }

    #[test]
    fn test_invalid_field_comparison() {
        let (fields, row) = get_test_data();

        // Compare with non-existent field
        let comparison = Comparison {
            field_name: "age".to_string(),
            comparison_op: ComparisonOps::Equal,
            rhs: Value::FieldName("nonexistent".to_string()),
        };
        assert!(!comparison.evaluate(&fields, &row));
    }
}
#[cfg(test)]
mod string_comparison_tests {
    use super::*;

    fn get_test_data() -> (Vec<String>, Vec<String>) {
        let fields = vec![
            "name".to_string(),
            "department".to_string(),
            "email".to_string(),
        ];
        let row = vec![
            "John Doe".to_string(),
            "Engineering".to_string(),
            "john.doe@example.com".to_string(),
        ];
        (fields, row)
    }

    #[test]
    fn test_is_operator() {
        let (fields, row) = get_test_data();

        // Field to value comparison
        let comparison = Comparison {
            field_name: "name".to_string(),
            comparison_op: ComparisonOps::Is,
            rhs: Value::Literal("John Doe".to_string()),
        };
        assert!(comparison.evaluate(&fields, &row));

        // Field to field comparison (same value)
        let comparison = Comparison {
            field_name: "name".to_string(),
            comparison_op: ComparisonOps::Is,
            rhs: Value::FieldName("name".to_string()),
        };
        assert!(comparison.evaluate(&fields, &row));
    }

    #[test]
    fn test_is_not_operator() {
        let (fields, row) = get_test_data();

        let comparison = Comparison {
            field_name: "name".to_string(),
            comparison_op: ComparisonOps::IsNot,
            rhs: Value::Literal("Jane Doe".to_string()),
        };
        assert!(comparison.evaluate(&fields, &row));

        let comparison = Comparison {
            field_name: "name".to_string(),
            comparison_op: ComparisonOps::IsNot,
            rhs: Value::FieldName("department".to_string()),
        };
        assert!(comparison.evaluate(&fields, &row));
    }

    #[test]
    fn test_contains_operator() {
        let (fields, row) = get_test_data();

        // Field contains value
        let comparison = Comparison {
            field_name: "email".to_string(),
            comparison_op: ComparisonOps::Contains,
            rhs: Value::Literal("example".to_string()),
        };
        assert!(comparison.evaluate(&fields, &row));

        // Field contains another field's value
        let comparison = Comparison {
            field_name: "email".to_string(),
            comparison_op: ComparisonOps::Contains,
            rhs: Value::FieldName("name".to_string()),
        };
        assert!(!comparison.evaluate(&fields, &row)); // "john.doe@..." does not contains "John Doe"
    }

    #[test]
    fn test_in_operator() {
        let (fields, row) = get_test_data();

        // Value in list of strings
        let comparison = Comparison {
            field_name: "department".to_string(),
            comparison_op: ComparisonOps::In,
            rhs: Value::List(vec![
                "HR".to_string(),
                "Engineering".to_string(),
                "Finance".to_string(),
            ]),
        };
        assert!(comparison.evaluate(&fields, &row));

        let comparison = Comparison {
            field_name: "name".to_string(),
            comparison_op: ComparisonOps::In,
            rhs: Value::List(vec![
                "Engineering".to_string(),          // department value
                "john.doe@example.com".to_string(), // email value
            ]),
        };
        assert!(!comparison.evaluate(&fields, &row)); // "John Doe" not in the list
    }
    #[test]
    fn test_starts_with_operator() {
        let (fields, row) = get_test_data();

        let comparison = Comparison {
            field_name: "email".to_string(),
            comparison_op: ComparisonOps::StartsWith,
            rhs: Value::Literal("john".to_string()),
        };
        assert!(comparison.evaluate(&fields, &row));

        let comparison = Comparison {
            field_name: "email".to_string(),
            comparison_op: ComparisonOps::StartsWith,
            rhs: Value::FieldName("name".to_string()),
        };
        assert!(!comparison.evaluate(&fields, &row)); // email doesn't start with full name
    }

    #[test]
    fn test_ends_with_operator() {
        let (fields, row) = get_test_data();

        let comparison = Comparison {
            field_name: "email".to_string(),
            comparison_op: ComparisonOps::EndsWith,
            rhs: Value::Literal("example.com".to_string()),
        };
        assert!(comparison.evaluate(&fields, &row));

        let comparison = Comparison {
            field_name: "name".to_string(),
            comparison_op: ComparisonOps::EndsWith,
            rhs: Value::FieldName("department".to_string()),
        };
        assert!(!comparison.evaluate(&fields, &row));
    }

    #[test]
    fn test_case_sensitivity() {
        let (fields, row) = get_test_data();

        // Case-sensitive comparison
        let comparison = Comparison {
            field_name: "name".to_string(),
            comparison_op: ComparisonOps::Is,
            rhs: Value::Literal("john doe".to_string()),
        };
        assert!(!comparison.evaluate(&fields, &row));

        // Case-insensitive contains
        let comparison = Comparison {
            field_name: "email".to_string(),
            comparison_op: ComparisonOps::Contains,
            rhs: Value::Literal("EXAMPLE".to_string()),
        };
        assert!(!comparison.evaluate(&fields, &row));
    }
}
