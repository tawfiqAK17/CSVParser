use super::where_clause::WhereClause;
use super::assign_list::AssignList;

#[derive(Debug)]
pub struct SetQuery {
    where_clause: Option<WhereClause>,
    assign_list: AssignList
}

impl SetQuery {
    pub fn evaluate(&self, fields: &mut Vec<String>, rows: &mut Vec<Vec<String>>) -> () {
        if let Some(where_clause) = &self.where_clause {
            // this for loop will evaluate the where condition for every line
            for i in 0..rows.len() {
                if where_clause.evaluate(fields, &rows[i]) {
                  self.assign_list.evaluate(fields, &rows[i]);
                }
            }
        } else { 
            for i in 0..rows.len() {
                self.assign_list.evaluate(fields, &rows[i]);
            }
        }
        ()
    }
}
