use super::function::Function;
#[derive(Debug)]
pub struct FunctionCall {
    function: Option<Function>,
    function_call: Option<Box<FunctionCall>>,
}

impl FunctionCall {
    pub fn parse(lexemes: &[&String], idx: usize) -> Option<Self> {
        match lexemes.get(idx) {
            Some(_) => {}
            None => return None,
        }
        let (function_option, last_idx) = Function::parse(lexemes, idx);
        match function_option {
            Some(function) => {
              println!("{last_idx}");
                match Self::parse(lexemes, last_idx) {
                    Some(function_call) => {
                      return Some(FunctionCall{function: Some(function), function_call: Some(Box::new(function_call))});
                    },
                    None => return Some(FunctionCall{function: Some(function), function_call: None}),
                }
            }
            None => return None,
        }
    }
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
