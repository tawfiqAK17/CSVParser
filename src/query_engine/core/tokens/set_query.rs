use super::ParseResult;
use super::assign_list::AssignList;
use super::where_clause::WhereClause;

#[derive(Debug)]
pub struct SetQuery {
    where_clause: Option<WhereClause>,
    assign_list: AssignList,
}

impl SetQuery {
    pub fn parse(lexemes: &[String]) -> ParseResult<Self> {
        if let Some(lexeme) = lexemes.get(0) {
            if *lexeme != "set" {
                return ParseResult::None;
            }
        } else {
            return ParseResult::None;
        }
        match lexemes.get(1) {
            Some(lexeme) => {
                let where_clause: Option<WhereClause>;
                let assign_list: AssignList;
                if *lexeme == "where" { // the case where there is a where clause
                    let (where_clause_parse_result, last_idx) = WhereClause::parse(lexemes, 1);
                    match where_clause_parse_result {
                        ParseResult::Val(val) => where_clause = Some(val),
                        ParseResult::None => where_clause = None,
                        ParseResult::Err => return ParseResult::Err,
                    }
                    if let Some(to_lexeme) = lexemes.get(last_idx + 1) {
                        if to_lexeme.as_str() == "to" {
                            let assign_list_parse_result = AssignList::parse(lexemes, last_idx + 2);
                            match assign_list_parse_result {
                                ParseResult::Val(val) => {
                                    println!("val");
                                    assign_list = val;
                                    return ParseResult::Val(SetQuery {
                                        where_clause,
                                        assign_list,
                                    });
                                }
                                ParseResult::None => {
                                    println!("none");
                                    eprintln!(
                                        "expecting a where clause or an assign list after the set key word"
                                    );
                                    return ParseResult::Err;
                                }
                                ParseResult::Err => {
                                    println!("err");
                                    return ParseResult::Err;
                                }
                            }
                        } else {
                            eprintln!(
                                "missing a 'to' key word at the beginning of the assigning list"
                            );
                            return ParseResult::Err;
                        }
                    } else {
                        eprintln!("expecting an assigning list for the set command");
                        return ParseResult::Err;
                    }
                } else if *lexeme == "to" { // the case where there is only the assigning list
                    let assign_list_parse_result = AssignList::parse(lexemes, 2);
                    match assign_list_parse_result {
                        ParseResult::Val(val) => {
                            assign_list = val;
                            return ParseResult::Val(SetQuery {
                                where_clause: None,
                                assign_list,
                            });
                        }
                        ParseResult::None => {
                            eprintln!(
                                "expecting a where clause or an assign list after the set key word"
                            );
                            return ParseResult::Err;
                        }
                        ParseResult::Err => {
                            return ParseResult::Err;
                        }
                    }
                } else {
                    eprintln!("expecting a where clause or an assign list after the set key word");
                    return ParseResult::Err;
                }
            }
            None => {
                eprintln!("expecting a where clause or an assign list after the set key word");
                return ParseResult::Err;
            }
        }
    }
    pub fn evaluate(&self, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) -> () {
        if let Some(where_clause) = &self.where_clause {
            // this for loop will evaluate the where condition for every line
            for i in 0..rows.len() {
                if where_clause.evaluate(fields, &rows[i]) {
                    self.assign_list.evaluate(fields, &mut rows[i]);
                }
            }
        } else {
            for i in 0..rows.len() {
                self.assign_list.evaluate(fields, &mut rows[i]);
            }
        }
        ()
    }
}
