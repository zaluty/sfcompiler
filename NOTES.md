# This will contain all the notes I have for the project

## What i have learned

i am new to rust and i am learning it by doing this project.

- What is Some, Self, Option and how to use them
- How to create a new struct
- How to create a new enum
- How to create a new impl block
- How to create a new function
- How to create a new method
- How to create a new trait
- How to create a new module
- How to create a new enum variant
 
## How does the lexer work 

From what i have understood, the lexer is a function that takes a string and returns a list of tokens.

first we created a struct that holds the input, the current position in the input, and the current character. 

Then we created a method that returns the current character.

```rust
impl Lexer {
    pub fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }
}
```
and then we created a function that returns the next token.

```rust
  pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos >= self.input.len() {
            if self.current_pos == self.input.len() {
                self.current_pos += 1;
                return Some(Token::new(
                    TokenKind::EOF,
                    TextSpan::new(self.current_pos - 1, self.current_pos, "\0".to_string())
                ));
            }
            return None;
        }

        let start: usize = self.current_pos;
        let c: char = self.current_char().unwrap();
        let mut kind: TokenKind = TokenKind::BadToken;
        
        if Self::is_number_start(&c) {
            let number: i64 = self.consume_number();
            kind = TokenKind::Number(number);
        }

        let end: usize = self.current_pos;
        let literal: String = self.input[start..end].to_string();
        let span: TextSpan = TextSpan::new(start, end, literal);
        Some(Token::new(kind, span))
    }
```
