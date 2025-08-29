use super::ParseResult;
use super::function_call::FunctionCall;
use super::value;
use super::where_clause::WhereClause;
use crate::OPTIONS;
use crate::log_error;
use terminal_size::{Width, terminal_size};

#[derive(Debug)]
pub struct GetQuery {
    selector: Vec<String>,
    where_clause: Option<WhereClause>,
    function_call: Option<FunctionCall>,
}

impl GetQuery {
    pub fn parse(lexemes: &[String]) -> ParseResult<Self> {
        let mut selector: Vec<String> = Vec::new();
        if let Some(lexeme) = lexemes.get(0) {
            if *lexeme != "get" {
                // if the first lexeme is not get than return None
                return ParseResult::None;
            }
        }
        match lexemes.get(1) {
            Some(lexeme) => {
                if *lexeme == "*" {
                    // the case where the user select all fields (ex: get * ....)
                    selector.push("*".to_string());
                    match Self::parse_where_clause_and_function_call(lexemes, 2) {
                        ParseResult::Val((where_clause, function_call)) => {
                            return ParseResult::Val(GetQuery {
                                selector,
                                where_clause,
                                function_call,
                            });
                        }
                        ParseResult::None => return ParseResult::None,
                        ParseResult::Err => return ParseResult::Err,
                    }
                }
                // the case where the selector is a field name or a list of theme
                if let Some((mut fields_names, idx)) = Self::parse_field_name_list(lexemes, 1) {
                    selector.append(&mut fields_names); // adding the fields names to the selector

                    match Self::parse_where_clause_and_function_call(lexemes, idx + 1) {
                        ParseResult::Val((where_clause, function_call)) => {
                            return ParseResult::Val(GetQuery {
                                selector,
                                where_clause,
                                function_call,
                            });
                        }
                        ParseResult::None => return ParseResult::None,
                        ParseResult::Err => return ParseResult::Err,
                    }
                } else {
                    log_error!("a selector was expected for the get command");
                    return ParseResult::Err;
                }
            }
            None => {
                log_error!("a selector was expected for the get command");
                return ParseResult::Err;
            }
        }
    }
    pub fn parse_where_clause_and_function_call(
        lexemes: &[String],
        idx: usize,
    ) -> ParseResult<(Option<WhereClause>, Option<FunctionCall>)> {
        let final_where_clause: Option<WhereClause>;
        let final_function_call: Option<FunctionCall>;
        let (where_clause_parse_result, last_idx) = WhereClause::parse(lexemes, idx);

        match where_clause_parse_result {
            ParseResult::Val(where_clause) => {
                final_where_clause = Some(where_clause);
            }
            ParseResult::None => {
                final_where_clause = None;
            }
            ParseResult::Err => {
                return ParseResult::Err;
            }
        }
        let function_call_parse_result = FunctionCall::parse(lexemes, last_idx + 1);
        match function_call_parse_result {
            ParseResult::Val(function_call) => {
                final_function_call = Some(function_call);
            }
            ParseResult::None => {
                final_function_call = None;
            }
            ParseResult::Err => {
                return ParseResult::Err;
            }
        }
        return ParseResult::Val((final_where_clause, final_function_call));
    }
    // a helper method that parse the list of fields names
    fn parse_field_name_list(
        lexemes: &[String],
        mut start_idx: usize,
    ) -> Option<(Vec<String>, usize)> {
        let mut fields_names: Vec<String> = Vec::new();
        // while there is next lexeme
        while let Some(lexeme) = lexemes.get(start_idx) {
            // if the lexeme is a field name we add it to the list
            if let Some(field_name) = value::parse_field_name(lexeme) {
                fields_names.push(field_name.clone());
            } else {
                // if the lexeme is not a field name that means we reached the end of the
                // selectors list
                break;
            }
            start_idx += 1;
        }
        if fields_names.is_empty() {
            // there was no field name
            return None;
        }
        return Some((fields_names, start_idx - 1));
    }
    pub fn evaluate(&self, fields: &Vec<String>, rows: &mut Vec<Vec<String>>) {
        // will hold the rows that satisfies the condition
        let mut valid_rows: Vec<&Vec<String>> = Vec::new();

        if let Some(where_clause) = &self.where_clause {
            // this for loop will evaluate the where condition for every line
            for i in 0..rows.len() {
                if where_clause.evaluate(fields, &rows[i]) {
                    // the line satisfies the condition
                    valid_rows.push(&rows[i]);
                }
            }
        } else {
            // all lines are valid because there is no where clause
            for i in 0..rows.len() {
                valid_rows.push(&rows[i]);
            }
        }
        if let Some(function_call) = &self.function_call {
            // evaluate the function call on the valid rows
            function_call.evaluate(fields, &mut valid_rows);
        }
        self.print_result(fields, &valid_rows);
    }

    fn print_result(&self, fields: &Vec<String>, rows: &Vec<&Vec<String>>) {
        println!();
        let mut idxs: Vec<usize> = Vec::new();
        for selector in self.selector.iter() {
            if selector == "*" {
                idxs = (0..fields.len()).collect();
                break;
            }
            match fields.iter().position(|f| f == selector) {
                Some(idx) => idxs.push(idx),
                None => {
                    log_error!("no field named {}", selector);
                    return;
                }
            }
        }
        let mut longest_vals_len = self.get_longest_vals_in_rows(idxs.clone(), fields, rows);
        let terminal_width: u16; // the width of the terminal (in char)

        if let Some((Width(w), _)) = terminal_size() {
            terminal_width = w;
        } else {
            terminal_width = u16::MAX; // it there is an error set the with to the max
        }

        // while the length of the longest values + the separator and the space after the separator
        // is greater than the terminal width we cut the longest value to half and add the 3 for
        // the 3 dotes that indicates the value isn't fully printed
        while longest_vals_len.iter().sum::<usize>() + 2 * idxs.len() > terminal_width as usize {
            if let Some(max_val) = longest_vals_len.iter().max() {
                if let Some(max_val_idx) = longest_vals_len.iter().position(|i| i == max_val) {
                    longest_vals_len[max_val_idx] = max_val / 2 + 3;
                }
            }
        }
        self.print_rows(&idxs, fields, rows, &longest_vals_len);
    }
    // this method returns the length longest value in each column
    fn get_longest_vals_in_rows(
        &self,
        mut idxs: Vec<usize>,
        fields: &Vec<String>,
        rows: &Vec<&Vec<String>>,
    ) -> Vec<usize> {
        if idxs.is_empty() {
            // the user is using the * selector so all fields will be included
            idxs = (0..fields.len()).collect();
        }
        let mut longest_vals: Vec<usize> = vec![0; fields.len()];

        for i in idxs.iter() {
            longest_vals[i.clone()] = fields[i.clone()].len();
        }

        for i in idxs.iter() {
            for row in rows.iter() {
                if row[i.clone()].len() > longest_vals[i.clone()] {
                    longest_vals[i.clone()] = row[i.clone()].len();
                }
            }
        }
        return longest_vals;
    }
    fn print_rows(
        &self,
        idxs: &Vec<usize>,
        fields: &Vec<String>,
        rows: &Vec<&Vec<String>>,
        longerst_vals_len: &Vec<usize>,
    ) {
        let options = OPTIONS.get().unwrap();
        let separator: &String;
        match options.get(&crate::Options::FieldsSeparator) {
            Some(sep) => separator = sep,
            None => unreachable!("there is no default value for the separator option"),
        }
        self.print_row(idxs, &fields, longerst_vals_len, separator);
        for row in rows {
            self.print_row(idxs, &row, longerst_vals_len, separator);
        }
    }

    fn print_row(
        &self,
        idxs: &Vec<usize>,
        row: &Vec<String>,
        longerst_vals_len: &Vec<usize>,
        separator: &String,
    ) {
        for &i in idxs {
            if row[i].len() > longerst_vals_len[i] {
                print!("{}...", &row[i][0..longerst_vals_len[i] - 3]);
            } else {
                print!("{}", row[i]);
                for _ in row[i].len()..longerst_vals_len[i] {
                    print!(" ");
                }
            }
            // the last val in a row wont have a , after it
            if i < idxs[idxs.len() - 1] {
                print!("{} ", separator);
            }
        }
        println!();
        return;
    }
}
