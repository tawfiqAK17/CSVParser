use std::cmp::Ordering;
use std::collections::HashMap;

use super::ParseResult;
use super::modification::Modification;
use super::where_clause::WhereClause;
use crate::log_error;

#[derive(Debug)]
pub enum AggregationFunction {
    Sum,
    Avg,
    Mean,
    Count,
    Counter,
}
impl AggregationFunction {
    pub fn get_available_functions_names<'a>() -> Vec<&'a str> {
        return vec!["sum", "avg", "mean", "count", "counter"];
    }
    pub fn get_function_from_name(name: &str) -> Option<AggregationFunction> {
        match name {
            "sum" => return Some(AggregationFunction::Sum),
            "avg" => return Some(AggregationFunction::Avg),
            "mean" => return Some(AggregationFunction::Mean),
            "count" => return Some(AggregationFunction::Count),
            "counter" => return Some(AggregationFunction::Counter),
            _ => None,
        }
    }
}
#[derive(Debug)]
pub struct AggregationFunctionCall {
    aggregation_function: AggregationFunction,
    modification: Modification,
    where_clause: Option<WhereClause>,
}

impl AggregationFunctionCall {
    pub fn parse(lexemes: &[String]) -> ParseResult<Self> {
        match lexemes.get(0) {
            Some(lexeme) => match AggregationFunction::get_function_from_name(lexeme) {
                Some(aggregation_function) => {
                    let (modification_parse_result, last_idx) = Modification::parse(lexemes, 1);
                    match modification_parse_result {
                        ParseResult::Val(modification) => {
                            let (where_clause_parse_result, _) =
                                WhereClause::parse(lexemes, last_idx + 1);
                            match where_clause_parse_result {
                                ParseResult::Val(where_clause) => {
                                    return ParseResult::Val(AggregationFunctionCall {
                                        aggregation_function,
                                        modification,
                                        where_clause: Some(where_clause),
                                    });
                                }
                                ParseResult::None => {
                                    return ParseResult::Val(AggregationFunctionCall {
                                        aggregation_function,
                                        modification,
                                        where_clause: None,
                                    });
                                }
                                ParseResult::Err => return ParseResult::Err,
                            }
                        }
                        ParseResult::None => {
                            log_error!("expecting a modification after the function name");
                            return ParseResult::Err;
                        }
                        ParseResult::Err => return ParseResult::Err,
                    }
                }
                None => return ParseResult::None,
            },

            None => return ParseResult::None,
        }
    }

    pub fn evaluate(&self, fields: &Vec<String>, rows: &Vec<Vec<String>>) {
        // will contain all the values returned by the modification on every row that satisfies the
        // where condition
        let mut modification_values: Vec<String> = Vec::new();
        match &self.where_clause {
            Some(where_clause) => {
                for row in rows {
                    if where_clause.evaluate(fields, row) {
                        if let Some(val) = self.modification.evaluate(fields, row) {
                            modification_values.push(val);
                        }
                    }
                }
            }
            None => {
                for row in rows {
                    if let Some(val) = self.modification.evaluate(fields, row) {
                        modification_values.push(val);
                    }
                }
            }
        }

        match &self.aggregation_function {
            AggregationFunction::Sum => {
                let modification_values_as_numbers =
                    Self::vec_str_to_vec_number(&modification_values);
                let mut sum = 0f32;
                for n in modification_values_as_numbers {
                    sum += n;
                }
                println!("{sum}");
            }
            AggregationFunction::Avg => {
                let modification_values_as_numbers =
                    Self::vec_str_to_vec_number(&modification_values);
                let mut sum = 0f32;
                for n in modification_values_as_numbers.iter() {
                    sum += n;
                }
                println!("{}", sum / modification_values_as_numbers.len() as f32);
            }

            AggregationFunction::Mean => {
                let mut modification_values_as_numbers =
                    Self::vec_str_to_vec_number(&modification_values);
                modification_values_as_numbers.sort_by(Self::compaire_floats);
                let center_idx = modification_values_as_numbers.len() / 2;
                if modification_values_as_numbers.len().is_multiple_of(2) {
                    let mean_2: f32 = modification_values_as_numbers[center_idx]
                        + modification_values_as_numbers[center_idx + 1];
                    println!("{}", mean_2 / 2f32);
                    return;
                }
                println!("{}", modification_values_as_numbers[center_idx]);
            }

            AggregationFunction::Count => {
              println!("{}", modification_values.len());
            }
            AggregationFunction::Counter => {
                let mut counter: HashMap<String, usize> = HashMap::new();
                let mut longer_val_len = 0; // to make the output clean
                for val in modification_values {
                    if val.len() > longer_val_len {
                      longer_val_len = val.len()
                    }
                    *counter.entry(val).or_insert(0) += 1;
                }
                for key in counter.keys() {
                    print!("{key}");
                    for _ in key.len()..longer_val_len {
                      print!(" ")
                    }
                    println!(" :{}", counter[key]);
                }
            }
        }
    }
    fn compaire_floats(f1: &f32, f2: &f32) -> Ordering {
        if f1 - f2 > 0f32 {
            Ordering::Greater
        } else if f1 - f2 < 0f32 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
    fn vec_str_to_vec_number(vector: &Vec<String>) -> Vec<f32> {
        let mut vec_as_numbers: Vec<f32> = Vec::new();
        for s in vector {
            match s.parse::<f32>() {
                Ok(number) => vec_as_numbers.push(number),
                Err(_) => {
                    log_error!("'{s}' is not a numerical value");
                }
            }
        }
        return vec_as_numbers;
    }
}
