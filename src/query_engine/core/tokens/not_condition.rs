use super::primary_condition::PrimaryCondition;
use super::ParseResult;

#[derive(Debug)]
pub struct NotCondition {
    not: Option<()>,
    primary_condition: PrimaryCondition,
}

impl NotCondition {
    pub fn parse(lexemes: &[&String], mut idx: usize) -> (ParseResult<Self>, usize) {
        let mut not: Option<()> = None;
        if let Some(lexeme) = lexemes.get(idx) {
            if *lexeme == "not" {
                not = Some(());
                idx += 1;
            }
        } else {
            // it is the end of the lexemes
            return (ParseResult::None, idx);
        }
        match lexemes.get(idx) {
            Some(lexeme) => {
                let (primary_condition_parse_result, last_idx) = PrimaryCondition::parse(lexemes, idx);
                match primary_condition_parse_result {
                    ParseResult::Val(primary_condition) => {
                        return (ParseResult::Val(NotCondition {
                            not,
                            primary_condition,
                        }), last_idx);
                    }
                    ParseResult::None => {
                        return (ParseResult::None, idx);
                    }
                    ParseResult::Err => {
                        return (ParseResult::Err, idx);
                    }
                }
            }
            None => {
                eprintln!("expecting a condition after the not key word");
                return (ParseResult::Err, idx);
            }
        }
    }
    pub fn evaluate(&self, fields: &Vec<&String>, row: &Vec<&String>) -> bool {
        match &self.not {
            Some(_) => return !self.primary_condition.evaluate(fields, row),
            None => return self.primary_condition.evaluate(fields, row),
        }
    }
}
