use super::assignment::Assignment;
#[derive(Debug)]
pub struct AssignList {
  assignments: Vec<Assignment>
}

impl AssignList {
    pub fn evaluate(&self, fields: &Vec<String>, row: &Vec<String>) -> () {
        for assignment in self.assignments.iter() {
          assignment.evaluate(fields, row);
        }
    ()
    }
}
