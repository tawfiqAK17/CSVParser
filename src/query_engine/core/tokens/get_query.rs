use std::string::ParseError;

use crate::query_engine::core::tokens::ParseResult;
use crate::query_engine::core::tokens::value::parse_field_name;

use super::function_call::FunctionCall;
use super::value;
use super::where_clause::WhereClause;
use indexmap::IndexMap;

#[derive(Debug)]
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
                let mut final_where_clause: Option<WhereClause>;
                let mut final_function_call: Option<FunctionCall>;

                if *lexeme == "*" {
                    selector.push("*".to_string());
                    let (where_clause_parse_result, last_idx) = WhereClause::parse(lexemes, 2);
                    match where_clause_parse_result {
                        ParseResult::Val(where_clause) => {
                            final_where_clause = Some(where_clause);
                        }
                        ParseResult::None => {
                            final_where_clause = None;
                        }
                        ParseResult::Err => {
                            return None;
                        }
                    }
                    let function_call_parse_result = FunctionCall::parse(lexemes, last_idx);
                    match function_call_parse_result {
                        ParseResult::Val(function_call) => {
                            final_function_call = Some(function_call);
                        }
                        ParseResult::None => {
                            final_function_call = None;
                        }
                        ParseResult::Err => {
                            return None;
                        }
                    }
                    return Some(GetQuery {
                        selector,
                        where_clause: final_where_clause,
                        function_call: final_function_call,
                    });
                }

                // the case where the selector is a field name or a list of theme
                if let Some((mut fields_names, idx)) = Self::parse_field_name_list(lexemes, 1) {
                    selector.append(&mut fields_names);
                    let (where_clause_parse_result, last_idx) = WhereClause::parse(lexemes, idx);
                    match where_clause_parse_result {
                        ParseResult::Val(where_clause) => {
                            final_where_clause = Some(where_clause);
                        }
                        ParseResult::None => {
                            final_where_clause = None;
                        }
                        ParseResult::Err => {
                            return None;
                        }
                    }
                    let function_call_parse_result = FunctionCall::parse(lexemes, last_idx);
                    match function_call_parse_result {
                        ParseResult::Val(function_call) => {
                            final_function_call = Some(function_call);
                        }
                        ParseResult::None => {
                            final_function_call = None;
                        }
                        ParseResult::Err => {
                            return None;
                        }
                    }
                    return Some(GetQuery {
                        selector,
                        where_clause: final_where_clause,
                        function_call: final_function_call,
                    });
                } else {
                    eprintln!("a selector was expected for the get command");
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
            } else {
                if fields_names.is_empty() {
                    return None;
                }
                return Some((fields_names, start_idx));
            }
            start_idx += 1;
        }
        if fields_names.is_empty() {
            return None;
        }
        return Some((fields_names, start_idx));
    }
    pub fn evaluate(&self, columns: &IndexMap<String, Vec<String>>) -> () {
        let mut row: Vec<&String> = Vec::new();
        let mut idx = 0;
        let mut cols_number = 0;

        // will hold the rows that satisfies the condition
        let mut valid_rows: Vec<Vec<&String>> = Vec::new();

        if let Some((_key, values)) = columns.first() {
            cols_number = values.len(); // Length of the first Vec<String>
        } else {
            return ();
        }

        // this for loop will evaluate the where condition for every line
        for i in 0..cols_number {
            for key in columns.keys() {
                if let Some(column) = columns.get(key) {
                    row.push(&column[idx]);
                }
            }
            idx += 1;
            if let Some(where_clause) = &self.where_clause {
                if where_clause.evaluate(&columns.keys().collect(), &row) {
                    valid_rows.push(row.clone());
                } else {
                    row.clear();
                    continue;
                }
            } else {
                valid_rows.push(row.clone());
            }
            row.clear();
        }
        if let Some(function_call) = &self.function_call {
            function_call.evaluate(&columns.keys().collect(), &mut valid_rows);
        }
        self.print_result(&columns.keys().collect(), &valid_rows);
        ()
    }
    fn print_result(&self, fields: &Vec<&String>, rows: &Vec<Vec<&String>>) {
        let mut idxs: Vec<usize> = Vec::new();
        let mut select_all: bool = false;
        for selector in self.selector.iter() {
            if selector == "*" {
                select_all = true;
                break;
            }
            match fields.iter().position(|&f| f == selector) {
                Some(idx) => idxs.push(idx),
                None => {
                    eprintln!("no field named {}", selector);
                    return;
                }
            }
        }
        if select_all {
            idxs.clear();
        }
        self.print_rows(&idxs, rows);
    }
    fn print_row(&self, idxs: &Vec<usize>, row: &Vec<&String>) {
        if idxs.is_empty() {
            for val in row {
                print!("{val},");
            }
            println!();
            return;
        }
        for i in 0..row.len() {
            if idxs.contains(&i) {
                print!("{}", row[i]);
            }
        }
        println!();
    }
    fn print_rows(&self, idxs: &Vec<usize>, rows: &Vec<Vec<&String>>) {
        for row in rows {
            self.print_row(idxs, &row);
        }
    }
}
