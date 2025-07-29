use std::fmt::format;

use crate::query_engine::core::tokens::Tokens;


#[derive(Debug)]
pub struct TokensParser {
    query: Vec<String>,
    index: usize,
    tokens: Vec<Tokens>,
}

enum ParseResult {
    Ok,          // the item exist
    None,        // the item doesn't exist
    Err(String), // the item exist but there is a syntax error
}

impl TokensParser {
    pub fn new(query: String) -> TokensParser {
        TokensParser {
            query: query.split(' ').map(|item| item.to_string()).collect(),
            index: 0,
            tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Tokens>, String> {
        println!("{self:?}");
        match self.query() {
            ParseResult::Ok => {}
            ParseResult::None => {}
            ParseResult::Err(error) => return Err(error),
        }
        Ok(self.tokens.clone())
    }

    fn end_of_query(&mut self) -> bool {
        self.index >= self.query.len()
    }

    fn consume_literal(&mut self, literal: &str) -> bool {
        if self.end_of_query() {
            return false;
        }
        if self.query[self.index] == literal.to_string() {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn query(&mut self) -> ParseResult {
        match self.command() {
            ParseResult::Ok => {}
            ParseResult::None => {
                if self.end_of_query() {
                    return ParseResult::Ok;
                }
                return ParseResult::Err("a command expected".to_string());
            }
            ParseResult::Err(error) => return ParseResult::Err(error),
        }
        match self.selector() {
            ParseResult::Ok => {}
            ParseResult::None => {
                return ParseResult::Err("a selector expected".to_string());
            }
            ParseResult::Err(error) => {
                return ParseResult::Err(error);
            }
        }
        match self.where_clause() {
            ParseResult::Ok => {}
            ParseResult::None => {
                if !self.end_of_query() {
                    return ParseResult::Err("a where clause expected".to_string());
                }
                return ParseResult::Ok;
            }
            ParseResult::Err(error) => {
                return ParseResult::Err(error);
            }
        }
        ParseResult::Ok
    }

    fn command(&mut self) -> ParseResult {
        if self.end_of_query() {
            return ParseResult::None;
        }
        if self.consume_literal("get") {
            self.tokens.push(Tokens::Command("get".to_string()));
            return ParseResult::Ok;
        }
        if self.consume_literal("set") {
            self.tokens.push(Tokens::Command("set".to_string()));
            return ParseResult::Ok;
        }
        ParseResult::Err(format(format_args!(
            "no command named {}",
            self.query[self.index]
        )))
    }

    fn selector(&mut self) -> ParseResult {
        if self.consume_literal("*") {
            self.tokens.push(Tokens::AllSelector());
            return ParseResult::Ok;
        }
        self.field_list()
    }

    fn field_list(&mut self) -> ParseResult {
        match self.field_name() {
            ParseResult::Ok => {}
            ParseResult::None => {
                return ParseResult::None;
            }
            ParseResult::Err(error) => {
                return ParseResult::Err(error);
            }
        }
        if self.consume_literal(",") {
            return self.field_name();
        }
        ParseResult::Ok
    }

    fn where_clause(&mut self) -> ParseResult {
        if !self.consume_literal("where") {
            return ParseResult::None;
        }
        self.tokens.push(Tokens::Where());
        match self.condition() {
            ParseResult::Ok => {}
            ParseResult::None => {
                return ParseResult::Err("missing a condition for the where clause".to_string());
            }
            ParseResult::Err(error) => {
                return ParseResult::Err(error);
            }
        }
        ParseResult::Ok
    }

    fn condition(&mut self) -> ParseResult {
        match self.expression() {
            ParseResult::Ok => {}
            ParseResult::None => {
                return ParseResult::None;
            }
            ParseResult::Err(error) => {
                return ParseResult::Err(error);
            }
        }
        match self.logical_op() {
            ParseResult::Ok => {}
            ParseResult::None => {
                return ParseResult::Ok;
            }
            ParseResult::Err(_) => {
                return ParseResult::Ok;
            }
        }
        return self.condition();
    }

    fn expression(&mut self) -> ParseResult {
        self.comparison()
    }

    fn comparison(&mut self) -> ParseResult {
        match self.comparable() {
            ParseResult::Ok => {}
            ParseResult::None => {
                return ParseResult::None;
            }
            ParseResult::Err(error) => {
                return ParseResult::Err(error);
            }
        }
        match self.comparison_op() {
            ParseResult::Ok => {}
            ParseResult::None => {
                return ParseResult::Err("missing a comparison operator".to_string());
            }
            ParseResult::Err(error) => {
                return ParseResult::Err(error);
            }
        }
        match self.comparable() {
            ParseResult::Ok => {
                return ParseResult::Ok;
            }
            ParseResult::None => {
                return ParseResult::Err("missing the right hand side comparable".to_string());
            }
            ParseResult::Err(error) => {
                return ParseResult::Err(error);
            }
        }
    }

    fn comparable(&mut self) -> ParseResult {
        match self.value() {
            ParseResult::Ok => {
                return ParseResult::Ok;
            }
            ParseResult::None => {}
            ParseResult::Err(error) => {
              return ParseResult::Err(error);
            }
        }
        return self.field_name();
    }

    fn logical_op(&mut self) -> ParseResult {
        if self.consume_literal("and") {
            self.tokens.push(Tokens::LogicalOp("and".to_string()));
            return ParseResult::Ok;
        }
        if self.consume_literal("or") {
            self.tokens.push(Tokens::LogicalOp("or".to_string()));
            return ParseResult::Ok;
        }
        if self.end_of_query() {
            return ParseResult::None;
        }
        ParseResult::Err(format(format_args!(
            "no logical operator named {}",
            self.query[self.index]
        )))
    }

    fn comparison_op(&mut self) -> ParseResult {
        let operators = vec![
            "==",
            "!=",
            ">",
            "<",
            ">=",
            "<=",
            "contains",
            "in",
            "start-with",
            "ends-with",
            "matches",
            "like",
            "between",
        ];
        for operator in operators {
            if self.consume_literal(operator) {
                self.tokens.push(Tokens::ComparisonOp(operator.to_string()));
                return ParseResult::Ok;
            }
        }
        if self.end_of_query() {
            return ParseResult::None;
        }
        ParseResult::Err(format(format_args!(
            "no comparison operator named {}",
            self.query[self.index]
        )))
    }

    fn value(&mut self) -> ParseResult {
        match self.number() {
            ParseResult::Ok => {
                return ParseResult::Ok;
            }
            ParseResult::None => {}
            ParseResult::Err(error) => return ParseResult::Err(error),
        }
        match self.boolean() {
            ParseResult::Ok => {
                return ParseResult::Ok;
            }
            ParseResult::None => {}
            ParseResult::Err(error) => return ParseResult::Err(error),
        }
        match self.literal() {
            ParseResult::Ok => {
                return ParseResult::Ok;
            }
            ParseResult::None => {}
            ParseResult::Err(error) => return ParseResult::Err(error),
        }
        ParseResult::None
    }

    fn literal(&mut self) -> ParseResult {
        if self.end_of_query() {
            return ParseResult::None;
        }
        let current: &str = &self.query[self.index];
        if current.starts_with("\"") && current.ends_with("\"") {
            self.tokens
                .push(Tokens::Literal(current.to_string().replace("\"", "")));
            return ParseResult::Ok;
        }
        ParseResult::None
    }

    fn number(&mut self) -> ParseResult {
        if self.end_of_query() {
            return ParseResult::None;
        }
        match self.query[self.index].parse::<isize>() {
            Ok(val) => {
                self.tokens.push(Tokens::Number(val));
                self.index += 1;
            }
            Err(_) => {
                return ParseResult::None;
            }
        }
        ParseResult::Ok
    }

    fn boolean(&mut self) -> ParseResult {
        if self.end_of_query() {
            return ParseResult::None;
        }
        if self.consume_literal("true") {
            self.tokens.push(Tokens::Boolean(true));
            return ParseResult::Ok;
        }
        if self.consume_literal("false") {
            self.tokens.push(Tokens::Boolean(false));
            return ParseResult::Ok;
        }
        ParseResult::None
    }

    fn field_name(&mut self) -> ParseResult {
        if self.end_of_query() {
            return ParseResult::None;
        }
        self.tokens
            .push(Tokens::FieldName(self.query[self.index].to_string()));
        self.index += 1;
        ParseResult::Ok
    }
}
