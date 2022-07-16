mod token;

use super::ast::Expression;
use token::{Symbol, Token, TokenType};

pub enum ParseError {
    UnexpectedEof,
}

#[derive(Debug, Default)]
pub struct Parser {
    parsed: Vec<Token>,
    parsing: Box<[char]>,
    cursor: usize,
    peek: usize,
    row: isize,
    column: isize,
}

impl Parser {
    pub fn new() -> Self {
        Parser::default()
    }

    pub fn emit_expressions(&mut self) -> Result<Box<[Expression]>, ParseError> {
        // let expressions = Vec::new();
        unimplemented!()
    }

    pub fn parse(&mut self, input: &str) -> Result<(), ParseError> {
        self.parsing = Parser::chars(input);
        self.parse_to_end()?;
        Ok(())
    }

    fn parse_to_end(&mut self) -> Result<(), ParseError> {
        while let Some(char) = self.read().copied() {
            if self.parsed.capacity() == 0 {
                self.parsed.reserve(1 << 8);
            }
            let token = self.tokenize(&char)?;
            self.parsed.push(token);
            self.column += 1;
        }
        self.parsed.resize(
            self.parsed.len(),
            Token::new(TokenType::Unexpected(' '), 0, 0),
        );
        self.parsing = "".chars().collect::<Vec<char>>().into_boxed_slice();
        Ok(())
    }

    fn tokenize(&mut self, char: &char) -> Result<Token, ParseError> {
        match Symbol::try_from(char) {
            Ok(sym) => {
                let result = Ok(Token::new(TokenType::Symbol(sym), self.row, self.column));
                if sym == Symbol::NewLine {
                    self.column += -1;
                    self.row += 1;
                }
                result
            }
            Err(char) => {
                let mut ident = String::from(char);
                while let Err(char) =
                    Symbol::try_from(self.peek().ok_or(ParseError::UnexpectedEof)?)
                {
                    ident.push(char);
                }
                let token = Token::new(
                    TokenType::Ident(ident.into_boxed_str()),
                    self.row,
                    self.column,
                );
                self.column += self.peek as isize;
                self.advance_cursor_to_peek();
                Ok(token)
            }
        }
    }

    fn chars(input: &str) -> Box<[char]> {
        input.chars().collect()
    }

    fn read(&mut self) -> Option<&char> {
        let char = self.parsing.get(self.cursor);
        self.cursor += 1;
        char
    }

    fn peek(&mut self) -> Option<&char> {
        self.peek += 1;
        self.parsing.get(self.cursor + self.peek)
    }

    fn advance_cursor_to_peek(&mut self) {
        // We don't want to actually match peek, because recursive peeking stops
        // one AFTER the end of the token
        while (self.peek - self.cursor) > 1 {
            self.cursor += 1;
        }
        self.peek = 0;
    }
}
