use super::ParseResult;
use super::function_call::FunctionCall;
use super::value;
use super::where_clause::WhereClause;

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
                    eprintln!("a selector was expected for the get command");
                    return ParseResult::Err;
                }
            }
            None => {
                eprintln!("a selector was expected for the get command");
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
        let mut idxs: Vec<usize> = Vec::new();
        let mut select_all: bool = false;
        for selector in self.selector.iter() {
            if selector == "*" {
                select_all = true;
                break;
            }
            match fields.iter().position(|f| f == selector) {
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
    fn print_row(&self, idxs: &Vec<usize>, row: &Vec<String>) {
        if idxs.is_empty() {
            for val in row {
                print!("{val},");
            }
            println!();
            return;
        }
        for i in 0..row.len() {
            if idxs.contains(&i) {
                print!("{},", row[i]);
            }
        }
        println!();
    }
    fn print_rows(&self, idxs: &Vec<usize>, rows: &Vec<&Vec<String>>) {
        for row in rows {
            self.print_row(idxs, &row);
        }
    }
}
