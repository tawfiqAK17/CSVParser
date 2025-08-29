use super::ParseResult;
use super::assignment::Assignment;
#[derive(Debug)]
pub struct AssignList {
    assignments: Vec<Assignment>,
}

impl AssignList {
    pub fn parse(lexemes: &[String], idx: usize) -> (ParseResult<Self>, usize) {
        let mut assignments: Vec<Assignment> = Vec::new();
        let mut current_idx = idx;
        loop {
            let (assignment_parse_result, last_idx) = Assignment::parse(lexemes, current_idx);
            current_idx = last_idx + 1;
            match assignment_parse_result {
                ParseResult::Val(assignment) => assignments.push(assignment),
                ParseResult::None => {
                    if assignments.is_empty() {
                        return (ParseResult::None, idx);
                    }
                    return (ParseResult::Val(AssignList { assignments }), last_idx);
                }
                ParseResult::Err => return (ParseResult::Err, idx),
            }
        }
    }
    pub fn set_evaluation(&self, fields: &Vec<String>, row: &mut Vec<String>) {
        for assignment in self.assignments.iter() {
            assignment.set_evaluation(fields, row);
        }
    }
    pub fn insert_column_evaluation(
        &self,
        fields: &mut Vec<String>,
        rows: &mut Vec<Vec<String>>,
        where_clause_eval_results: &Vec<bool>,
    ) {
        for assignment in self.assignments.iter() {
            assignment.insert_column_evaluation(fields, rows, where_clause_eval_results);
        }
    }
    pub fn insert_row_evaluation(&self, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) {
        let mut new_row: Vec<String> = vec!["".to_string(); fields.len()];
        for assignment in self.assignments.iter() {
            assignment.set_evaluation(fields, &mut new_row);
        }
        rows.push(new_row);
    }
}
