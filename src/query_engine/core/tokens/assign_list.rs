use super::ParseResult;
use super::assignment::Assignment;
#[derive(Debug)]
pub struct AssignList {
    assignments: Vec<Assignment>,
}

impl AssignList {
    pub fn parse(lexemes: &[&String], idx: usize) -> ParseResult<Self> {
        let mut assignments: Vec<Assignment> = Vec::new();
        let mut current_idx = idx;
        loop {
            let (assignment_parse_result, last_idx) = Assignment::parse(lexemes, current_idx);
            current_idx = last_idx + 1;
            match assignment_parse_result {
                ParseResult::Val(assignment) => assignments.push(assignment),
                ParseResult::None => {
                    if assignments.is_empty() {
                        return ParseResult::None;
                    }
                    return ParseResult::Val(AssignList { assignments });
                }
                ParseResult::Err => return ParseResult::Err,
            }
        }
    }
    pub fn evaluate(&self, fields: &Vec<String>, row: &mut Vec<String>) -> () {
        for assignment in self.assignments.iter() {
            assignment.evaluate(fields, row);
        }
        ()
    }
}
