use core::f32;
use std::{cmp::Ordering, process::exit};

pub enum Functions {
    Sort(String),
    ReverseSort(String),
    NSort(String),
    NReverseSort(String),
    Head(usize),
    Tail(usize),
}

pub struct Function {
    function_name: Functions,
}

impl Function {
    pub fn run(&self, fields: &Vec<&String>, mut rows: &mut Vec<Vec<&String>>) {
        match &self.function_name {
            Functions::Sort(field_name) => {
                match fields.iter().position(|&name| *name == *field_name) {
                    Some(idx) => {
                        self.sort(idx, rows);
                    }
                    None => {
                        eprintln!("no field named {}", field_name);
                    }
                }
            }
            Functions::ReverseSort(field_name) => {
                match fields.iter().position(|&name| *name == *field_name) {
                    Some(idx) => {
                        self.reverse_sort(idx, rows);
                    }
                    None => {
                        eprintln!("no field named {}", field_name);
                        exit(1);
                    }
                }
            }
            Functions::NSort(field_name) => {
                match fields.iter().position(|&name| *name == *field_name) {
                    Some(idx) => {
                        self.n_sort(idx, rows);
                    }
                    None => {
                        eprintln!("no field named {}", field_name);
                        exit(1);
                    }
                }
            }

            Functions::NReverseSort(field_name) => {
                match fields.iter().position(|&name| *name == *field_name) {
                    Some(idx) => {
                        self.n_reverse_sort(idx, rows);
                    }
                    None => {
                        eprintln!("no field named {}", field_name);
                        exit(1);
                    }
                }
            }

            Functions::Head(arg) => {
                self.print_rows(&rows[..arg.clone()]);
            }
            Functions::Tail(arg) => {
                self.print_rows(&rows[arg.clone()..]);
            }
        }
    }

    fn sort(&self, field_idx: usize, rows: &mut Vec<Vec<&String>>) {
        rows.sort_by(|a, b| a[field_idx].cmp(b[field_idx]));
    }

    fn reverse_sort(&self, field_idx: usize, rows: &mut Vec<Vec<&String>>) {
        rows.sort_by(|a, b| b[field_idx].cmp(a[field_idx]));
    }

    fn n_sort(&self, field_idx: usize, rows: &mut Vec<Vec<&String>>) {
        rows.sort_by(|a, b| self.compaire_numbers(&a[field_idx], &b[field_idx]));
    }

    fn n_reverse_sort(&self, field_idx: usize, rows: &mut Vec<Vec<&String>>) {
        rows.sort_by(
            |a, b| match self.compaire_numbers(&a[field_idx], &b[field_idx]) {
                Ordering::Less => return Ordering::Greater,
                Ordering::Equal => return Ordering::Equal,
                Ordering::Greater => return Ordering::Less,
            },
        );
    }

    fn compaire_numbers(&self, a: &String, b: &String) -> Ordering {
        let mut lhs: f32;
        let mut rhs: f32;
        match a.parse::<f32>() {
            Ok(val) => lhs = val,
            Err(_) => {
                eprintln!("\"{a}\" is not a numerical value it has been evaluated as infinity");
                return Ordering::Greater;
            }
        }
        match b.parse::<f32>() {
            Ok(val) => rhs = val,
            Err(_) => {
                eprintln!("\"{b}\" is not a numerical value it has been evaluated as infinity");
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
    fn print_row(&self, row: &Vec<&String>) {
        for val in row {
            print!("{val},");
        }
        println!();
    }

    fn print_rows(&self, rows: &[Vec<&String>]) {
        for row in rows {
            self.print_row(&row);
        }
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
    fn get_empty_data() -> (Vec<String>, Vec<Vec<String>>) {
        let fields: Vec<String> = vec![];
        let rows: Vec<Vec<String>> = vec![];
        (fields, rows)
    }
    #[test]
    fn sort_test() {
        use super::*;
        let (fields, rows) = get_data();
        let fields_ref: Vec<&String> = fields.iter().collect();
        let mut rows_ref: Vec<Vec<&String>> = rows.iter().map(|row| row.iter().collect()).collect();
        let function = Function {
            function_name: Functions::Sort("name".to_string()),
        };
        function.run(&fields_ref, &mut rows_ref);
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
        let fields_ref: Vec<&String> = fields.iter().collect();
        let mut rows_ref: Vec<Vec<&String>> = rows.iter().map(|row| row.iter().collect()).collect();
        let function = Function {
            function_name: Functions::Sort("name".to_string()),
        };
        function.run(&fields_ref, &mut rows_ref);
        let expected_rows = vec![];
        assert!(equal_rows(&expected_rows, &rows_ref))
    }

    #[test]
    fn reverse_sort_test() {
        use super::*;
        let (fields, rows) = get_data();
        let fields_ref: Vec<&String> = fields.iter().collect();
        let mut rows_ref: Vec<Vec<&String>> = rows.iter().map(|row| row.iter().collect()).collect();
        let function = Function {
            function_name: Functions::ReverseSort("name".to_string()),
        };
        function.run(&fields_ref, &mut rows_ref);
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
        let fields_ref: Vec<&String> = fields.iter().collect();
        let mut rows_ref: Vec<Vec<&String>> = rows.iter().map(|row| row.iter().collect()).collect();
        let function = Function {
            function_name: Functions::NReverseSort("points".to_string()),
        };
        function.run(&fields_ref, &mut rows_ref);
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
        let fields_ref: Vec<&String> = fields.iter().collect();
        let mut rows_ref: Vec<Vec<&String>> = rows.iter().map(|row| row.iter().collect()).collect();
        let function = Function {
            function_name: Functions::NSort("points".to_string()),
        };
        function.run(&fields_ref, &mut rows_ref);
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
        let fields_ref: Vec<&String> = fields.iter().collect();
        let mut rows_ref: Vec<Vec<&String>> = rows.iter().map(|row| row.iter().collect()).collect();
        let function = Function {
            function_name: Functions::Sort("age".to_string()),
        };
        function.run(&fields_ref, &mut rows_ref);
        let expected_rows = vec![];
        assert!(equal_rows(&expected_rows, &rows_ref))
    }

    fn equal_rows(expected_rows: &Vec<Vec<&str>>, rows: &Vec<Vec<&String>>) -> bool {
        if expected_rows.len() != rows.len() {
            return false;
        }
        for (expected_row, actual_row) in expected_rows.iter().zip(rows.iter()) {
            if expected_row.len() != actual_row.len() {
                return false;
            }
            for (expected_cell, actual_cell) in expected_row.iter().zip(actual_row.iter()) {
                if *expected_cell != **actual_cell {
                    return false;
                }
            }
        }
        true
    }
}
