pub mod and_condition;
pub mod comparison;
pub mod condition;
pub mod function;
pub mod function_call;
pub mod get_query;
pub mod not_condition;
pub mod or_condition;
pub mod primary_condition;
pub mod query;
pub mod set_query;
pub mod value;
pub mod where_clause;
pub mod assign_list;
pub mod assignment;
pub mod modification;
pub mod insert_column_query;
pub mod insert_row_query;
pub mod delete_query;
pub mod aggregation_function_call;

pub enum ParseResult<T> {
    Val(T),
    None,
    Err,
}
