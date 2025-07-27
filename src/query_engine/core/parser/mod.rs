use super::tokens;
mod tokens_parser;

pub fn parse(query: String) {
    let mut parser = tokens_parser::tokens_parser::new(query);
    let tokens = parser.parse();
    match tokens {
        Ok(vals) => {
          print!("[");
            for token in vals {
                print!("{token:?}, ");
            }
          print!("]");
        }
        Err(error) => println!("{error}"),
    }
}
