use super::function_call::FunctionCall;
use super::where_clause::WhereClause;
use indexmap::IndexMap;

pub struct GetQuery {
    selector: Vec<String>,
    where_clause: Option<WhereClause>,
    function_call: Option<FunctionCall>,
}

impl GetQuery {
    pub fn evaluate(&self, columns: &IndexMap<String, Vec<String>>) -> () {
        let mut row: Vec<&String> = Vec::new();
        let mut idx = 0;

        // will hold the rows that satisfies the condition if there is a function call at the end
        let mut valid_rows: Vec<Vec<&String>> = Vec::new();

        // this for loop will evaluate the where condition for every line
        for _ in [0..columns.len()] {
            for key in columns.keys() {
                if let Some(column) = columns.get(key) {
                    row.push(&column[idx]);
                }
            }
          if let Some(where_clause) = &self.where_clause {
                if where_clause.evaluate(&row) {
                    if let Some(_) = &self.function_call {
                        valid_rows.push(row.clone());
                    } else {
                        self.print_row(&row);
                    }
                }
            } else {
                if let Some(_) = &self.function_call {
                    valid_rows.push(row.clone());
                } else {
                    self.print_row(&row);
                }
            }
            idx = 0;
        }
        if let Some(function_call) = &self.function_call {
            function_call.evaluate(&columns.keys().collect(), &mut valid_rows);
        }
        ()
    }
    fn print_row(&self, row: &Vec<&String>) {
        for val in row {
            print!("{val},");
        }
        println!();
    }

    fn print_rows(&self, rows: Vec<Vec<&String>>) {
      for row in rows {
        self.print_row(&row);
      }
    }
}
