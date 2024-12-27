use ast::lexer::{Lexer, Token};

mod ast;

fn main() {
   let input = "5";
   let mut lexer = ast::lexer::Lexer::new(input);
   let mut tokens = Vec::new();
   while let Some(Token) = lexer.next_token() {
      tokens.push(Token);
   }
   println!("{:?}", tokens);
}
