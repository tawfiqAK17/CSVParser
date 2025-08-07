use crate::query_engine::core::tokens::value::parse_field_name;

use super::function_call::FunctionCall;
use super::value;
use super::where_clause::WhereClause;
use indexmap::IndexMap;

pub struct GetQuery {
    selector: Vec<String>,
    where_clause: Option<WhereClause>,
    function_call: Option<FunctionCall>,
}

impl GetQuery {
    pub fn parse(lexemes: &[&String]) -> Option<Self> {
        let mut selector: Vec<String> = Vec::new();
        if let Some(lexeme) = lexemes.get(0) {
            if *lexeme != "get" {
                return None;
            }
        }
        match lexemes.get(1) {
            Some(lexeme) => {
                if *lexeme == "*" {
                    selector.push("*".to_string());
                    let (where_clause, last_idx): (Option<WhereClause>, usize) =
                        WhereClause::parse(lexemes, 2);
                    let function_call = FunctionCall::parse(lexemes, last_idx);
                    return Some(GetQuery {
                        selector,
                        where_clause,
                        function_call,
                    });
                }
                // the case where the selector is a field name or a list of theme
                if let Some((mut fields_names, idx)) = Self::parse_field_name_list(lexemes, 1) {
                    selector.append(&mut fields_names);
                    let (where_clause, last_idx) = WhereClause::parse(lexemes, idx);
                    let function_call = FunctionCall::parse(lexemes, last_idx);
                    return Some(GetQuery {
                        selector,
                        where_clause,
                        function_call,
                    });
                } else {
                    eprintln!("{:?}", selector);
                    eprintln!("1 a selector was expected for the get command");
                    return None;
                }
            }
            None => {
                eprintln!("missing a selector after the get command");
                return None;
            }
        }
    }
    // a helper method that parse the list of fields names
    fn parse_field_name_list(
        lexemes: &[&String],
        mut start_idx: usize,
    ) -> Option<(Vec<String>, usize)> {
        let mut fields_names: Vec<String> = Vec::new();
        while let Some(lexeme) = lexemes.get(start_idx) {
            if let Some(field_name) = value::parse_field_name(lexeme) {
                fields_names.push(field_name.clone());
                start_idx += 1;
            } else {
                if fields_names.is_empty() {
                    return None;
                }
                return Some((fields_names, start_idx));
            }
        }
        if fields_names.is_empty() {
            return None;
        }
        return Some((fields_names, start_idx));
    }
    pub fn evaluate(&self, columns: &IndexMap<String, Vec<String>>) -> () {
        let mut row: Vec<&String> = Vec::new();
        let mut idx = 0;

        // will hold the rows that satisfies the condition if there is a function call at the end
        let mut valid_rows: Vec<Vec<&String>> = Vec::new();

        // this for loop will evaluate the where condition for every line
        for _ in [0..columns.len()] {
            for key in columns.keys() {
                if let Some(column) = columns.get(key) {
                    row.push(&column[idx]);
                }
            }
            if let Some(where_clause) = &self.where_clause {
                if where_clause.evaluate(&columns.keys().collect(), &row) {
                    if let Some(_) = &self.function_call {
                        valid_rows.push(row.clone());
                    } else {
                        self.print_row(&row);
                    }
                }
            } else {
                if let Some(_) = &self.function_call {
                    valid_rows.push(row.clone());
                } else {
                    self.print_row(&row);
                }
            }
            idx = 0;
        }
        if let Some(function_call) = &self.function_call {
            function_call.evaluate(&columns.keys().collect(), &mut valid_rows);
        }
        ()
    }
    fn print_row(&self, row: &Vec<&String>) {
        for val in row {
            print!("{val},");
        }
        println!();
    }
    fn print_rows(&self, rows: Vec<Vec<&String>>) {
        for row in rows {
            self.print_row(&row);
        }
    }
}
