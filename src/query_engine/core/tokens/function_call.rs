use super::function::Function;
pub struct FunctionCall {
    function: Option<Function>,
    function_call: Option<Box<FunctionCall>>,
}

impl FunctionCall {
    pub fn evaluate(&self, fields: &Vec<&String>, mut valid_rows: &mut Vec<Vec<&String>>) -> () {
        if let Some(function) = &self.function {
            function.run(&fields, &mut valid_rows);
        }
        if let Some(function_call) = &self.function_call {
            function_call.evaluate(&fields, &mut valid_rows);
        }
        self.print_rows(&valid_rows);
    }
    fn print_row(&self, row: &Vec<&String>) {
        for val in row {
            print!("{val},");
        }
        println!();
    }

    fn print_rows(&self, rows: &Vec<Vec<&String>>) {
        for row in rows {
            self.print_row(&row);
        }
    }
}
