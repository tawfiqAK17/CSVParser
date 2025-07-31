pub struct WhereClause {

}

impl WhereClause {
    pub fn evaluate(&self, row: &Vec<&String>) -> bool {
      true
    }
}
