use super::ParseResult;
use super::where_clause::WhereClause;
use crate::log_error;

#[derive(Debug)]
pub struct DeleteQuery {
    where_clause: WhereClause,
}

impl DeleteQuery {
    pub fn parse(lexemes: &[String]) -> ParseResult<Self> {
        if let Some(lexeme) = lexemes.get(0) {
            if *lexeme != "delete" {
                return ParseResult::None;
            }
        } else {
            return ParseResult::None;
        }

        match lexemes.get(1) {
            Some(_) => {
                let (where_clause_parse_result, _) = WhereClause::parse(lexemes, 1);
                match where_clause_parse_result {
                    ParseResult::Val(where_clause) => {
                        return ParseResult::Val(DeleteQuery { where_clause });
                    }
                    ParseResult::None => {
                        log_error!("expecting a where clause after the delete key word");
                        return ParseResult::Err;
                    }

                    ParseResult::Err => return ParseResult::Err,
                }
            }
            None => {
                log_error!("expecting a where clause after the delete key word");
                return ParseResult::Err;
            }
        }
    }

    pub fn evaluate(&self, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) -> () {
      rows.retain(|row| !self.where_clause.evaluate(fields, row));
    }
}
