use crate::query_engine::core::tokens::ParseResult;

use super::function::Function;
#[derive(Debug)]
pub struct FunctionCall {
    function: Option<Function>,
    function_call: Option<Box<FunctionCall>>,
}

impl FunctionCall {
    pub fn parse(lexemes: &[&String], idx: usize) -> ParseResult<Self> {
        match lexemes.get(idx) {
            Some(_) => {}
            None => return ParseResult::None,
        }
        let (function_option, last_idx) = Function::parse(lexemes, idx);
        match function_option {
            ParseResult::Val(function) => {
                match Self::parse(lexemes, last_idx) {
                    ParseResult::Val(function_call) => {
                      return ParseResult::Val(FunctionCall{function: Some(function), function_call: Some(Box::new(function_call))});
                    },
                    ParseResult::None => return ParseResult::Val(FunctionCall{function: Some(function), function_call: None}),
                    ParseResult::Err => return ParseResult::Err,
                }
            }
            ParseResult::None => ParseResult::None,
            ParseResult::Err => ParseResult::Err,
        }
    }
    pub fn evaluate(&self, fields: &Vec<String>, valid_rows: &mut Vec<&Vec<String>>) -> () {
        if let Some(function) = &self.function {
            function.run(&fields, valid_rows);
        }
        if let Some(function_call) = &self.function_call {
            function_call.evaluate(&fields, valid_rows);
        }
    }
}
