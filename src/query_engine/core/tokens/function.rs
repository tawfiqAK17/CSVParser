use super::ParseResult;
use crate::log_error;

use super::value;
use core::f32;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum Functions {
    Sort(String),
    ReverseSort(String),
    NSort(String),
    ReverseNSort(String),
    Head(usize),
    Tail(usize),
}
impl Functions {
    pub fn get_available_functions_names<'a>() -> Vec<&'a str> {
      return vec!["sort", "rsort", "nsort", "nrsort", "head", "tail"];
    }
}
#[derive(Debug)]
pub struct Function {
    function_name: Functions,
}

impl Function {
    pub fn parse(lexemes: &[String], mut idx: usize) -> (ParseResult<Self>, usize) {
        match lexemes.get(idx) {
            Some(lexeme) => {
              if !Functions::get_available_functions_names().contains(&lexeme.as_str()) {
                log_error!("no function named {}", lexeme);
                return (ParseResult::Err, idx);
              }
                idx += 1;
                match lexemes.get(idx) {
                    Some(param) => match value::parse_field_name(param) {
                        Some(field_name) => match lexeme.as_str() {
                          // if the parameter is a field name than the function can be one of the
                          // next match cases
                            "sort" => {
                                return (
                                    ParseResult::Val(Function {
                                        function_name: Functions::Sort(field_name),
                                    }),
                                    idx + 1,
                                );
                            }
                            "rsort" => {
                                return (
                                    ParseResult::Val(Function {
                                        function_name: Functions::ReverseSort(field_name),
                                    }),
                                    idx + 1,
                                );
                            }
                            "nsort" => {
                                return (
                                    ParseResult::Val(Function {
                                        function_name: Functions::NSort(field_name),
                                    }),
                                    idx + 1,
                                );
                            }
                            "nrsort" => {
                                return (
                                    ParseResult::Val(Function {
                                        function_name: Functions::ReverseNSort(field_name),
                                    }),
                                    idx + 1,
                                );
                            }
                            "head" | "tail" => {
                              // these functions does not accept a field name as a parameter
                                log_error!(
                                    "the function {} expect a parameter of type number",
                                    lexeme
                                );
                                return (ParseResult::Err, idx);
                            }
                            _ => {
                                log_error!("no function named {}", lexeme);
                                return (ParseResult::Err, idx);
                            }
                        },
                        None => match value::parse_number(param) {
                            Some(val) => match lexeme.as_str() {
                                "head" => {
                                    return (
                                        ParseResult::Val(Function {
                                            function_name: Functions::Head(val.round() as usize),
                                        }),
                                        idx + 1,
                                    );
                                }
                                "tail" => {
                                    return (
                                        ParseResult::Val(Function {
                                            function_name: Functions::Tail(val.round() as usize),
                                        }),
                                        idx + 1,
                                    );
                                }
                                "sort" | "rsort" | "nsort" | "reverse-nsort" => {
                                    log_error!(
                                        "the function {} expect a parameter of type field name",
                                        lexeme
                                    );
                                    return (ParseResult::Err, idx);
                                }
                                _ => {
                                    log_error!("no function named {}", lexeme);
                                    return (ParseResult::Err, idx);
                                }
                            },
                            None => {
                              // the parameter is neither a field name or a number
                                log_error!(
                                    "the parameters of function can only be a field name or a number"
                                );
                                return (ParseResult::Err, idx);
                            }
                        },
                    },
                    None => {
                        // the function exist but no parameter was given
                        log_error!("expecting a parameter for the function {}", lexeme);
                        return (ParseResult::Err, idx);
                    }
                }
            }
            None => return (ParseResult::None, idx),
        }
    }
    pub fn run(&self, fields: &Vec<String>, rows: &mut Vec<&Vec<String>>) {
        match &self.function_name {
            Functions::Sort(field_name) => {
                match fields.iter().position(|name| name == field_name) {
                    Some(idx) => {
                        self.sort(idx, rows);
                    }
                    None => {
                        log_error!("no field named {}", field_name);
                    }
                }
            }
            Functions::ReverseSort(field_name) => {
                match fields.iter().position(|name| name == field_name) {
                    Some(idx) => {
                        self.reverse_sort(idx, rows);
                    }
                    None => {
                        log_error!("no field named {}", field_name);
                    }
                }
            }
            Functions::NSort(field_name) => {
                match fields.iter().position(|name| name == field_name) {
                    Some(idx) => {
                        self.n_sort(idx, rows);
                    }
                    None => {
                        log_error!("no field named {}", field_name);
                    }
                }
            }

            Functions::ReverseNSort(field_name) => {
                match fields.iter().position(|name| name == field_name) {
                    Some(idx) => {
                        self.n_reverse_sort(idx, rows);
                    }
                    None => {
                        log_error!("no field named {}", field_name);
                    }
                }
            }

            Functions::Head(arg) => {
                rows.drain(arg..);
            }
            Functions::Tail(arg) => {
                rows.drain(0..rows.len() - arg);
            }
        }
    }

    fn sort(&self, field_idx: usize, rows: &mut Vec<&Vec<String>>) {
        rows.sort_by(|a, b| a[field_idx].cmp(&b[field_idx]));
    }

    fn reverse_sort(&self, field_idx: usize, rows: &mut Vec<&Vec<String>>) {
        rows.sort_by(|a, b| b[field_idx].cmp(&a[field_idx]));
    }

    fn n_sort(&self, field_idx: usize, rows: &mut Vec<&Vec<String>>) {
        rows.sort_by(|a, b| self.compaire_numbers(&a[field_idx], &b[field_idx]));
    }

    fn n_reverse_sort(&self, field_idx: usize, rows: &mut Vec<&Vec<String>>) {
        rows.sort_by(
            |a, b| match self.compaire_numbers(&a[field_idx], &b[field_idx]) {
                Ordering::Less => return Ordering::Greater,
                Ordering::Equal => return Ordering::Equal,
                Ordering::Greater => return Ordering::Less,
            },
        );
    }

    fn compaire_numbers(&self, a: &String, b: &String) -> Ordering {
        let lhs: f32;
        let rhs: f32;
        match a.parse::<f32>() {
            Ok(val) => lhs = val,
            Err(_) => {
                return Ordering::Greater;
            }
        }
        match b.parse::<f32>() {
            Ok(val) => rhs = val,
            Err(_) => {
                return Ordering::Less;
            }
        }
        if lhs - rhs == 0f32 {
            return Ordering::Equal;
        }
        if lhs - rhs > 0f32 {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
}

#[cfg(test)]
mod tests {
    fn get_data() -> (Vec<String>, Vec<Vec<String>>) {
        let fields = vec!["name".to_string(), "age".to_string(), "points".to_string()];
        let rows = vec![
            vec!["bob".to_string(), "45".to_string(), "60".to_string()],
            vec!["ossama".to_string(), "27".to_string(), "100".to_string()],
            vec!["jack".to_string(), "20".to_string(), "90".to_string()],
        ];
        (fields, rows)
    }
    fn prepair_rows(rows: &Vec<Vec<String>>) -> Vec<&Vec<String>> {
        let mut rows_ref: Vec<&Vec<String>> = Vec::new();
        for row in rows.iter() {
            rows_ref.push(row); 
        }
        return rows_ref.clone();
    }
    fn get_empty_data() -> (Vec<String>, Vec<Vec<String>>) {
        let fields: Vec<String> = vec![];
        let rows: Vec<Vec<String>> = vec![];
        (fields, rows)
    }
    #[test]
    fn sort_test() {
        use super::*;
        let (fields, rows) = get_data();
        let mut rows_ref = prepair_rows(&rows);
        let function = Function {
            function_name: Functions::Sort("name".to_string()),
        };
        function.run(&fields, &mut rows_ref);
        let expected_rows = vec![
            vec!["bob", "45", "60"],
            vec!["jack", "20", "90"],
            vec!["ossama", "27", "100"],
        ];

        assert!(equal_rows(&expected_rows, &rows_ref))
    }
    #[test]
    fn empty_sort_test() {
        use super::*;
        let (fields, rows) = get_empty_data();
        let mut rows_ref = prepair_rows(&rows);
        let function = Function {
            function_name: Functions::Sort("name".to_string()),
        };
        function.run(&fields, &mut rows_ref);
        let expected_rows = vec![];
        assert!(equal_rows(&expected_rows, &rows_ref))
    }

    #[test]
    fn reverse_sort_test() {
        use super::*;
        let (fields, rows) = get_data();
        let mut rows_ref = prepair_rows(&rows);
        let function = Function {
            function_name: Functions::ReverseSort("name".to_string()),
        };
        function.run(&fields, &mut rows_ref);
        let expected_rows = vec![
            vec!["ossama", "27", "100"],
            vec!["jack", "20", "90"],
            vec!["bob", "45", "60"],
        ];

        assert!(equal_rows(&expected_rows, &rows_ref))
    }
    #[test]
    fn n_reverse_sort_test() {
        use super::*;
        let (fields, rows) = get_data();
        let mut rows_ref = prepair_rows(&rows);
        let function = Function {
            function_name: Functions::ReverseNSort("points".to_string()),
        };
        function.run(&fields, &mut rows_ref);
        let expected_rows = vec![
            vec!["ossama", "27", "100"],
            vec!["jack", "20", "90"],
            vec!["bob", "45", "60"],
        ];

        assert!(equal_rows(&expected_rows, &rows_ref))
    }
    #[test]
    fn n_sort_test() {
        use super::*;
        let (fields, rows) = get_data();
        let mut rows_ref = prepair_rows(&rows);
        let function = Function {
            function_name: Functions::NSort("points".to_string()),
        };
        function.run(&fields, &mut rows_ref);
        let expected_rows = vec![
            vec!["bob", "45", "60"],
            vec!["jack", "20", "90"],
            vec!["ossama", "27", "100"],
        ];

        assert!(equal_rows(&expected_rows, &rows_ref))
    }
    #[test]
    fn empty_n_sort_test() {
        use super::*;
        let (fields, rows) = get_empty_data();
        let mut rows_ref = prepair_rows(&rows);
        let function = Function {
            function_name: Functions::Sort("age".to_string()),
        };
        function.run(&fields, &mut rows_ref);
        let expected_rows = vec![];
        assert!(equal_rows(&expected_rows, &rows_ref))
    }

    fn equal_rows(expected_rows: &Vec<Vec<&str>>, rows: &Vec<&Vec<String>>) -> bool {
        if expected_rows.len() != rows.len() {
            return false;
        }
        for (expected_row, actual_row) in expected_rows.iter().zip(rows.iter()) {
            if expected_row.len() != actual_row.len() {
                return false;
            }
            for (expected_cell, actual_cell) in expected_row.iter().zip(actual_row.iter()) {
                if *expected_cell != *actual_cell {
                    return false;
                }
            }
        }
        true
    }
}
