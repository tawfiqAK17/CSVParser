mod tokens_parser;
use crate::query_engine::core::tokens::Tokens;

pub fn parse(query: String) -> Result<Vec<Tokens>, String> {
    let mut parser = tokens_parser::TokensParser::new(query);
    parser.parse()
}
